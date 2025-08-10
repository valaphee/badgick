use crate::{pfic::PficExt, sys::SysExt};
use core::{
    cell::{OnceCell, RefCell},
    sync::atomic::{AtomicU32, Ordering},
};
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use pac::{
    Pfic, Sys, Systick,
    interrupt::{CoreInterrupt, Priority},
};

pub struct SystickDriver {
    systick: OnceCell<Systick>,
    cnt_per_tick: AtomicU32,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

unsafe impl Sync for SystickDriver {}

embassy_time_driver::time_driver_impl!(static DRIVER: SystickDriver = SystickDriver {
    systick: OnceCell::new(),
    cnt_per_tick: AtomicU32::new(0),
    queue: Mutex::new(RefCell::new(Queue::new()))
});

impl SystickDriver {
    fn init(&'static self, systick: Systick, sys: &Sys) {
        self.systick.set(systick).unwrap();
        let systick = self.systick.get().unwrap();

        let cnt_per_second = sys.fsys() as u64;
        let cnt_per_tick = cnt_per_second / TICK_HZ;
        self.cnt_per_tick
            .store(cnt_per_tick as u32, Ordering::Relaxed);

        // Init and enable counter
        systick.ctl().write(|w| w.init().set_bit().ste().set_bit());

        // Reset compare value and clear interrupt flag
        systick.cmp().reset();
        systick.s().write(|w| w.cntif().clear_bit());

        // Count up without reloading and use HCLK/8
        systick
            .ctl()
            .modify(|_, w| w.mode().clear_bit().stre().clear_bit().stclk().clear_bit());
    }

    fn cnt(&self) -> u64 {
        self.systick.get().unwrap().cnt().read().bits()
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        // Clear interrupt flag
        self.systick
            .get()
            .unwrap()
            .s()
            .write(|w| w.cntif().clear_bit());

        let mut next = self
            .queue
            .borrow(cs)
            .borrow_mut()
            .next_expiration(self.cnt());
        while !self.set_alarm(cs, next) {
            next = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.cnt());
        }
    }

    fn set_alarm(&self, _cs: CriticalSection, next_alarm_cnt: u64) -> bool {
        let systick = self.systick.get().unwrap();

        // Already passed
        if next_alarm_cnt <= self.cnt() {
            return false;
        }

        // Set compare value
        systick
            .cmp()
            .write(|w| unsafe { w.cmp().bits(next_alarm_cnt) });

        // Enable interrupt
        systick.ctl().modify(|_, w| w.stie().set_bit());
        systick.s().write(|w| w.cntif().clear_bit());

        // Already passed, disable interrupt
        if next_alarm_cnt <= self.cnt() {
            systick.ctl().modify(|_, w| w.stie().clear_bit());
            systick.s().write(|w| w.cntif().clear_bit());
            return false;
        }

        true
    }
}

impl Driver for SystickDriver {
    fn now(&self) -> u64 {
        let cnt_per_tick = self.cnt_per_tick.load(Ordering::Relaxed) as u64;
        self.cnt() / cnt_per_tick
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        let cnt_per_tick = self.cnt_per_tick.load(Ordering::Relaxed) as u64;
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at * cnt_per_tick, waker) {
                let mut next = queue.next_expiration(self.cnt());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.cnt());
                }
            }
        })
    }
}

pub fn init(systick: Systick, sys: &Sys, pfic: &Pfic) {
    DRIVER.init(systick, sys);

    pfic.set_priority(CoreInterrupt::SysTick, Priority::P15);
    pfic.enable(CoreInterrupt::SysTick);
}

#[riscv_rt::core_interrupt(CoreInterrupt::SysTick)]
fn systick() {
    critical_section::with(|cs| DRIVER.trigger_alarm(cs));
}

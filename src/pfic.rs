use pac::{Pfic, interrupt::Priority};
use riscv::{InterruptNumber, PriorityNumber};

pub trait PficExt {
    fn enable(&self, interrupt: impl InterruptNumber);
    fn disable(&self, interrupt: impl InterruptNumber);
    fn is_enabled(&self, interrupt: impl InterruptNumber) -> bool;
    fn is_pending(&self, interrupt: impl InterruptNumber) -> bool;
    fn pend(&self, interrupt: impl InterruptNumber);
    fn unpend(&self, interrupt: impl InterruptNumber);
    fn is_active(&self, interrupt: impl InterruptNumber) -> bool;
    fn set_priority(&self, interrupt: impl InterruptNumber, priority: Priority);
    fn get_priority(&self, interrupt: impl InterruptNumber) -> Priority;
}

impl PficExt for Pfic {
    fn enable(&self, interrupt: impl InterruptNumber) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ienr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn disable(&self, interrupt: impl InterruptNumber) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.irer1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_enabled(&self, interrupt: impl InterruptNumber) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.isr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn is_pending(&self, interrupt: impl InterruptNumber) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ipr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn pend(&self, interrupt: impl InterruptNumber) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ipsr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn unpend(&self, interrupt: impl InterruptNumber) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.iprr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_active(&self, interrupt: impl InterruptNumber) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.iactr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn set_priority(&self, interrupt: impl InterruptNumber, priority: Priority) {
        let off = interrupt.number();
        unsafe {
            self.iprior0()
                .as_ptr()
                .add(off)
                .write_volatile(priority.number() as _)
        }
    }

    fn get_priority(&self, interrupt: impl InterruptNumber) -> Priority {
        let off = interrupt.number() / 32;
        Priority::from_number(unsafe { self.iprior0().as_ptr().add(off).read_volatile() } as _)
            .unwrap()
    }
}

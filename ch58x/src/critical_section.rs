use critical_section::{set_impl, Impl, RawRestoreState};

struct SingleHartCriticalSection;
set_impl!(SingleHartCriticalSection);

unsafe impl Impl for SingleHartCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let value: usize;
        unsafe { core::arch::asm!("csrrc {}, 0x800, {}", out(reg) value, in(reg) 0x8) };
        value & 0x8 != 0
    }

    unsafe fn release(restore_state: RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if restore_state {
            unsafe { core::arch::asm!("csrs 0x800, {}", in(reg) 0x8) };
        }
    }
}

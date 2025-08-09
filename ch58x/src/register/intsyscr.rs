use riscv::{read_write_csr, read_write_csr_field};

read_write_csr! {
    /// Interrupt system control register
    Intsyscr: 0x804,
    mask: 0x3,
}

read_write_csr_field! {
    Intsyscr,
    /// Hardware pushstack function enable
    hwstken: 0
}

read_write_csr_field! {
    Intsyscr,
    /// Interrupt nesting enable
    inesten: 1
}

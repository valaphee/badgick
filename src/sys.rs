use fugit::HertzU32 as Hertz;

/// Clock frequencies
pub struct Clocks {
    // System frequency
    pub fsys: Hertz,
}

impl Default for Clocks {
    fn default() -> Clocks {
        Clocks {
            fsys: Hertz::from_raw(6_400_000),
        }
    }
}

/// Constrained `SYS` peripheral
pub struct Sys {
    /// Clock configuration
    pub(crate) sys: pac::SYS,
    pub clocks: Clocks,
}

impl Sys {
    /// Apply clock configuration
    pub fn freeze(self) -> Self {
        self
    }
}

/// Extension trait that constrains the `SYS` peripheral
pub trait SysExt {
    /// Constrains the `SYS` peripheral so it plays nicely with the other
    /// abstractions
    fn constrain(self) -> Sys;
    /// Constrains the `SYS` peripheral and apply clock configuration
    fn freeze(self) -> Sys;
}

impl SysExt for pac::SYS {
    fn constrain(self) -> Sys {
        Sys {
            sys: self,
            clocks: Clocks::default(),
        }
    }

    fn freeze(self) -> Sys {
        self.constrain().freeze()
    }
}

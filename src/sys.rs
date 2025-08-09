use fugit::HertzU32 as Hertz;

pub struct Clocks {
    pub fsys: Hertz,
}

impl Default for Clocks {
    fn default() -> Clocks {
        Clocks {
            fsys: Hertz::from_raw(6_400_000),
        }
    }
}

pub struct Sys {
    sys: pac::SYS,
    pub clocks: Clocks,
}

impl Sys {
    pub fn freeze(self) -> Self {
        self
    }
}

pub trait SysExt {
    fn constrain(self) -> Sys;
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

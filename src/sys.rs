use riscv::asm::{delay, nop};

pub trait SysExt {
    fn set(self, config: Config) -> Self;
    fn fsys(&self) -> u32;
}

impl SysExt for pac::Sys {
    fn set(self, config: Config) -> Self {
        match config.clock32ksrc {
            Clock32KSrc::LSE => {
                with_safe_mode(|| {
                    self.ck32k_config()
                        .modify(|_, w| w.clk_xt32k_pon().set_bit());
                });
                delay(self.fsys() / 10 / 4);
                with_safe_mode(|| {
                    self.ck32k_config()
                        .modify(|_, w| w.clk_osc32k_xt().set_bit());
                });
                delay(self.fsys() / 1000);
            }
            Clock32KSrc::LSI => {
                with_safe_mode(|| {
                    self.ck32k_config()
                        .modify(|_, w| w.clk_osc32k_xt().clear_bit().clk_int32k_pon().set_bit());
                });
            }
        }

        with_safe_mode(|| {
            self.pll_config()
                .modify(|r, w| unsafe { w.pll_cfg_dat().bits(r.pll_cfg_dat().bits() & !(1 << 5)) });
        });
        match config.clocksyssrc {
            ClockSysSrc::Clock32K => {
                with_safe_mode(|| {
                    self.clk_sys_cfg()
                        .modify(|_, w| unsafe { w.clk_sys_mod().bits(0b11) });
                });
            }
            ClockSysSrc::HSE(div) => {
                if self.hfck_pwr_ctrl().read().clk_xt32m_pon().bit_is_clear() {
                    with_safe_mode(|| {
                        self.hfck_pwr_ctrl()
                            .modify(|_, w| w.clk_xt32m_pon().set_bit());
                    });
                    delay(2400);
                }

                with_safe_mode(|| {
                    self.clk_sys_cfg().write(|w| unsafe {
                        w.clk_sys_mod().bits(0b00).clk_pll_div().bits(div & 0x1F)
                    });
                    nop();
                    nop();
                    nop();
                    nop();
                });

                with_safe_mode(|| {
                    nop();
                    nop();
                    self.flash_cfg().write(|w| unsafe { w.bits(0x51) });
                });
            }
            ClockSysSrc::PLL(div) => {
                if self.hfck_pwr_ctrl().read().clk_pll_pon().bit_is_clear() {
                    with_safe_mode(|| {
                        self.hfck_pwr_ctrl()
                            .modify(|_, w| w.clk_pll_pon().set_bit());
                    });
                    delay(4000);
                }

                with_safe_mode(|| {
                    self.clk_sys_cfg().write(|w| unsafe {
                        w.clk_sys_mod().bits(0b01).clk_pll_div().bits(div & 0x1F)
                    });
                    nop();
                    nop();
                    nop();
                    nop();
                });

                if div == 6 {
                    with_safe_mode(|| {
                        self.flash_cfg().write(|w| unsafe { w.bits(0x02) });
                    });
                } else {
                    with_safe_mode(|| {
                        self.flash_cfg().write(|w| unsafe { w.bits(0x52) });
                    });
                }
            }
        }
        with_safe_mode(|| {
            self.pll_config().modify(|_, w| w.flash_io_mod().set_bit());
        });

        self
    }

    fn fsys(&self) -> u32 {
        let clk_sys_cfg = self.clk_sys_cfg().read();
        match clk_sys_cfg.clk_sys_mod().bits() {
            0b00 => 32_000_000 / clk_sys_cfg.clk_pll_div().bits() as u32,
            0b01 => 480_000_000 / clk_sys_cfg.clk_pll_div().bits() as u32,
            0b10 => 32_000_000,
            _ => 32_000,
        }
    }
}

enum Clock32KSrc {
    LSE,
    LSI,
}

enum ClockSysSrc {
    Clock32K,
    HSE(u8),
    PLL(u8),
}

pub struct Config {
    clock32ksrc: Clock32KSrc,
    clocksyssrc: ClockSysSrc,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock32ksrc: Clock32KSrc::LSI,
            clocksyssrc: ClockSysSrc::HSE(5),
        }
    }
}

impl Config {
    pub fn clock32k() -> Self {
        Self {
            clock32ksrc: Clock32KSrc::LSI,
            clocksyssrc: ClockSysSrc::Clock32K,
        }
    }

    pub fn hse(div: u8) -> Self {
        Self {
            clock32ksrc: Clock32KSrc::LSI,
            clocksyssrc: ClockSysSrc::HSE(div),
        }
    }

    pub fn pll(div: u8) -> Self {
        Self {
            clock32ksrc: Clock32KSrc::LSI,
            clocksyssrc: ClockSysSrc::PLL(div),
        }
    }
}

pub fn with_safe_mode<R>(f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| {
        unsafe {
            pac::Sys::steal().safe_access_sig().write(|w| w.bits(0x57));
            pac::Sys::steal().safe_access_sig().write(|w| w.bits(0xA8));
            nop();
            nop();
        }

        let value = f();

        unsafe {
            pac::Sys::steal().safe_access_sig().write(|w| w.bits(0x00));
            nop();
            nop();
        }

        value
    })
}

use crate::{bus::Bus, cpu::Cpu};
use anyhow::Result;

// Timer 0
pub const TCR0_8: u32 = 0xffff80;
pub const TCSR0_8: u32 = 0xffff82;
pub const TCORA0: u32 = 0xffff84;
pub const TCORB0: u32 = 0xffff86;
pub const TCNT0_8: u32 = 0xffff88;

// Timer 1
pub const TCR1_8: u32 = 0xffff81;
pub const TCSR1_8: u32 = 0xffff83;
pub const TCORA1: u32 = 0xffff85;
pub const TCORB1: u32 = 0xffff87;
pub const TCNT1_8: u32 = 0xffff89;

// Timer 2
pub const TCR2_8: u32 = 0xffff90;
pub const TCSR2_8: u32 = 0xffff92;
pub const TCORA2: u32 = 0xffff94;
pub const TCORB2: u32 = 0xffff96;
pub const TCNT2_8: u32 = 0xffff98;

// Timer 3
pub const TCR3_8: u32 = 0xffff91;
pub const TCSR3_8: u32 = 0xffff93;
pub const TCORA3: u32 = 0xffff95;
pub const TCORB3: u32 = 0xffff97;
pub const TCNT3_8: u32 = 0xffff99;

enum CounterClear {
    Forbidden,
    CompareA,
    CompareInputB,
    InputB,
}

pub struct Timer8_0 {
    is_active: bool,
    state: u16,
    // Interrupt
    is_allowed_cmib: bool,
    is_allowed_cmia: bool,
    is_allowed_ovi: bool,

    is_cleared_by: CounterClear,
    prescaler: u16,
}

impl Timer8_0 {
    pub fn new() -> Self {
        Timer8_0 {
            is_active: false,
            state: 0,
            is_allowed_cmib: false,
            is_allowed_cmia: false,
            is_allowed_ovi: false,
            is_cleared_by: CounterClear::Forbidden, // TODO: 8TCNT1 および 8TCNT3 のカウンタクリア要因を、インプットキャプチャ B に設定した場合、8TCNT0および 8TCNT2 はコンぺアマッチ B によりクリアされません。
            prescaler: 0,
        }
    }

    pub fn update_timer8_0(&mut self, bus: &mut Bus, state: u8) -> Result<()> {
        if self.prescaler == 0 {
            return Ok(());
        }
        self.state += u16::from(state);
        let mut count = self.state / self.prescaler;
        self.state -= self.prescaler * count;

        while count != 0 {
            // count
            let tcnt = bus.read(TCNT0_8)?;
            let (mut tcnt, is_overflowed) = tcnt.overflowing_add(1);

            let tcora = bus.read(TCORA0)?;
            let tcorb = bus.read(TCORB0)?;

            let mut tcsr = bus.read(TCSR0_8)?;

            // CMFA, Compare match on TCORA0
            if tcnt == tcora {
                tcsr |= 0b0100_0000;
                if let CounterClear::CompareA = self.is_cleared_by {
                    tcnt = 0;
                }
                if self.is_allowed_cmia {
                    // cpu.request_interrupt(36);
                }
            }

            // CMFB, Compare match on TCORB0
            if tcnt == tcorb {
                tcsr |= 0b1000_0000;
                if let CounterClear::CompareInputB = self.is_cleared_by {
                    tcnt = 0;
                }
                if self.is_allowed_cmib {
                    // cpu.request_interrupt(37);
                }
            }

            // OVF, Overflow TCNT
            if is_overflowed {
                tcsr |= 0b0010_0000;
                if self.is_allowed_ovi {
                    // cpu.request_interrupt(39);
                }
            }

            bus.write(TCNT0_8, tcnt)?;
            bus.write(TCSR0_8, tcsr)?;

            count -= 1;
        }

        Ok(())
    }

    pub fn update_tcr(&mut self, tcr: u8) {
        self.is_allowed_cmib = tcr & 0b1000_0000 != 0;
        self.is_allowed_cmia = tcr & 0b0100_0000 != 0;
        self.is_allowed_ovi = tcr & 0b0010_0000 != 0;

        match tcr & 0b0001_1000 {
            0b0000_0000 => self.is_cleared_by = CounterClear::Forbidden,
            0b0000_1000 => self.is_cleared_by = CounterClear::CompareA,
            0b0001_0000 => self.is_cleared_by = CounterClear::CompareInputB,
            0b0001_1000 => self.is_cleared_by = CounterClear::InputB,
            _ => (),
        }

        // 外部クロックは未実装
        match tcr & 0b0000_0111 {
            0b0000_0000 => self.prescaler = 0,
            0b0000_0001 => self.prescaler = 8,
            0b0000_0010 => self.prescaler = 64,
            0b0000_0011 => self.prescaler = 8192,
            0b0000_0100 => (), // TODO: 16ビットモード
            _ => (),
        }

        self.is_active = self.prescaler != 0;
    }
}

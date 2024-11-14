use anyhow::Result;
use timer8::{Timer8_0, TCR0_8};

use crate::{bus::Bus, cpu::Cpu};

mod timer8;

struct Modules {
    timer8_0: Timer8_0,
}

impl Modules {
    pub fn new() -> Self {
        Modules { timer8_0: Timer8_0::new() }
    }
}

pub struct ModuleManager {
    modules: Modules,
}

impl ModuleManager {
    pub fn new() -> Self {
        ModuleManager { modules: Modules::new() }
    }

    pub fn write_registers(&mut self, addr: u32, value: u8) {
        match addr {
            TCR0_8 => self.modules.timer8_0.update_tcr(value),
            _ => (),
        }
    }

    pub fn update_modules(&mut self, bus: &mut Bus, state: u8) -> Result<()> {
        self.modules.timer8_0.update_timer8_0(bus, state)?;

        Ok(())
    }

    pub fn test(&mut self, bus: &Bus) {}
}

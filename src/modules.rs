use timer8::{Timer8_0, TCR0_8};

mod timer8;

struct Modules {
    timer8_0: Timer8_0,
}

impl Modules {
    pub fn new() -> Self {
        Modules { timer8_0: Timer8_0::new() }
    }
}

struct ModuleManager {
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
}

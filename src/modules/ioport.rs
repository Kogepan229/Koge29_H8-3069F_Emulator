use crate::bus::{Bus, IO_REGISTERS1_START_ADDR, IO_REGISTERS2_EMC1_START_ADDR};
use anyhow::Result;

pub const IO_PORT_1_DDR_ADDR: u32 = 0xfee000;
pub const IO_PORT_1_DR_ADDR: u32 = 0xffffd0;

impl Bus {
    /// port: 1..=0xb
    fn read_ddr(&self, port: u8) -> u8 {
        self.io_registrs1[IO_PORT_1_DDR_ADDR as usize + port as usize - 1 - IO_REGISTERS1_START_ADDR as usize]
    }

    /// port: 1..=0xb
    fn write_dr(&mut self, port: u8, value: u8) {
        let index = IO_PORT_1_DR_ADDR as usize + port as usize - 1 - IO_REGISTERS2_EMC1_START_ADDR as usize;
        self.io_registrs2[index] = value;
    }

    /// port: 1..=0xb
    fn read_dr(&self, port: u8) -> u8 {
        let index = IO_PORT_1_DR_ADDR as usize + port as usize - 1 - IO_REGISTERS2_EMC1_START_ADDR as usize;
        self.io_registrs2[index]
    }

    /// port: 1..=0xb
    pub fn write_port(&mut self, port: u8, value: u8) {
        if port >= 1 && port <= 0xb {
            self.io_port_in[port as usize - 1] = value;
            let ddr = self.read_ddr(port);
            let dr = (self.read_dr(port) & ddr) | (!ddr & value);
            self.write_dr(port, dr);
        }
    }

    pub fn on_write_ddr(&mut self, addr: u32, ddr: u8) -> Result<()> {
        self.io_registrs1[(addr - IO_REGISTERS1_START_ADDR) as usize] = ddr;
        let port = (addr - IO_PORT_1_DDR_ADDR) as u8 + 1;
        // Input
        let dr = (self.read_dr(port) & ddr) | (!ddr & self.io_port_in[port as usize - 1]);
        self.write_dr(port, dr);

        // Output
        let io_port_out = self.read_dr(port) & ddr;
        self.send_io_port_value(port, io_port_out)?;
        Ok(())
    }

    pub fn on_write_dr(&mut self, addr: u32, dr: u8) -> Result<()> {
        let port = (addr - IO_PORT_1_DR_ADDR) as u8 + 1;
        let ddr = self.read_ddr(port);
        // Input
        let real_dr = (dr & ddr) | (!ddr & self.io_port_in[port as usize - 1]);
        self.write_dr(port, real_dr);

        // Output
        let io_port_out = dr & ddr;
        self.send_io_port_value(port, io_port_out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{IO_PORT_1_DDR_ADDR, IO_PORT_1_DR_ADDR};
    use crate::{
        bus::{Bus, IO_REGISTERS1_START_ADDR, IO_REGISTERS2_EMC1_START_ADDR},
        modules::ModuleManager,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_write_port() {
        let module_manager = Rc::new(RefCell::new(ModuleManager::new()));

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[0] = 0;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize] = 0xf0;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0;
        bus.write_port(1, 0xff);
        assert_eq!(bus.io_port_in[0], 0xff);
        assert_eq!(bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize], 0x0f);

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[0] = 0xff;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize] = 0xf0;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0xff;
        bus.write_port(1, 0);
        assert_eq!(bus.io_port_in[0], 0);
        assert_eq!(bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize], 0xf0);
    }

    #[test]
    fn test_on_write_ddr() {
        let module_manager = Rc::new(RefCell::new(ModuleManager::new()));

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[0] = 0xff;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize] = 0xff;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0;
        bus.on_write_ddr(IO_PORT_1_DDR_ADDR, 0xf0).unwrap();
        assert_eq!(bus.io_port_in[0], 0xff);
        assert_eq!(bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize], 0x0f);
        assert_eq!(bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize], 0xf0);

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[4] = 0xff;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR + 4 - IO_REGISTERS1_START_ADDR) as usize] = 0xff;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR + 4 - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0;
        bus.on_write_ddr(IO_PORT_1_DDR_ADDR + 4, 0xf0).unwrap();
        assert_eq!(bus.io_port_in[4], 0xff);
        assert_eq!(
            bus.io_registrs2[(IO_PORT_1_DR_ADDR + 4 - IO_REGISTERS2_EMC1_START_ADDR) as usize],
            0x0f
        );
        assert_eq!(bus.io_registrs1[(IO_PORT_1_DDR_ADDR + 4 - IO_REGISTERS1_START_ADDR) as usize], 0xf0);
    }

    #[test]
    fn test_on_write_dr() {
        let module_manager = Rc::new(RefCell::new(ModuleManager::new()));

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[0] = 0;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize] = 0x0f;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0;
        bus.on_write_dr(IO_PORT_1_DR_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_port_in[0], 0);
        assert_eq!(bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize], 0x0f);

        let mut bus = Bus::new(Rc::downgrade(&module_manager));
        bus.io_port_in[0] = 0xf0;
        bus.io_registrs1[(IO_PORT_1_DDR_ADDR - IO_REGISTERS1_START_ADDR) as usize] = 0x0f;
        bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize] = 0;
        bus.on_write_dr(IO_PORT_1_DR_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_port_in[0], 0xf0);
        assert_eq!(bus.io_registrs2[(IO_PORT_1_DR_ADDR - IO_REGISTERS2_EMC1_START_ADDR) as usize], 0xff);
    }
}

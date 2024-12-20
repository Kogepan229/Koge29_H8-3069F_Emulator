use crate::bus::{Bus, IO_REGISTERS1_START_ADDR, IO_REGISTERS2_EMC1_START_ADDR};

pub const IO_PORT_1_DDR_ADDR: u32 = 0xfee000;
pub const IO_PORT_1_DR_ADDR: u32 = 0xffffd0;

impl Bus {
    // 1..=0xb
    fn read_ddr(&self, port: u8) -> u8 {
        self.io_registrs1[IO_PORT_1_DDR_ADDR as usize + port as usize - 1 - IO_REGISTERS1_START_ADDR as usize]
    }

    fn write_dr(&mut self, port: u8, value: u8) {
        let index = IO_PORT_1_DR_ADDR as usize + port as usize - 1 - IO_REGISTERS2_EMC1_START_ADDR as usize;
        self.io_registrs2[index] = value;
    }

    fn read_dr(&self, port: u8) -> u8 {
        let index = IO_PORT_1_DR_ADDR as usize + port as usize - 1 - IO_REGISTERS2_EMC1_START_ADDR as usize;
        self.io_registrs2[index]
    }

    pub fn write_port(&mut self, port_str: String, value: u8) {
        if let Ok(port) = u8::from_str_radix(&port_str, 16) {
            if port >= 1 && port <= 0xb {
                self.io_port_in[port as usize - 1] = value;
                let ddr = self.read_ddr(port);
                let dr = (self.read_dr(port) & ddr) | (!ddr & value);
                self.write_dr(port, dr);
            }
        }
    }

    pub fn on_write_ddr(&mut self, addr: u32, ddr: u8) {
        let port = (addr - IO_PORT_1_DDR_ADDR) as u8 + 1;
        // Input
        let dr = (self.read_dr(port) & ddr) | (!ddr & self.io_port_in[port as usize - 1]);
        self.write_dr(port, dr);

        // Output
        let io_port_out = self.read_dr(port) & ddr;
        // TODO: send
    }

    pub fn on_write_dr(&mut self, addr: u32, dr: u8) {
        let port = (addr - IO_PORT_1_DR_ADDR) as u8 + 1;
        let ddr = self.read_ddr(port);
        // Input
        let real_dr = (dr & ddr) | (!ddr & self.io_port_in[port as usize - 1]);
        self.write_dr(port, real_dr);

        // Output
        let io_port_out = self.read_dr(dr) & ddr;
        // TODO: send
    }
}

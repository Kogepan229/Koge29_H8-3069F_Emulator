use crate::memory::{create_memory, Memory, MEMORY_END_ADDR, MEMORY_START_ADDR};
use anyhow::{bail, Result};

const EXCEPTION_HANDLING_VENCTOR_SIZE: usize = 0xff;

pub const IO_REGISTERS1_START_ADDR: u32 = 0xfee000;
pub const IO_REGISTERS1_END_ADDR: u32 = 0xfee0bf;
pub const IO_REGISTERS1_SIZE: usize =
    (IO_REGISTERS1_END_ADDR - IO_REGISTERS1_START_ADDR + 1) as usize;

pub const IO_REGISTERS2_EMC1_START_ADDR: u32 = 0xffff20;
pub const IO_REGISTERS2_EMC1_END_ADDR: u32 = 0xffffe9;
pub const IO_REGISTERS2_EMC1_SIZE: usize =
    (IO_REGISTERS2_EMC1_END_ADDR - IO_REGISTERS2_EMC1_START_ADDR + 1) as usize;

// pub const IO_REGISTERIES2_EMC0_STRAT_ADDR: u32 = 0xfffe80;
// pub const IO_REGISTERIES2_EMC0_END_ADDR: u32 = 0xffffff;
// pub const IO_REGISTERIES2_EMC0_SIZE: usize = (MEMORY_END_ADDR - MEMORY_START_ADDR + 1) as usize;

pub struct Bus {
    pub memory: Memory,
    pub exception_handling_vector: [u8; EXCEPTION_HANDLING_VENCTOR_SIZE],
    io_registrs1: [u8; IO_REGISTERS1_SIZE],
    io_registrs2: [u8; IO_REGISTERS2_EMC1_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: create_memory(),
            exception_handling_vector: [0; EXCEPTION_HANDLING_VENCTOR_SIZE],
            io_registrs1: [0; IO_REGISTERS1_SIZE],
            io_registrs2: [0; IO_REGISTERS2_EMC1_SIZE],
        }
    }

    pub fn write(&mut self, addr: u32, value: u8) -> Result<()> {
        match addr {
            0x00..=0xff => self.exception_handling_vector[addr as usize] = value,
            IO_REGISTERS1_START_ADDR..=IO_REGISTERS1_END_ADDR => {
                self.io_registrs1[(addr - IO_REGISTERS1_START_ADDR) as usize] = value
            }
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                self.memory[(addr - MEMORY_START_ADDR) as usize] = value
            }
            IO_REGISTERS2_EMC1_START_ADDR..=IO_REGISTERS2_EMC1_END_ADDR => {
                self.io_registrs2[(addr - IO_REGISTERS2_EMC1_START_ADDR) as usize] = value
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
        Ok(())
    }

    pub fn read(&self, addr: u32) -> Result<u8> {
        match addr {
            0x00..=0xff => return Ok(self.exception_handling_vector[addr as usize]),
            IO_REGISTERS1_START_ADDR..=IO_REGISTERS1_END_ADDR => {
                println!("{:x}", addr);
                return Ok(self.io_registrs1[(addr - IO_REGISTERS1_START_ADDR) as usize]);
            }
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                return Ok(self.memory[(addr - MEMORY_START_ADDR) as usize])
            }
            IO_REGISTERS2_EMC1_START_ADDR..=IO_REGISTERS2_EMC1_END_ADDR => {
                return Ok(self.io_registrs2[(addr - IO_REGISTERS2_EMC1_START_ADDR) as usize])
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bus::{
            IO_REGISTERS1_END_ADDR, IO_REGISTERS1_SIZE, IO_REGISTERS1_START_ADDR,
            IO_REGISTERS2_EMC1_END_ADDR, IO_REGISTERS2_EMC1_SIZE, IO_REGISTERS2_EMC1_START_ADDR,
        },
        memory::{MEMORY_END_ADDR, MEMORY_SIZE, MEMORY_START_ADDR},
    };

    use super::Bus;

    #[test]
    fn test_write_memory() {
        let mut bus = Bus::new();
        bus.memory[0] = 0xff;
        bus.write(MEMORY_START_ADDR, 0xff).unwrap();
        assert_eq!(bus.memory[0], 0xff);

        bus.memory[MEMORY_SIZE - 1] = 0xff;
        bus.write(MEMORY_END_ADDR, 0xff).unwrap();
        assert_eq!(bus.memory[MEMORY_SIZE - 1], 0xff)
    }

    #[test]
    fn test_read_memory() {
        let mut bus = Bus::new();
        bus.memory[0] = 0xff;
        assert_eq!(bus.read(MEMORY_START_ADDR).unwrap(), 0xff);

        bus.memory[MEMORY_SIZE - 1] = 0xff;
        assert_eq!(bus.read(MEMORY_END_ADDR).unwrap(), 0xff)
    }

    #[test]
    fn test_write_io_registers() {
        let mut bus = Bus::new();
        bus.io_registrs1[0] = 0xff;
        bus.write(IO_REGISTERS1_START_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_registrs1[0], 0xff);

        let mut bus = Bus::new();
        bus.io_registrs1[IO_REGISTERS1_SIZE - 1] = 0xff;
        bus.write(IO_REGISTERS1_END_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_registrs1[IO_REGISTERS1_SIZE - 1], 0xff);

        let mut bus = Bus::new();
        bus.io_registrs2[0] = 0xff;
        bus.write(IO_REGISTERS2_EMC1_START_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_registrs2[0], 0xff);

        let mut bus = Bus::new();
        bus.io_registrs2[IO_REGISTERS2_EMC1_SIZE - 1] = 0xff;
        bus.write(IO_REGISTERS2_EMC1_END_ADDR, 0xff).unwrap();
        assert_eq!(bus.io_registrs2[IO_REGISTERS2_EMC1_SIZE - 1], 0xff)
    }

    #[test]
    fn test_read_io_registers() {
        let mut bus = Bus::new();
        bus.io_registrs1[0] = 0xff;
        assert_eq!(bus.read(IO_REGISTERS1_START_ADDR).unwrap(), 0xff);

        let mut bus = Bus::new();
        bus.io_registrs1[IO_REGISTERS1_SIZE - 1] = 0xff;
        assert_eq!(bus.read(IO_REGISTERS1_END_ADDR).unwrap(), 0xff);

        let mut bus = Bus::new();
        bus.io_registrs2[0] = 0xff;
        assert_eq!(bus.read(IO_REGISTERS2_EMC1_START_ADDR).unwrap(), 0xff);

        let mut bus = Bus::new();
        bus.io_registrs2[IO_REGISTERS2_EMC1_SIZE - 1] = 0xff;
        assert_eq!(bus.read(IO_REGISTERS2_EMC1_END_ADDR).unwrap(), 0xff)
    }
}

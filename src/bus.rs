use crate::{
    memory::{create_memory, Memory, MEMORY_END_ADDR, MEMORY_START_ADDR},
    registers::DRCRA,
};
use anyhow::{bail, Result};

pub const VENCTOR_START_ADDR: u32 = 0;
pub const VENCTOR_END_ADDR: u32 = 0xff;
pub const VENCTOR_SIZE: usize = (VENCTOR_END_ADDR - VENCTOR_START_ADDR + 1) as usize;

pub const AREA0_START_ADDR: u32 = 0;
pub const AREA0_END_ADDR: u32 = 0x1fffff;
pub const AREA0_SIZE: usize = (AREA0_END_ADDR - AREA0_START_ADDR + 1) as usize;

pub const AREA1_START_ADDR: u32 = 0x200000;
pub const AREA1_END_ADDR: u32 = 0x3fffff;
pub const AREA1_SIZE: usize = (AREA1_END_ADDR - AREA1_START_ADDR + 1) as usize;

// DRAM
pub const AREA2_START_ADDR: u32 = 0x400000;
pub const AREA2_END_ADDR: u32 = 0x5fffff;
pub const AREA2_SIZE: usize = (AREA2_END_ADDR - AREA2_START_ADDR + 1) as usize;

pub const AREA3_START_ADDR: u32 = 0x600000;
pub const AREA3_END_ADDR: u32 = 0x7fffff;
pub const AREA3_SIZE: usize = (AREA3_END_ADDR - AREA3_START_ADDR + 1) as usize;

pub const AREA4_START_ADDR: u32 = 0x800000;
pub const AREA4_END_ADDR: u32 = 0x9fffff;
pub const AREA4_SIZE: usize = (AREA4_END_ADDR - AREA4_START_ADDR + 1) as usize;

pub const AREA5_START_ADDR: u32 = 0xa00000;
pub const AREA5_END_ADDR: u32 = 0xbfffff;
pub const AREA5_SIZE: usize = (AREA5_END_ADDR - AREA5_START_ADDR + 1) as usize;

pub const AREA6_START_ADDR: u32 = 0xc00000;
pub const AREA6_END_ADDR: u32 = 0xdfffff;
pub const AREA6_SIZE: usize = (AREA6_END_ADDR - AREA6_START_ADDR + 1) as usize;

pub const AREA7_START_ADDR: u32 = 0xe00000;
pub const AREA7_END_ADDR: u32 = 0xffffff;
pub const AREA7_SIZE: usize = (AREA7_END_ADDR - AREA7_START_ADDR + 1) as usize;

pub const IO_REGISTERS1_START_ADDR: u32 = 0xfee000;
pub const IO_REGISTERS1_END_ADDR: u32 = 0xfee0ff;
pub const IO_REGISTERS1_SIZE: usize = (IO_REGISTERS1_END_ADDR - IO_REGISTERS1_START_ADDR + 1) as usize;

pub const IO_REGISTERS2_EMC1_START_ADDR: u32 = 0xffff20;
pub const IO_REGISTERS2_EMC1_END_ADDR: u32 = 0xffffe9;
pub const IO_REGISTERS2_EMC1_SIZE: usize = (IO_REGISTERS2_EMC1_END_ADDR - IO_REGISTERS2_EMC1_START_ADDR + 1) as usize;

// pub const IO_REGISTERIES2_EMC0_STRAT_ADDR: u32 = 0xfffe80;
// pub const IO_REGISTERIES2_EMC0_END_ADDR: u32 = 0xffffff;
// pub const IO_REGISTERIES2_EMC0_SIZE: usize = (MEMORY_END_ADDR - MEMORY_START_ADDR + 1) as usize;

#[derive(Clone)]
pub struct Bus {
    pub memory: Memory,
    pub exception_handling_vector: Box<[u8]>,
    pub dram: Box<[u8]>,
    io_registrs1: Box<[u8]>,
    io_registrs2: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: create_memory(),
            exception_handling_vector: vec![0; VENCTOR_SIZE].into_boxed_slice(),
            dram: vec![0; AREA2_SIZE].into_boxed_slice(),
            io_registrs1: vec![0; IO_REGISTERS1_SIZE].into_boxed_slice(),
            io_registrs2: vec![0; IO_REGISTERS2_EMC1_SIZE].into_boxed_slice(),
        }
    }

    pub fn write(&mut self, addr: u32, value: u8) -> Result<()> {
        match addr {
            VENCTOR_START_ADDR..=VENCTOR_END_ADDR => self.exception_handling_vector[addr as usize] = value,
            IO_REGISTERS1_START_ADDR..=IO_REGISTERS1_END_ADDR => self.io_registrs1[(addr - IO_REGISTERS1_START_ADDR) as usize] = value,
            AREA2_START_ADDR..=AREA2_END_ADDR => self.dram[(addr - AREA2_START_ADDR) as usize] = value,
            MEMORY_START_ADDR..=MEMORY_END_ADDR => self.memory[(addr - MEMORY_START_ADDR) as usize] = value,
            IO_REGISTERS2_EMC1_START_ADDR..=IO_REGISTERS2_EMC1_END_ADDR => {
                self.io_registrs2[(addr - IO_REGISTERS2_EMC1_START_ADDR) as usize] = value
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
        Ok(())
    }

    pub fn read(&self, addr: u32) -> Result<u8> {
        match addr {
            VENCTOR_START_ADDR..=VENCTOR_END_ADDR => return Ok(self.exception_handling_vector[addr as usize]),
            IO_REGISTERS1_START_ADDR..=IO_REGISTERS1_END_ADDR => {
                return Ok(self.io_registrs1[(addr - IO_REGISTERS1_START_ADDR) as usize]);
            }
            AREA2_START_ADDR..=AREA2_END_ADDR => return Ok(self.dram[(addr - AREA2_START_ADDR) as usize]),
            MEMORY_START_ADDR..=MEMORY_END_ADDR => return Ok(self.memory[(addr - MEMORY_START_ADDR) as usize]),
            IO_REGISTERS2_EMC1_START_ADDR..=IO_REGISTERS2_EMC1_END_ADDR => {
                return Ok(self.io_registrs2[(addr - IO_REGISTERS2_EMC1_START_ADDR) as usize])
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
    }

    pub fn get_area_index(target_addr: u32) -> Result<u8> {
        match target_addr {
            AREA0_START_ADDR..=AREA0_END_ADDR => {
                return Ok(0);
            }
            AREA1_START_ADDR..=AREA1_END_ADDR => {
                return Ok(1);
            }
            AREA2_START_ADDR..=AREA2_END_ADDR => {
                return Ok(2);
            }
            AREA3_START_ADDR..=AREA3_END_ADDR => {
                return Ok(3);
            }
            AREA4_START_ADDR..=AREA4_END_ADDR => {
                return Ok(4);
            }
            AREA5_START_ADDR..=AREA5_END_ADDR => {
                return Ok(5);
            }
            AREA6_START_ADDR..=AREA6_END_ADDR => {
                return Ok(6);
            }
            AREA7_START_ADDR..=AREA7_END_ADDR => {
                return Ok(7);
            }
            _ => bail!("Invalid Addr [{}].", target_addr),
        }
    }

    pub fn check_dram_area(&self, area_index: u8) -> Result<bool> {
        let register = self.read(DRCRA)? >> 5;

        match area_index {
            2 => {
                if register >= 1 {
                    return Ok(true);
                }
            }
            3 => {
                if register >= 2 {
                    return Ok(true);
                }
            }
            4 => {
                if register >= 4 {
                    return Ok(true);
                }
            }
            5 => {
                if register >= 5 {
                    return Ok(true);
                }
            }
            _ => return Ok(false),
        }
        return Ok(false);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bus::{
            IO_REGISTERS1_END_ADDR, IO_REGISTERS1_SIZE, IO_REGISTERS1_START_ADDR, IO_REGISTERS2_EMC1_END_ADDR, IO_REGISTERS2_EMC1_SIZE,
            IO_REGISTERS2_EMC1_START_ADDR,
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

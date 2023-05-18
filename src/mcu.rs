use crate::memory::{create_memory, Memory, MEMORY_END_ADDR, MEMORY_START_ADDR};
use anyhow::{bail, Result};

const EXCEPTION_HANDLING_VENCTOR_SIZE: usize = 0xff;

// cpuは別 本来のmcuとは意味が異なるのであとで変えるかも
pub struct Mcu {
    pub memory: Memory,
    pub exception_handling_vector: [u8; EXCEPTION_HANDLING_VENCTOR_SIZE],
}

impl Mcu {
    pub fn new() -> Self {
        Mcu {
            memory: create_memory(),
            exception_handling_vector: [0; EXCEPTION_HANDLING_VENCTOR_SIZE],
        }
    }

    pub fn write(&mut self, addr: u32, value: u8) -> Result<()> {
        match addr {
            0x00..=0xff => self.exception_handling_vector[addr as usize] = value,
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                self.memory[(addr - MEMORY_START_ADDR) as usize] = value
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
        Ok(())
    }

    pub fn read(&self, addr: u32) -> Result<u8> {
        match addr {
            0x00..=0xff => return Ok(self.exception_handling_vector[addr as usize]),
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                return Ok(self.memory[(addr - MEMORY_START_ADDR) as usize])
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{MEMORY_END_ADDR, MEMORY_SIZE, MEMORY_START_ADDR};

    use super::Mcu;

    #[test]
    fn test_write_memory() {
        let mut mcu = Mcu::new();
        mcu.memory[0] = 0xff;
        mcu.write(MEMORY_START_ADDR, 0xff).unwrap();
        assert_eq!(mcu.memory[0], 0xff);

        mcu.memory[MEMORY_SIZE - 1] = 0xff;
        mcu.write(MEMORY_END_ADDR, 0xff).unwrap();
        assert_eq!(mcu.memory[MEMORY_SIZE - 1], 0xff)
    }

    #[test]
    fn test_read_memory() {
        let mut mcu = Mcu::new();
        mcu.memory[0] = 0xff;
        assert_eq!(mcu.read(MEMORY_START_ADDR).unwrap(), 0xff);

        mcu.memory[MEMORY_SIZE - 1] = 0xff;
        assert_eq!(mcu.read(MEMORY_END_ADDR).unwrap(), 0xff)
    }
}

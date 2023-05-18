use crate::memory::{create_memory, Memory, MEMORY_END_ADDR, MEMORY_START_ADDR};
use anyhow::{bail, Result};
use std::error::Error;
#[derive(Debug, PartialEq)]
pub struct AddressAccessError;

impl Error for AddressAccessError {}

impl std::fmt::Display for AddressAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Occurred address access error.")
    }
}

// cpuは別 本来のmcuとは意味が異なるのであとで変えるかも
pub struct Mcu {
    pub memory: Memory,
}

impl Mcu {
    pub fn new() -> Self {
        Mcu {
            memory: create_memory(),
        }
    }

    pub fn write(&mut self, addr: u32, value: u8) -> Result<()> {
        match addr {
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                self.memory[(addr - MEMORY_START_ADDR) as usize] = value
            }
            _ => bail!("Invalid address [{:x}]", addr),
        }
        Ok(())
    }

    pub fn read(&self, addr: u32) -> Result<u8> {
        match addr {
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

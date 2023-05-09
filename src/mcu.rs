use crate::memory::{create_memory, Memory, MEMORY_END_ADDR, MEMORY_START_ADDR};
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

    pub fn write(&mut self, addr: u32, value: u8) -> Result<(), AddressAccessError> {
        match addr {
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                self.memory[(addr - MEMORY_START_ADDR) as usize] = value
            }
            _ => return Err(AddressAccessError),
        }
        Ok(())
    }

    pub fn read(&self, addr: u32) -> Result<u8, AddressAccessError> {
        match addr {
            MEMORY_START_ADDR..=MEMORY_END_ADDR => {
                return Ok(self.memory[(addr - MEMORY_START_ADDR) as usize])
            }
            _ => return Err(AddressAccessError),
        }
    }
}

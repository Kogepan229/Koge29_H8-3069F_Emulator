use crate::memory::{create_memory, Memory};

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
}

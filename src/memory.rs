pub const MEMORY_START_ADDR: u32 = 0xffbf20;
pub const MEMORY_END_ADDR: u32 = 0xffff1f;
pub const MEMORY_SIZE: usize = (MEMORY_END_ADDR - MEMORY_START_ADDR + 1) as usize;

pub type Memory = Box<[u8; MEMORY_SIZE]>;

pub fn create_memory() -> Memory {
    Box::new([0; MEMORY_SIZE])
}

#[allow(dead_code)]
pub fn print_memory(memory: &Memory) {
    for i in 0..MEMORY_SIZE {
        if (i + 1) % 32 == 0 {
            println!("{:02x}", memory[i]);
        } else {
            print!("{:02x} ", memory[i]);
        }
    }
}

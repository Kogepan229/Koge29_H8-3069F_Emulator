mod elf;
mod memory;

use memory::*;

fn main() {
    let memory: Memory = create_memory();
    elf::load(
        "D:/Desktop/VSCode/Koge29_H8-300H_Emulator/example/example1.elf".to_string(),
        memory,
    );
}

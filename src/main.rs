mod cpu;
mod elf;
mod mcu;
mod memory;
mod setting;

use crate::cpu::Cpu;
use mcu::Mcu;

fn main() {
    let mut mcu = Mcu::new();
    let mut cpu = Cpu::new(&mut mcu);

    //let mut memory: Memory = create_memory();
    elf::load(
        "D:/Desktop/VSCode/Koge29_H8-300H_Emulator/example/example2.elf".to_string(),
        &mut cpu,
    );
    // print_memory(&cpu.mcu.memory);

    cpu.run().unwrap();
}

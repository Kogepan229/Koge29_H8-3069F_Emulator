mod elf;

fn main() {
    let memory: Box<u8> = Box::new(200);
    elf::load(
        "D:/Desktop/VSCode/Koge29_H8-300H_Emulator/example/example1.elf".to_string(),
        memory,
    );
}

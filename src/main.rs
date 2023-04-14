mod elf;

fn main() {
    let memory: Box<u8> = Box::new(200);
    elf::load("".to_string(), memory);
    println!("Hello, world!");
}

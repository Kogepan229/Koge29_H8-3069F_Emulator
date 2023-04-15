use std::io::Read;

mod header;
mod parse_header;

fn read_elf(path: String) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("failed open elf");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    return buf;
}

pub fn load(elf_path: String, memory: Box<u8>) {
    let program = read_elf(elf_path);
    let (_, hd) = parse_header::parse_elf_header32(&program).unwrap();
    println!("{:?}", hd)
}

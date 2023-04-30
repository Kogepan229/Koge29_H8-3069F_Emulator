use crate::elf::program_header::SegmentType;
use crate::memory::*;
use std::io::Read;

mod header;
mod parse_header;
mod parse_program_header;
mod parse_section;
mod program_header;
mod section;
mod string_table;

fn read_elf(path: String) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("failed open elf");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    return buf;
}

pub fn load(elf_path: String, memory: &mut Memory) {
    let program = read_elf(elf_path);
    let (_, hd) = parse_header::parse_elf_header32(&program).unwrap();
    println!("{:?}", hd);

    let (_, sht) = parse_section::parse_section_header_table32(hd.shnum as usize)(
        &program[hd.shoff as usize..],
    )
    .unwrap();

    let raw_section_names_offset = sht[hd.shstrndx as usize].offset;
    let raw_section_names = &program[raw_section_names_offset as usize..];
    let sections = sht
        .into_iter()
        .map(|header| section::Section32 {
            name: string_table::parse_string_table_entry(
                &raw_section_names[header.name_idx as usize..],
            )
            .unwrap()
            .1,
            header,
        })
        .collect::<Vec<section::Section32>>();
    println!("{:#?}", sections);

    let (_, pht) = parse_program_header::parse_program_header_table32(hd.phnum as usize)(
        &program[hd.phoff as usize..],
    )
    .unwrap();
    println!("{:#?}", pht);

    for ph in pht {
        if ph.ty == SegmentType::Load {
            memory[ph.virtual_addr as usize..(ph.virtual_addr + ph.size_in_file) as usize]
                .copy_from_slice(
                    &program[ph.offset as usize..(ph.offset + ph.size_in_file) as usize],
                );
        }
    }
}

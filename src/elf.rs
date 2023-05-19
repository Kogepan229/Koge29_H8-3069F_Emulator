use crate::cpu::Cpu;
use crate::elf::program_header::SegmentType;
use crate::memory::MEMORY_START_ADDR;
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

pub fn load(elf_path: String, cpu: &mut Cpu) {
    let program = read_elf(elf_path);
    let (_, hd) = parse_header::parse_elf_header32(&program).unwrap();
    // println!("{:?}", hd);

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
    // println!("{:#?}", sections);

    let (_, pht) = parse_program_header::parse_program_header_table32(hd.phnum as usize)(
        &program[hd.phoff as usize..],
    )
    .unwrap();
    // println!("{:#?}", pht);

    // load to memory
    for ph in pht {
        if ph.ty == SegmentType::Load {
            cpu.mcu.memory[ph.virtual_addr as usize..(ph.virtual_addr + ph.size_in_file) as usize]
                .copy_from_slice(
                    &program[ph.offset as usize..(ph.offset + ph.size_in_file) as usize],
                );
        }
    }

    for s in sections {
        if s.name == ".got" {
            // set .got section address to er5
            cpu.er[5] = s.header.addr + MEMORY_START_ADDR;
            println!("Set er5 [{:x}({:x})]", cpu.er[5], s.header.addr);

            // Add start address of program to Global Offset
            for i in 0..(s.header.size / 4) {
                let mut global_off = ((cpu.mcu.memory[(s.header.addr + 4 * i) as usize] as u32)
                    << 24)
                    | ((cpu.mcu.memory[(s.header.addr + 4 * i + 1) as usize] as u32) << 16)
                    | ((cpu.mcu.memory[(s.header.addr + 4 * i + 2) as usize] as u32) << 8)
                    | (cpu.mcu.memory[(s.header.addr + 4 * i + 3) as usize] as u32);
                global_off += MEMORY_START_ADDR;
                cpu.mcu.memory
                    [((s.header.addr + 4 * i) as usize)..((s.header.addr + 4 * i + 4) as usize)]
                    .copy_from_slice(&global_off.to_be_bytes());
            }
        }
    }
}

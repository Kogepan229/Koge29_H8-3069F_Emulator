use crate::bus::AREA2_START_ADDR;
use crate::cpu::Cpu;
use crate::elf::parse_symtab::parse_symbol_table32;
use crate::elf::program_header::SegmentType;
use std::io::Read;

mod header;
mod parse_header;
mod parse_program_header;
mod parse_section;
mod parse_symtab;
mod program_header;
mod section;
mod string_table;
mod symtab;

const PROGRAM_START_ADDR: usize = 0x416900;

fn read_elf(path: String) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("failed open elf");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    return buf;
}

pub async fn load(elf_path: String, cpu: &mut Cpu) {
    let elf_binary = read_elf(elf_path);
    let (_, hd) = parse_header::parse_elf_header32(&elf_binary).unwrap();
    // println!("{:?}", hd);

    let (_, sht) = parse_section::parse_section_header_table32(hd.shnum as usize)(
        &elf_binary[hd.shoff as usize..],
    )
    .unwrap();

    let raw_section_names_offset = sht[hd.shstrndx as usize].offset;
    let raw_section_names = &elf_binary[raw_section_names_offset as usize..];
    let sections = sht
        .iter()
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
        &elf_binary[hd.phoff as usize..],
    )
    .unwrap();
    // println!("{:#?}", pht);

    let mut bus_lock = cpu.bus.lock().await;

    cpu.er[2] = PROGRAM_START_ADDR as u32;
    println!("Set er2 [0x{:x}]", cpu.er[2]);

    // load to memory
    let program_dram_offset = PROGRAM_START_ADDR - AREA2_START_ADDR as usize;
    for ph in pht {
        if ph.ty == SegmentType::Load {
            bus_lock.dram[program_dram_offset + ph.virtual_addr as usize
                ..program_dram_offset + (ph.virtual_addr + ph.size_in_file) as usize]
                .copy_from_slice(
                    &elf_binary[ph.offset as usize..(ph.offset + ph.size_in_file) as usize],
                );
        }
    }

    for s in sections {
        if s.name == ".got" {
            // set .got section address to er5
            cpu.er[5] = s.header.addr + PROGRAM_START_ADDR as u32;
            println!("Set er5 [0x{:x}(0x{:x})]", cpu.er[5], s.header.addr);

            // Add start address of program to Global Offset
            for i in 0..(s.header.size / 4) {
                let mut global_off = ((bus_lock.dram
                    [program_dram_offset + (s.header.addr + 4 * i) as usize]
                    as u32)
                    << 24)
                    | ((bus_lock.dram[program_dram_offset + (s.header.addr + 4 * i + 1) as usize]
                        as u32)
                        << 16)
                    | ((bus_lock.dram[program_dram_offset + (s.header.addr + 4 * i + 2) as usize]
                        as u32)
                        << 8)
                    | (bus_lock.dram[program_dram_offset + (s.header.addr + 4 * i + 3) as usize]
                        as u32);
                global_off += PROGRAM_START_ADDR as u32;
                bus_lock.dram[program_dram_offset + ((s.header.addr + 4 * i) as usize)
                    ..program_dram_offset + ((s.header.addr + 4 * i + 4) as usize)]
                    .copy_from_slice(&global_off.to_be_bytes());
            }
        } else if s.name == ".symtab" {
            let (_, symtabs) =
                parse_symbol_table32((s.header.size / s.header.entry_size) as usize)(
                    &elf_binary[s.header.offset as usize..],
                )
                .unwrap();
            // println!("{:#?}", symtabs);

            let raw_symbol_names_offset = sht[s.header.link as usize].offset;
            let raw_symbol_names = &elf_binary[raw_symbol_names_offset as usize..];
            let symtabs_with_name = symtabs
                .into_iter()
                .map(|symtab| -> symtab::SymbolTableWithName32 {
                    // nameの中にスペースがあるとエラー？
                    symtab::SymbolTableWithName32 {
                        name: string_table::parse_string_table_entry(
                            &raw_symbol_names[symtab.name_idx as usize..],
                        )
                        .unwrap_or_else(|_| (&raw_symbol_names, "Error".to_string()))
                        .1,
                        symtab,
                    }
                })
                .collect::<Vec<symtab::SymbolTableWithName32>>();
            // println!("{:#?}", symtabs_with_name);

            for symtab in symtabs_with_name {
                if symtab.name == "___exit" {
                    cpu.exit_addr = symtab.symtab.value + PROGRAM_START_ADDR as u32;
                    println!("Set ___exit address");
                }
            }
        }
    }
}

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

pub const PROGRAM_START_ADDR: usize = 0x416900;
const SIZE_OF_TCB: usize = 88;

fn read_elf(path: String) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("failed open elf");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    return buf;
}

pub fn load(elf_path: String, cpu: &mut Cpu, args: String) {
    let elf_binary = read_elf(elf_path);
    let (_, hd) = parse_header::parse_elf_header32(&elf_binary).unwrap();

    let (_, sht) = parse_section::parse_section_header_table32(hd.shnum as usize)(&elf_binary[hd.shoff as usize..]).unwrap();

    let raw_section_names_offset = sht[hd.shstrndx as usize].offset;
    let raw_section_names = &elf_binary[raw_section_names_offset as usize..];
    let sections = sht
        .iter()
        .map(|header| section::Section32 {
            name: string_table::parse_string_table_entry(&raw_section_names[header.name_idx as usize..])
                .unwrap()
                .1,
            header,
        })
        .collect::<Vec<section::Section32>>();

    let (_, pht) = parse_program_header::parse_program_header_table32(hd.phnum as usize)(&elf_binary[hd.phoff as usize..]).unwrap();

    cpu.er[2] = PROGRAM_START_ADDR as u32;
    log::info!("Set er2 [0x{:x}]", cpu.er[2]);

    // load to memory
    let program_dram_offset = PROGRAM_START_ADDR - AREA2_START_ADDR as usize;
    for ph in &pht {
        if ph.ty == SegmentType::Load {
            cpu.bus.dram
                [program_dram_offset + ph.virtual_addr as usize..program_dram_offset + (ph.virtual_addr + ph.size_in_file) as usize]
                .copy_from_slice(&elf_binary[ph.offset as usize..(ph.offset + ph.size_in_file) as usize]);
        }
    }

    for s in sections {
        if s.name == ".got" {
            // set .got section address to er5
            cpu.er[5] = s.header.addr + PROGRAM_START_ADDR as u32;
            log::info!("Set er5 [0x{:x}(0x{:x})]", cpu.er[5], s.header.addr);

            // Add start address of program to Global Offset
            for i in 0..(s.header.size / 4) {
                let got_addr = program_dram_offset + (s.header.addr + 4 * i) as usize;
                let mut global_off = ((cpu.bus.dram[got_addr] as u32) << 24)
                    | ((cpu.bus.dram[got_addr + 1] as u32) << 16)
                    | ((cpu.bus.dram[got_addr + 2] as u32) << 8)
                    | (cpu.bus.dram[got_addr + 3] as u32);
                global_off += PROGRAM_START_ADDR as u32;
                cpu.bus.dram[got_addr..=got_addr + 3].copy_from_slice(&global_off.to_be_bytes());
            }
        } else if s.name == ".stack" {
            let program_size = pht[pht.len() - 1].size_in_mem + pht[pht.len() - 1].physical_addr;
            let stack_size = s.header.addr;
            let mut a = PROGRAM_START_ADDR + program_size as usize + stack_size as usize + 3;
            a >>= 2;
            a <<= 2;
            cpu.er[7] = a as u32 - 8;
            log::info!("Set er7(stack pointer) [0x{:x}]", cpu.er[7]);

            a += SIZE_OF_TCB + 3;
            a >>= 2;
            a <<= 2;

            log::info!("args: [{}]", args);
            let mut args_list: Vec<&str> = args.split_whitespace().collect();
            args_list.insert(0, "prog.elf");

            cpu.er[0] = args_list.len() as u32;
            log::info!("Set er0 [0x{:x}]", cpu.er[0]);
            cpu.er[1] = a as u32;
            log::info!("Set er1 [0x{:x}]", cpu.er[1]);

            let mut argp = a;
            a += 4 * (args_list.len() + 1);

            for arg in args_list {
                // Set args pointer
                let argp_i = argp - AREA2_START_ADDR as usize;
                cpu.bus.dram[argp_i..argp_i + 4].copy_from_slice(&(a as u32).to_be_bytes());
                argp += 4;

                // Set args string
                for char in arg.as_bytes() {
                    cpu.bus.dram[a - AREA2_START_ADDR as usize] = *char;
                    a += 1;
                }
                cpu.bus.dram[a - AREA2_START_ADDR as usize] = b'\0';
                a += 1;
            }
        } else if s.name == ".symtab" {
            let (_, symtabs) =
                parse_symbol_table32((s.header.size / s.header.entry_size) as usize)(&elf_binary[s.header.offset as usize..]).unwrap();

            let raw_symbol_names_offset = sht[s.header.link as usize].offset;
            let raw_symbol_names = &elf_binary[raw_symbol_names_offset as usize..];
            let symtabs_with_name = symtabs
                .into_iter()
                .map(|symtab| -> symtab::SymbolTableWithName32 {
                    // nameの中にスペースがあるとエラー？
                    symtab::SymbolTableWithName32 {
                        name: string_table::parse_string_table_entry(&raw_symbol_names[symtab.name_idx as usize..])
                            .unwrap_or_else(|_| (&raw_symbol_names, "Error".to_string()))
                            .1,
                        symtab,
                    }
                })
                .collect::<Vec<symtab::SymbolTableWithName32>>();

            for symtab in symtabs_with_name {
                if symtab.name == "___exit" {
                    cpu.exit_addr = symtab.symtab.value + PROGRAM_START_ADDR as u32;
                    log::info!("Set ___exit address [0x{:x}]", cpu.exit_addr);
                }
            }
        }
    }
}

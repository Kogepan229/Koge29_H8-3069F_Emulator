/*
The major code of this file was based on https://github.com/Drumato/elf-parser-in-rust-book/blob/impl/src/parser/header.rs
Here is the original copyright notice.

MIT License

Copyright (c) 2021 Drumato

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use nom::multi::count;
use nom::number::complete::{be_u16, be_u32};
use nom::{bytes::complete::tag, combinator::map, number::complete::u8 as read_u8};
use nom::{IResult, Parser};

use crate::elf::header::{ElfClass, ElfData, ElfHeader32, ElfIdentification, ElfOsAbi, ElfVersion, ELF_MAGIC_SIGNATURE};

// 32bit, big endian only

pub fn parse_elf_header32(raw: &[u8]) -> IResult<&[u8], ElfHeader32> {
    let (r, ident) = parse_elf_identification(raw)?;
    let (r, e_type) = be_u16(r)?;
    let (r, machine) = be_u16(r)?;
    let (r, version) = be_u32(r)?;
    let (r, entry) = be_u32(r)?;
    let (r, phoff) = be_u32(r)?;
    let (r, shoff) = be_u32(r)?;
    let (r, flags) = be_u32(r)?;
    let (r, ehsize) = be_u16(r)?;
    let (r, phentsize) = be_u16(r)?;
    let (r, phnum) = be_u16(r)?;
    let (r, shentsize) = be_u16(r)?;
    let (r, shnum) = be_u16(r)?;
    let (r, shstrndx) = be_u16(r)?;
    Ok((
        r,
        ElfHeader32 {
            ident,
            e_type,
            machine,
            version,
            entry,
            phoff,
            shoff,
            flags,
            ehsize,
            phentsize,
            phnum,
            shentsize,
            shnum,
            shstrndx,
        },
    ))
}

fn parse_elf_magic_number(raw: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(ELF_MAGIC_SIGNATURE)(raw)
}

fn parse_elf_class(raw: &[u8]) -> IResult<&[u8], ElfClass> {
    map(read_u8, |byte: u8| ElfClass::from(byte)).parse(raw)
}

fn parse_elf_data(raw: &[u8]) -> IResult<&[u8], ElfData> {
    map(read_u8, |byte: u8| ElfData::from(byte)).parse(raw)
}

fn parse_elf_version(raw: &[u8]) -> IResult<&[u8], ElfVersion> {
    map(read_u8, |byte: u8| ElfVersion::from(byte)).parse(raw)
}

fn parse_elf_osabi(raw: &[u8]) -> IResult<&[u8], ElfOsAbi> {
    map(read_u8, |byte: u8| ElfOsAbi::from(byte)).parse(raw)
}

fn parse_elf_abi_version(raw: &[u8]) -> IResult<&[u8], u8> {
    read_u8(raw)
}

fn parse_elf_identification(raw: &[u8]) -> IResult<&[u8], ElfIdentification> {
    let (r, _) = parse_elf_magic_number(raw)?;
    let (r, class) = parse_elf_class(r)?;
    let (r, data) = parse_elf_data(r)?;
    let (r, version) = parse_elf_version(r)?;
    let (r, osabi) = parse_elf_osabi(r)?;
    let (r, abi_version) = parse_elf_abi_version(r)?;
    let (r, _padding) = count(read_u8, 7).parse(r)?;

    Ok((
        r,
        ElfIdentification {
            class,
            data,
            version,
            osabi,
            abi_version,
        },
    ))
}

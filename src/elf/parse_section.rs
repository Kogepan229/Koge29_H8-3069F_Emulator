/*
The major code of this file was based on https://github.com/Drumato/elf-parser-in-rust-book/blob/impl/src/parser/section.rs
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

use nom::{combinator::map, multi::count, number::complete::be_u32, IResult};

use crate::elf::section::{SectionHeader32, SectionType};

pub fn parse_section_header_table32<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<SectionHeader32>> {
    move |raw: &'a [u8]| count(parse_section_header32, entries)(raw)
}

fn parse_section_type(raw: &[u8]) -> IResult<&[u8], SectionType> {
    map(be_u32, |v| SectionType::from(v))(raw)
}

fn parse_section_header32(raw: &[u8]) -> IResult<&[u8], SectionHeader32> {
    let (r, name_idx) = be_u32(raw)?;
    let (r, ty) = parse_section_type(r)?;
    let (r, flags) = be_u32(r)?;
    let (r, addr) = be_u32(r)?;
    let (r, offset) = be_u32(r)?;
    let (r, size) = be_u32(r)?;
    let (r, link) = be_u32(r)?;
    let (r, info) = be_u32(r)?;
    let (r, addr_align) = be_u32(r)?;
    let (r, entry_size) = be_u32(r)?;

    Ok((
        r,
        SectionHeader32 {
            name_idx,
            ty,
            flags,
            addr,
            offset,
            size,
            link,
            info,
            addr_align,
            entry_size,
        },
    ))
}

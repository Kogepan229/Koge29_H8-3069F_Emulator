/*
The major code of this file was based on https://github.com/Drumato/elf-parser-in-rust-book/blob/impl/src/parser/program_header.rs
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

use crate::elf::program_header::{ProgramHeader32, SegmentType};

pub fn parse_program_header_table32<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<ProgramHeader32>> {
    move |raw: &'a [u8]| count(parse_program_header32, entries)(raw)
}

fn parse_segment_type(raw: &[u8]) -> IResult<&[u8], SegmentType> {
    map(be_u32, |v| SegmentType::from(v))(raw)
}

fn parse_program_header32(raw: &[u8]) -> IResult<&[u8], ProgramHeader32> {
    let (r, ty) = parse_segment_type(raw)?;
    let (r, offset) = be_u32(r)?;
    let (r, virtual_addr) = be_u32(r)?;
    let (r, physical_addr) = be_u32(r)?;
    let (r, size_in_file) = be_u32(r)?;
    let (r, size_in_mem) = be_u32(r)?;
    let (r, flags) = be_u32(r)?;
    let (r, align) = be_u32(r)?;

    Ok((
        r,
        ProgramHeader32 {
            ty,
            flags,
            offset,
            virtual_addr,
            physical_addr,
            size_in_file,
            size_in_mem,
            align,
        },
    ))
}

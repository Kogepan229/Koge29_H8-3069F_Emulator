/*
The major code of this file was based on https://github.com/Drumato/elf-parser-in-rust-book/blob/impl/src/elf/program_header.rs
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramHeader32 {
    pub ty: SegmentType,
    pub offset: u32,
    pub virtual_addr: u32,
    pub physical_addr: u32,
    pub size_in_file: u32,
    pub size_in_mem: u32,
    pub flags: u32,
    pub align: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SegmentType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    ShLib = 5,
    Phdr = 6,
    TLS = 7,
    Num = 8,
    GNUEHFrame = 0x6474e550,
    GNUStack = 0x6474e551,
    GNURelRO = 0x6474e552,
    Unknown,
}

impl From<u32> for SegmentType {
    fn from(v: u32) -> SegmentType {
        match v {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interp,
            4 => SegmentType::Note,
            5 => SegmentType::ShLib,
            6 => SegmentType::Phdr,
            7 => SegmentType::TLS,
            8 => SegmentType::Num,
            0x6474e550 => SegmentType::GNUEHFrame,
            0x6474e551 => SegmentType::GNUStack,
            0x6474e552 => SegmentType::GNURelRO,
            _ => SegmentType::Unknown,
        }
    }
}

use crate::elf::symtab::SymbolTable32;
use nom::{
    combinator::map, multi::count, number::complete::be_u16, number::complete::be_u32,
    number::complete::be_u8, IResult,
};

pub fn parse_symbol_table32<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<SymbolTable32>> {
    move |raw: &'a [u8]| count(parse_symbol_table_header32, entries)(raw)
}

fn parse_symbol_table_header32(raw: &[u8]) -> IResult<&[u8], SymbolTable32> {
    let (r, name_idx) = be_u32(raw)?;
    let (r, value) = be_u32(r)?;
    let (r, size) = be_u32(r)?;
    let (r, info) = be_u8(r)?;
    let (r, other) = be_u8(r)?;
    let (r, shndx) = be_u16(r)?;

    Ok((
        r,
        SymbolTable32 {
            name_idx,
            value,
            size,
            info,
            other,
            shndx,
        },
    ))
}

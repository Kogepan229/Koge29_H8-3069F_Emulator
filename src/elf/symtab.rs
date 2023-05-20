#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SymbolTableWithName32 {
    pub name: String,
    pub symtab: SymbolTable32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SymbolTable32 {
    pub name_idx: u32,
    pub value: u32,
    pub size: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
}

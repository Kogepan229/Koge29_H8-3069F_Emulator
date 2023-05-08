use crate::mcu::Mcu;

mod addressing_mode;

pub struct Cpu<'a> {
    mcu: &'a Mcu,
    cp: u32,
    cr: u8,
    er: [u32; 8],
}

#[derive(Debug, Clone, Copy)]
enum Resister8 {
    R0H,
    R1H,
    R2H,
    R3H,
    R4H,
    R5H,
    R6H,
    R7H,
    R0L,
    R1L,
    R2L,
    R3L,
    R4L,
    R5L,
    R6L,
    R7L,
}

#[derive(Debug, Clone, Copy)]
enum Resister16 {
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug, Clone, Copy)]
enum Resister32 {
    ER0,
    ER1,
    ER2,
    ER3,
    ER4,
    ER5,
    ER6,
    ER7,
}

impl<'a> Cpu<'a> {
    pub fn new(mcu: &'a Mcu) -> Self {
        Cpu {
            mcu: mcu,
            cp: 0,
            cr: 0,
            er: [0; 8],
        }
    }
}

use crate::mcu::Mcu;

mod addressing_mode;

pub struct Cpu<'a> {
    mcu: &'a mut Mcu,
    pc: u32,
    cr: u8,
    er: [u32; 8],
}

impl<'a> Cpu<'a> {
    pub fn new(mcu: &'a mut Mcu) -> Self {
        Cpu {
            mcu: mcu,
            pc: 0,
            cr: 0,
            er: [0; 8],
        }
    }
}

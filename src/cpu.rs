use crate::mcu::Mcu;

pub struct Cpu<'a> {
    mcu: &'a Mcu,
    cp: u32,
    cr: u8,
    er: [u32; 8],
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

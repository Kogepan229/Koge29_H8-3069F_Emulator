mod mov_l;

use super::*;

impl<'a> Cpu<'a> {
    // order: 1 ~ 4 [0x1234]
    pub fn get_nibble_opcode(opcode: u16, order: u8) -> u8 {
        match order {
            1..=4 => ((opcode >> (4 * (4 - order))) as u8) & 0xf,
            _ => panic!("invalud order"),
        }
    }
}

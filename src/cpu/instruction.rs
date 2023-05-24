mod add_b;
mod add_l;
mod add_w;
mod adds;
mod bcc;
mod cmp_b;
mod cmp_l;
mod cmp_w;
mod jmp;
mod jsr;
mod mov_l;
mod mov_w;
mod rts;
mod sub_b;
mod sub_l;
mod sub_w;
mod subs;

use crate::cpu::Cpu;
use anyhow::{bail, Result};

impl<'a> Cpu<'a> {
    // order: 1 ~ 4 [0x1234]
    pub fn get_nibble_opcode(opcode: u16, order: u8) -> Result<u8> {
        match order {
            1..=4 => Ok(((opcode >> (4 * (4 - order))) as u8) & 0xf),
            _ => bail!("Invalid order [{}]", order),
        }
    }
}

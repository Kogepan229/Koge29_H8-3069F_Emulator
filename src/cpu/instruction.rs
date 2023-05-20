mod bcc;
mod cmp_l;
mod jsr;
mod mov_l;
mod sub_l;
mod subs;

use super::*;
use anyhow::{bail, Context, Result};

impl<'a> Cpu<'a> {
    // order: 1 ~ 4 [0x1234]
    pub fn get_nibble_opcode(opcode: u16, order: u8) -> Result<u8> {
        match order {
            1..=4 => Ok(((opcode >> (4 * (4 - order))) as u8) & 0xf),
            _ => bail!("Invalid order [{}]", order),
        }
    }
}

mod add_b;
mod add_l;
mod add_w;
mod adds;
mod and;
mod band;
mod bcc;
mod bclr;
mod biand;
mod bild;
mod bior;
mod bixor;
mod bld;
mod bnot;
mod bor;
mod bset;
mod bst;
mod btst;
mod bxor;
mod cmp_b;
mod cmp_l;
mod cmp_w;
mod dec;
mod inc;
mod jmp;
mod jsr;
mod mov_b;
mod mov_l;
mod mov_w;
mod or;
mod rotl;
mod rotr;
mod rotxl;
mod rotxr;
mod rts;
mod shal;
mod shar;
mod shll;
mod shlr;
mod sub_b;
mod sub_l;
mod sub_w;
mod subs;

use crate::cpu::Cpu;
use anyhow::{bail, Result};

impl Cpu {
    // order: 1 ~ 4 [0x1234]
    pub fn get_nibble_opcode(opcode: u16, order: u8) -> Result<u8> {
        match order {
            1..=4 => Ok(((opcode >> (4 * (4 - order))) as u8) & 0xf),
            _ => bail!("Invalid order [{}]", order),
        }
    }
}

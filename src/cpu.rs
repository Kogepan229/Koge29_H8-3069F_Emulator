use crate::{
    mcu::Mcu,
    memory::{self, MEMORY_END_ADDR, MEMORY_START_ADDR},
};
use anyhow::{Context as _, Result};
use std::{arch::x86_64::_mm_loaddup_pd, thread, time::Duration};

mod addressing_mode;
mod instruction;

const CPUCLOCK: usize = 20000000;

pub struct Cpu<'a> {
    pub mcu: &'a mut Mcu,
    pc: u32,
    ccr: u8,
    er: [u32; 8],
}

pub enum CCR {
    C,
    V,
    Z,
    N,
    U,
    H,
    I,
}

impl<'a> Cpu<'a> {
    pub fn new(mcu: &'a mut Mcu) -> Self {
        Cpu {
            mcu: mcu,
            pc: MEMORY_START_ADDR,
            ccr: 0,
            er: [0; 8],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let opcode = self.fetch();
            let state = self
                .exec(opcode)
                .with_context(|| format!("opcode1 [{:0>4x}]", opcode))?;
            thread::sleep(Duration::from_secs_f64(
                state as f64 * 1.0 / CPUCLOCK as f64,
            ))
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let _pc = self.pc & !1;
        if _pc < MEMORY_START_ADDR || _pc > MEMORY_END_ADDR {
            panic!("fetch error [pc: {:4x}]", self.pc)
        }
        let op = (self.mcu.memory[(_pc - MEMORY_START_ADDR) as usize] as u16) << 8
            | (self.mcu.memory[(_pc - MEMORY_START_ADDR + 1) as usize] as u16);
        self.pc += 2;
        op
    }

    fn exec(&mut self, opcode: u16) -> Result<usize> {
        match opcode {
            0x0f80..=0x0ff7 | 0x7a00..=0x7a07 | 0x0100 => return self.mov_l(opcode),
            _ => panic!(
                "exec error. [opcode: {:>02x} {:>02x}]",
                opcode >> 8 as u8,
                opcode as u8
            ),
        }
    }

    pub fn write_ccr(&mut self, target: CCR, val: u8) {
        match val {
            0 => self.ccr &= !(1 << target as u8),
            1 => self.ccr |= 1 << target as u8,
            _ => panic!("[write_ccr] invalid value"),
        }
    }

    pub fn read_ccr(&self, target: CCR) -> u8 {
        (self.ccr >> target as u8) & 1
    }
}

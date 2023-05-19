use crate::{
    mcu::Mcu,
    memory::{MEMORY_END_ADDR, MEMORY_START_ADDR},
};
use anyhow::{bail, Context as _, Result};
use std::{thread, time::Duration};

mod addressing_mode;
mod instruction;

const CPUCLOCK: usize = 20000000;

pub struct Cpu<'a> {
    pub mcu: &'a mut Mcu,
    pc: u32,
    ccr: u8,
    pub er: [u32; 8],
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
        self.er[7] = MEMORY_END_ADDR - 0xf;
        loop {
            print!(" {:4x}:   ", self.pc.wrapping_sub(MEMORY_START_ADDR));

            let opcode = self.fetch();
            let state = self.exec(opcode).with_context(|| {
                format!(
                    "[pc: {:0>8x}({:0>8x})] opcode1 [{:0>4x}]",
                    self.pc - 2,
                    self.pc - 2 - MEMORY_START_ADDR,
                    opcode
                )
            })?;
            println!("");
            thread::sleep(Duration::from_secs_f64(
                state as f64 * 1.0 / CPUCLOCK as f64,
            ))
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let _pc = self.pc & !1;
        if _pc < MEMORY_START_ADDR || _pc > MEMORY_END_ADDR {
            panic!("fetch error [pc: {:0>8x}]", self.pc)
        }
        let op = (self.mcu.memory[(_pc - MEMORY_START_ADDR) as usize] as u16) << 8
            | (self.mcu.memory[(_pc - MEMORY_START_ADDR + 1) as usize] as u16);
        print!("{:0>2x} {:0>2x} ", (op >> 8) as u8, op as u8);
        self.pc += 2;
        op
    }

    fn exec(&mut self, opcode: u16) -> Result<usize> {
        match ((opcode & 0xff00) >> 8) as u8 {
            0x01 | 0x0f | 0x7a => return self.mov_l(opcode),
            0x1b => return self.subs(opcode),
            0x5d | 0x5e | 0x5f => return self.jsr(opcode),
            _ => bail!(
                "unimplemented instruction [{:>04x}] pc [{:x}]",
                opcode,
                self.pc - 2
            ),
        }

        // match opcode {
        //     0x0f80..=0x0ff7 | 0x7a00..=0x7a07 | 0x0100 => return self.mov_l(opcode),
        //     _ => bail!(
        //         "unimplemented instruction [{:>04x}] pc [{:x}]",
        //         opcode,
        //         self.pc - 2
        //     ),
        // }
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

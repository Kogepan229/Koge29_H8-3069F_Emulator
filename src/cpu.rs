use std::{arch::x86_64::_mm_loaddup_pd, thread, time::Duration};

use crate::{
    mcu::Mcu,
    memory::{self, MEMORY_END_ADDR, MEMORY_START_ADDR},
};

mod addressing_mode;

const CPUCLOCK: usize = 20000000;

pub struct Cpu<'a> {
    pub mcu: &'a mut Mcu,
    pc: u32,
    ccr: u8,
    er: [u32; 8],
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

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            let state = self.exec(opcode);
            thread::sleep(Duration::from_secs_f64(
                state as f64 * 1.0 / CPUCLOCK as f64,
            ))
        }
    }

    fn fetch(&mut self) -> u16 {
        let _pc = self.pc & !1;
        if _pc < MEMORY_START_ADDR || _pc > MEMORY_END_ADDR {
            panic!("fetch error [pc: {:4x}]", self.pc)
        }
        let op = (self.mcu.memory[(_pc - MEMORY_START_ADDR) as usize] as u16) << 8
            | (self.mcu.memory[(_pc - MEMORY_START_ADDR + 1) as usize] as u16);
        self.pc += 2;
        op
    }

    fn exec(&self, opcode: u16) -> u8 {
        match opcode {
            _ => panic!(
                "exec error. [opcode: {:>02x} {:>02x}]",
                opcode >> 8 as u8,
                opcode as u8
            ),
        }
    }
}

use crate::{
    mcu::Mcu,
    memory::{MEMORY_END_ADDR, MEMORY_START_ADDR},
    setting,
};
use anyhow::{bail, Context as _, Result};
use std::time;
use std::time::Duration;

mod addressing_mode;
mod instruction;

const CPUCLOCK: usize = 20000000;

pub struct Cpu {
    pub mcu: Mcu,
    pc: u32,
    ccr: u8,
    pub er: [u32; 8],
    pub exit_addr: u32, // address of ___exit
}

pub enum CCR {
    C,
    V,
    Z,
    N,
    U,
    H,
    UI,
    I,
}

macro_rules! unimpl {
    ($op:expr, $pc:expr ) => {
        bail!(
            "unimplemented instruction:[{:>04x}] pc:[{:x}({:x})]",
            $op,
            $pc - 2,
            $pc - 2 - MEMORY_START_ADDR
        )
    };
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            mcu: Mcu::new(),
            pc: MEMORY_START_ADDR,
            ccr: 0,
            er: [0; 8],
            exit_addr: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut state_sum: usize = 0;
        let mut loop_count: usize = 0;
        self.er[7] = MEMORY_END_ADDR - 0xf;
        let exec_time = time::Instant::now();
        let mut loop_time = time::Instant::now();
        loop {
            if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
                print!(" {:4x}:   ", self.pc.wrapping_sub(MEMORY_START_ADDR));
            }

            let opcode = self.fetch();
            let state = self.exec(opcode).with_context(|| {
                format!(
                    "[pc: {:0>8x}({:0>8x})] opcode1 [{:0>4x}]",
                    self.pc - 2,
                    self.pc - 2 - MEMORY_START_ADDR,
                    opcode
                )
            })?;
            state_sum += state;
            loop_count += state;

            if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
                println!("");
            }

            if self.pc == self.exit_addr {
                self.print_er();
                println!(
                    "state: {}, time: {}sec",
                    state_sum,
                    exec_time.elapsed().as_secs_f64()
                );
                return Ok(());
            }

            // sleep every 1msec (Windows timer max precision)
            if loop_count >= 20000 {
                spin_sleep::sleep(
                    Duration::from_secs_f64(loop_count as f64 * 1.0 / CPUCLOCK as f64)
                        .saturating_sub(loop_time.elapsed()),
                );
                loop_count = 0;
                loop_time = time::Instant::now();
            }
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let _pc = self.pc & !1;
        if _pc < MEMORY_START_ADDR || _pc > MEMORY_END_ADDR {
            panic!("fetch error [pc: {:0>8x}]", self.pc)
        }
        let op = (self.mcu.memory[(_pc - MEMORY_START_ADDR) as usize] as u16) << 8
            | (self.mcu.memory[(_pc - MEMORY_START_ADDR + 1) as usize] as u16);

        if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
            print!("{:0>2x} {:0>2x} ", (op >> 8) as u8, op as u8);
        }

        self.pc += 2;
        op
    }

    fn exec(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x0c | 0xf0..=0xff | 0x68 | 0x6e | 0x6c | 0x20..=0x2f | 0x30..=0x3f | 0x6a => {
                return self.mov_b(opcode)
            }
            0x0d => return self.mov_w(opcode),
            0x69 | 0x6f | 0x6d | 0x6b => return self.mov_w(opcode),
            0x0f => return self.mov_l(opcode),

            0x01 => match opcode as u8 {
                0x00 => return self.mov_l(opcode),
                0xf0 => {
                    let opcode2 = self.fetch();
                    match (opcode2 >> 8) as u8 {
                        0x64 => return self.or_l_rn(opcode, opcode2),
                        0x66 => return self.and_l_rn(opcode, opcode2),
                        _ => unimpl!(opcode, self.pc),
                    }
                }
                _ => unimpl!(opcode, self.pc),
            },

            0x78 => {
                let opcode2 = self.fetch();
                match (opcode2 >> 8) as u8 {
                    0x6a => return self.mov_b_disp24(opcode, opcode2),
                    0x6b => return self.mov_w_disp24(opcode, opcode2),
                    _ => unimpl!(opcode, self.pc),
                }
            }

            0x79 => match opcode & 0x00f0 {
                0x0 => return self.mov_w(opcode),
                0x0010 => return self.add_w(opcode),
                0x0020 => return self.cmp_w(opcode),
                0x0030 => return self.sub_w(opcode),
                0x0040 => return self.or_w_imm(opcode),
                0x0060 => return self.and_w_imm(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x7a => match opcode & 0x00f0 {
                0x0 => return self.mov_l(opcode),
                0x0010 => return self.add_l(opcode),
                0x0020 => return self.cmp_l(opcode),
                0x0030 => return self.sub_l(opcode),
                0x0040 => return self.or_l_imm(opcode),
                0x0060 => return self.and_l_imm(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x0a => match opcode as u8 {
                0x00..=0x0f => return self.inc_b(opcode),
                0x80..=0xf7 => return self.add_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x0b => match opcode as u8 {
                0x50..=0x5f => return self.inc_w_1(opcode),
                0xd0..=0xdf => return self.inc_w_2(opcode),
                0x70..=0x77 => return self.inc_l_1(opcode),
                0xf0..=0xf7 => return self.inc_l_2(opcode),
                0x00..=0x07 => return self.adds1(opcode),
                0x80..=0x87 => return self.adds2(opcode),
                0x90..=0x97 => return self.adds4(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x10 => match opcode as u8 {
                0x00..=0x0f => return self.shll_b(opcode),
                0x10..=0x1f => return self.shll_w(opcode),
                0x30..=0x37 => return self.shll_l(opcode),
                0x80..=0x8f => return self.shal_b(opcode),
                0x90..=0x9f => return self.shal_w(opcode),
                0xb0..=0xb7 => return self.shal_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x11 => match opcode as u8 {
                0x00..=0x0f => return self.shlr_b(opcode),
                0x10..=0x1f => return self.shlr_w(opcode),
                0x30..=0x3f => return self.shlr_l(opcode),
                0x80..=0x8f => return self.shar_b(opcode),
                0x90..=0x9f => return self.shar_w(opcode),
                0xb0..=0xb7 => return self.shar_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x12 => match opcode as u8 {
                0x80..=0x8f => return self.rotl_b(opcode),
                0x90..=0x9f => return self.rotl_w(opcode),
                0xb0..=0xb7 => return self.rotl_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x1a => match opcode as u8 {
                0x00..=0x0f => return self.dec_b(opcode),
                0x80..=0xf7 => return self.sub_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x1b => match opcode as u8 {
                0x50..=0x5f => return self.dec_w_1(opcode),
                0xd0..=0xdf => return self.dec_w_2(opcode),
                0x70..=0x77 => return self.dec_l_1(opcode),
                0xf0..=0xf7 => return self.dec_l_2(opcode),
                0x00..=0x07 => return self.subs1(opcode),
                0x80..=0x87 => return self.subs2(opcode),
                0x90..=0x97 => return self.subs4(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x80..=0x8f | 0x08 => return self.add_b(opcode),
            0x09 => return self.add_w(opcode),

            0x18 => return self.sub_b(opcode),
            0x19 => return self.sub_w(opcode),

            0x1c | 0xa0..=0xaf => return self.cmp_b(opcode),
            0x1d => return self.cmp_w(opcode),
            0x1f => return self.cmp_l(opcode),

            0xc0..=0xcf => return self.or_b_imm(opcode),
            0x14 => return self.or_b_rn(opcode),
            0x64 => return self.or_w_rn(opcode),

            0xe0..=0xef => return self.and_b_imm(opcode),
            0x16 => return self.and_b_rn(opcode),
            0x66 => return self.and_w_rn(opcode),

            0x59 | 0x5a | 0x5b => return self.jmp(opcode),
            0x5d | 0x5e | 0x5f => return self.jsr(opcode),
            0x40..=0x4f | 0x58 => return self.bcc(opcode),
            0x54 => return self.rts(),
            0x57 => Ok(14), // Ignore TRAPA
            _ => unimpl!(opcode, self.pc),
        }
    }

    pub fn write_ccr(&mut self, target: CCR, val: u8) {
        match val {
            0 => self.ccr &= !(1 << target as u8),
            1 => self.ccr |= 1 << target as u8,
            _ => panic!("[write_ccr] invalid value [{:x}]", val),
        }
    }

    pub fn read_ccr(&self, target: CCR) -> u8 {
        (self.ccr >> target as u8) & 1
    }

    fn print_er(&self) {
        for i in 0..8 {
            print!("ER{}:[{:x}] ", i, self.er[i]);
        }
        println!("");
    }
}

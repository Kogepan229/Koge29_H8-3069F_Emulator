use crate::{
    bus::Bus,
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
    pub bus: Bus,
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
            bus: Bus::new(),
            pc: MEMORY_START_ADDR,
            ccr: 0,
            er: [0; 8],
            exit_addr: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut state_sum: usize = 0;
        let exec_time = time::Instant::now();

        let mut loop_count: usize = 0;
        let mut loop_time = time::Instant::now();

        // set stack pointer
        self.er[7] = MEMORY_END_ADDR - 0xf;

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
                spin_sleep_tokio::sleep(
                    Duration::from_secs_f64(loop_count as f64 * 1.0 / CPUCLOCK as f64)
                        .saturating_sub(loop_time.elapsed()),
                )
                .await;
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
        let op = (self.bus.memory[(_pc - MEMORY_START_ADDR) as usize] as u16) << 8
            | (self.bus.memory[(_pc - MEMORY_START_ADDR + 1) as usize] as u16);

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
                        0x65 => return self.xor_l_rn(opcode, opcode2),
                        0x66 => return self.and_l_rn(opcode, opcode2),
                        _ => unimpl!(opcode, self.pc),
                    }
                }
                _ => unimpl!(opcode, self.pc),
            },

            0x55 => return self.bsr_disp16(opcode),
            0x5c => return self.bsr_disp24(opcode),

            0x60 => return self.bset_rn_from_rn(opcode),
            0x61 => return self.bnot_rn_from_rn(opcode),
            0x62 => return self.bclr_rn_from_rn(opcode),
            0x63 => return self.btst_rn_from_rn(opcode),

            0x67 => match opcode & 0x80 {
                0x00 => return self.bst_rn(opcode),
                0x80 => return self.bist_rn(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x70 => return self.bset_rn_from_imm(opcode),
            0x71 => return self.bnot_rn_from_imm(opcode),
            0x72 => return self.bclr_rn_from_imm(opcode),
            0x73 => return self.btst_rn_from_imm(opcode),

            0x74 => match opcode & 0x80 {
                0x00 => return self.bor_rn(opcode),
                0x80 => return self.bior_rn(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x75 => match opcode & 0x80 {
                0x00 => return self.bxor_rn(opcode),
                0x80 => return self.bixor_rn(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x76 => match opcode & 0x80 {
                0x00 => return self.band_rn(opcode),
                0x80 => return self.biand_rn(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x77 => match opcode & 0x80 {
                0x00 => return self.bld_rn(opcode),
                0x080 => return self.bild_rn(opcode),
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
                0x0050 => return self.xor_w_imm(opcode),
                0x0060 => return self.and_w_imm(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x7a => match opcode & 0x00f0 {
                0x0 => return self.mov_l(opcode),
                0x0010 => return self.add_l(opcode),
                0x0020 => return self.cmp_l(opcode),
                0x0030 => return self.sub_l(opcode),
                0x0040 => return self.or_l_imm(opcode),
                0x0050 => return self.xor_l_imm(opcode),
                0x0060 => return self.and_l_imm(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x7c => {
                let opcode2 = self.fetch();
                match opcode2 & 0xff80 {
                    0x6300 | 0x6380 | 0x7300 => return self.btst_ern(opcode, opcode2),
                    0x7400 => return self.bor_ern(opcode, opcode2),
                    0x7480 => return self.bior_ern(opcode, opcode2),
                    0x7500 => return self.bxor_ern(opcode, opcode2),
                    0x7580 => return self.bixor_ern(opcode, opcode2),
                    0x7600 => return self.band_ern(opcode, opcode2),
                    0x7680 => return self.biand_ern(opcode, opcode2),
                    0x7700 => return self.bld_ern(opcode, opcode2),
                    0x7780 => return self.bild_ern(opcode, opcode2),
                    _ => unimpl!(opcode, self.pc),
                }
            }

            0x7d => {
                let opcode2 = self.fetch();
                match opcode2 & 0xff80 {
                    0x6000 | 0x6080 | 0x7000 => return self.bset_ern(opcode, opcode2),
                    0x6100 | 0x6180 | 0x7100 => return self.bnot_ern(opcode, opcode2),
                    0x6200 | 0x6280 | 0x7200 => return self.bclr_ern(opcode, opcode2),
                    0x6700 => return self.bst_ern(opcode, opcode2),
                    0x6780 => return self.bist_ern(opcode, opcode2),
                    _ => unimpl!(opcode, self.pc),
                }
            }

            0x7e => {
                let opcode2 = self.fetch();
                match opcode2 & 0xff80 {
                    0x6300 | 0x6380 | 0x7300 => return self.btst_abs(opcode, opcode2),
                    0x7400 => return self.bor_abs(opcode, opcode2),
                    0x7480 => return self.bior_abs(opcode, opcode2),
                    0x7500 => return self.bxor_abs(opcode, opcode2),
                    0x7580 => return self.bixor_abs(opcode, opcode2),
                    0x7600 => return self.band_abs(opcode, opcode2),
                    0x7680 => return self.biand_abs(opcode, opcode2),
                    0x7700 => return self.bld_abs(opcode, opcode2),
                    0x7780 => return self.bild_abs(opcode, opcode2),
                    _ => unimpl!(opcode, self.pc),
                }
            }

            0x7f => {
                let opcode2 = self.fetch();
                match opcode2 & 0xff80 {
                    0x6000 | 0x6080 | 0x7000 => return self.bset_abs(opcode, opcode2),
                    0x6100 | 0x6180 | 0x7100 => return self.bnot_abs(opcode, opcode2),
                    0x6200 | 0x6280 | 0x7200 => return self.bclr_abs(opcode, opcode2),
                    0x6700 => return self.bst_abs(opcode, opcode2),
                    0x6780 => return self.bist_abs(opcode, opcode2),
                    _ => unimpl!(opcode, self.pc),
                }
            }

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
                0x00..=0x0f => return self.rotxl_b(opcode),
                0x10..=0x1f => return self.rotxl_w(opcode),
                0x30..=0x37 => return self.rotxl_l(opcode),
                0x80..=0x8f => return self.rotl_b(opcode),
                0x90..=0x9f => return self.rotl_w(opcode),
                0xb0..=0xb7 => return self.rotl_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x13 => match opcode as u8 {
                0x00..=0x0f => return self.rotxr_b(opcode),
                0x10..=0x1f => return self.rotxr_w(opcode),
                0x30..=0x37 => return self.rotxr_l(opcode),
                0x80..=0x8f => return self.rotr_b(opcode),
                0x90..=0x9f => return self.rotr_w(opcode),
                0xb0..=0xb7 => return self.rotr_l(opcode),
                _ => unimpl!(opcode, self.pc),
            },

            0x17 => match opcode as u8 {
                0x50..=0x5f => return self.extu_w(opcode),
                0x70..=0x77 => return self.extu_l(opcode),
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

            0xd0..=0xdf => return self.xor_b_imm(opcode),
            0x15 => return self.xor_b_rn(opcode),
            0x65 => return self.xor_w_rn(opcode),

            0xe0..=0xef => return self.and_b_imm(opcode),
            0x16 => return self.and_b_rn(opcode),
            0x66 => return self.and_w_rn(opcode),

            0x90..=0x9f => return self.addx_imm(opcode),
            0x0e => return self.addx_rn(opcode),

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

    pub fn read_pc(&self) -> u32 {
        self.pc
    }

    fn print_er(&self) {
        for i in 0..8 {
            print!("ER{}:[{:x}] ", i, self.er[i]);
        }
        println!("");
    }
}

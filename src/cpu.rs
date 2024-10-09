use crate::{
    bus::{Bus, AREA0_START_ADDR, AREA7_END_ADDR},
    elf::PROGRAM_START_ADDR,
    memory::{MEMORY_END_ADDR, MEMORY_START_ADDR},
    registers::{ABWCR, ASTCR, DRCRA, WCRH, WCRL},
    setting, socket,
};
use anyhow::{bail, Context as _, Result};
use std::time;
use std::time::Duration;

mod addressing_mode;
mod instruction;

#[cfg(test)]
mod testhelper;

const CPU_CLOCK: usize = 20_000_000;

#[derive(Clone)]
pub struct Cpu {
    pub bus: Bus,
    pc: u32,
    operating_pc: u32,
    ccr: u8,
    pub er: [u32; 8],
    pub exit_addr: u32, // address of ___exit
}

#[allow(dead_code)]
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

#[derive(PartialEq)]
pub enum StateType {
    I,
    J,
    K,
    L,
    M,
    N,
}

macro_rules! unimpl {
    ($op:expr, $pc:expr ) => {
        bail!(
            "unimplemented instruction:[{:>04x}] pc:[{:x}({:x})]",
            $op,
            $pc - 2,
            $pc - 2 - PROGRAM_START_ADDR as u32
        )
    };
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            pc: 0,
            operating_pc: 0,
            ccr: 0,
            er: [0; 8],
            exit_addr: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut state_sum: usize = 0;
        let exec_time = time::Instant::now();

        let mut loop_time = time::Instant::now();
        let mut loop_count: usize = 0;
        let mut one_sec_count: usize = 0;

        let mut is_paused = false;

        // Set program counter
        self.pc = self.er[2];

        self.init_registers()?;

        loop {
            // Print received messages
            if let Some(msgs) = socket::get_received_msgs() {
                for val in msgs {
                    let ls = val.lines();
                    for l in ls {
                        println!("rec: {}", l);
                        if l == "pause" {
                            is_paused = true;
                        } else if l == "restart" {
                            is_paused = false;
                            loop_time = time::Instant::now();
                        } else if l == "stop" {
                            println!("Stopped by message.");
                            return Ok(());
                        }
                    }
                }
            }

            if is_paused {
                continue;
            }

            if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
                print!(" {:4x}:   ", self.pc.wrapping_sub(PROGRAM_START_ADDR as u32));
            }

            let opcode = self.fetch();
            let state = self.exec(opcode).with_context(|| {
                format!(
                    "[pc: {:0>8x}({:0>8x})] opcode1 [{:0>4x}]",
                    self.pc - 2,
                    self.pc - 2 - PROGRAM_START_ADDR as u32,
                    opcode
                )
            })? * 3; // Temporary speed adjustment
            state_sum += state as usize;
            loop_count += state as usize;
            one_sec_count += state as usize;

            if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
                println!("");
            }

            if self.pc == self.exit_addr {
                self.print_er();
                println!("state: {}, time: {}sec", state_sum, exec_time.elapsed().as_secs_f64());
                return Ok(());
            }

            // sleep every 1msec (Windows timer max precision)
            if loop_count >= 20000 {
                spin_sleep_tokio::sleep(Duration::from_secs_f64(loop_count as f64 / CPU_CLOCK as f64).saturating_sub(loop_time.elapsed()))
                    .await;
                loop_count = 0;
                loop_time = time::Instant::now();
            }

            if one_sec_count >= CPU_CLOCK {
                socket::send_one_sec_message();
                one_sec_count -= CPU_CLOCK;
            }
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let _pc = self.pc & !1;

        self.operating_pc = _pc;

        let opcode = { ((self.bus.read(_pc).unwrap() as u16) << 8) | (self.bus.read(_pc + 1).unwrap() as u16) };

        if *setting::ENABLE_PRINT_OPCODE.read().unwrap() {
            print!("{:0>2x} {:0>2x} ", (opcode >> 8) as u8, opcode as u8);
        }

        self.pc += 2;
        opcode
    }

    fn exec(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x0c | 0xf0..=0xff | 0x68 | 0x6e | 0x6c | 0x20..=0x2f | 0x30..=0x3f | 0x6a => return self.mov_b(opcode),
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
            0x56 => return self.rte(),
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

    fn get_wait_state(&self, area_index: u8) -> Result<u8> {
        match area_index {
            0..=3 => return Ok((self.bus.read(WCRL)? >> (area_index * 2)) & 0x3),
            4..=7 => return Ok((self.bus.read(WCRH)? >> ((area_index - 4) * 2)) & 0x3),
            _ => bail!("Invalid area_index [{}]", area_index),
        }
    }

    pub fn calc_state(&self, state_type: StateType, state: u8) -> Result<u8> {
        if state_type == StateType::L || state_type == StateType::M {
            bail!("StateType L or M must be specified address. Use calc_state_with_addr.")
        }
        self.calc_state_with_addr(state_type, state, self.operating_pc)
    }

    // todo 内蔵周辺モジュール
    pub fn calc_state_with_addr(&self, state_type: StateType, state: u8, target_addr: u32) -> Result<u8> {
        if state_type == StateType::N {
            return Ok(state * 1);
        }
        match target_addr {
            MEMORY_START_ADDR..=MEMORY_END_ADDR => match state_type {
                StateType::N => return Ok(state * 1),
                _ => return Ok(state * 2),
            },
            AREA0_START_ADDR..=AREA7_END_ADDR => {
                let area_index = Bus::get_area_index(target_addr)?;
                if (self.bus.read(ABWCR)? >> area_index) & 1 == 1 {
                    // 8 bit

                    // dram
                    if self.bus.check_dram_area(area_index)? {
                        // as 4state
                        match state_type {
                            StateType::I | StateType::J | StateType::K | StateType::M => {
                                let wait_state: u8 = self.get_wait_state(area_index)?;
                                return Ok(state * (8 + 2 * wait_state));
                            }
                            StateType::L => {
                                let wait_state: u8 = self.get_wait_state(area_index)?;
                                return Ok(state * (4 + wait_state));
                            }
                            StateType::N => return Ok(state * 1),
                        }
                    }

                    if (self.bus.read(ASTCR)? >> area_index) & 1 == 0 {
                        // 2 state
                        match state_type {
                            StateType::I | StateType::J | StateType::K | StateType::M => return Ok(state * 4),
                            StateType::L => return Ok(state * 2),
                            StateType::N => return Ok(state * 1),
                        }
                    } else {
                        // 3 state
                        match state_type {
                            StateType::I | StateType::J | StateType::K | StateType::M => {
                                let wait_state: u8 = self.get_wait_state(area_index)?;
                                return Ok(state * (6 + 2 * wait_state));
                            }
                            StateType::L => {
                                let wait_state: u8 = self.get_wait_state(area_index)?;
                                return Ok(state * (3 + wait_state));
                            }
                            StateType::N => return Ok(state * 1),
                        }
                    }
                } else {
                    // 16 bit

                    // dram
                    if self.bus.check_dram_area(area_index)? {
                        let wait_state: u8 = self.get_wait_state(area_index)?;
                        return Ok(state * (4 + wait_state));
                    }

                    if (self.bus.read(ASTCR)? >> area_index) & 1 == 0 {
                        // 2 state
                        return Ok(state * 2);
                    } else {
                        // 3 state
                        let wait_state: u8 = self.get_wait_state(area_index)?;
                        return Ok(state * (3 + wait_state));
                    }
                }
            }
            _ => bail!("Invalid addr [{}]", target_addr),
        }
    }

    fn init_registers(&mut self) -> Result<()> {
        self.bus.write(ABWCR, 0xff)?;
        self.bus.write(ASTCR, 0xfb)?;
        self.bus.write(WCRH, 0xff)?;
        self.bus.write(WCRL, 0xcf)?;
        self.bus.write(DRCRA, 0xe0)?;

        return Ok(());
    }

    fn print_er(&self) {
        for i in 0..8 {
            print!("ER{}:[{:x}] ", i, self.er[i]);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bus::AREA0_START_ADDR,
        cpu::{Cpu, StateType},
        memory::MEMORY_START_ADDR,
        registers::{ABWCR, ASTCR, WCRH, WCRL},
    };

    #[tokio::test]
    async fn test_get_wait_state_wcrl() {
        let mut cpu = Cpu::new();
        cpu.bus.write(WCRL, 0xff).unwrap();
        assert_eq!(cpu.get_wait_state(0).unwrap(), 3);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 3);
        assert_eq!(cpu.get_wait_state(4).unwrap(), 0);

        cpu.bus.write(WCRL, 0xaa).unwrap();
        assert_eq!(cpu.get_wait_state(0).unwrap(), 2);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 2);
        assert_eq!(cpu.get_wait_state(4).unwrap(), 0);

        cpu.bus.write(WCRL, 0x55).unwrap();
        assert_eq!(cpu.get_wait_state(0).unwrap(), 1);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 1);
        assert_eq!(cpu.get_wait_state(4).unwrap(), 0);

        cpu.bus.write(WCRL, 0).unwrap();
        assert_eq!(cpu.get_wait_state(0).unwrap(), 0);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 0);
        assert_eq!(cpu.get_wait_state(4).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_get_wait_state_wcrh() {
        let mut cpu = Cpu::new();
        cpu.bus.write(WCRH, 0xff).unwrap();
        assert_eq!(cpu.get_wait_state(4).unwrap(), 3);
        assert_eq!(cpu.get_wait_state(7).unwrap(), 3);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 0);

        cpu.bus.write(WCRH, 0xaa).unwrap();
        assert_eq!(cpu.get_wait_state(4).unwrap(), 2);
        assert_eq!(cpu.get_wait_state(7).unwrap(), 2);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 0);

        cpu.bus.write(WCRH, 0x55).unwrap();
        assert_eq!(cpu.get_wait_state(4).unwrap(), 1);
        assert_eq!(cpu.get_wait_state(7).unwrap(), 1);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 0);

        cpu.bus.write(WCRH, 0).unwrap();
        assert_eq!(cpu.get_wait_state(4).unwrap(), 0);
        assert_eq!(cpu.get_wait_state(7).unwrap(), 0);
        assert_eq!(cpu.get_wait_state(3).unwrap(), 0);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_calc_state_type_l() {
        let cpu = Cpu::new();
        const STATE: u8 = 2;
        cpu.calc_state(StateType::L, STATE).unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_calc_state_type_m() {
        let cpu = Cpu::new();
        const STATE: u8 = 2;
        cpu.calc_state(StateType::M, STATE).unwrap();
    }

    #[tokio::test]
    async fn test_calc_state_memory() {
        let mut cpu = Cpu::new();
        cpu.operating_pc = MEMORY_START_ADDR;
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state(StateType::I, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::J, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::K, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::N, STATE).unwrap(), 1 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_external_8bit_2state() {
        let mut cpu = Cpu::new();
        cpu.operating_pc = AREA0_START_ADDR;
        cpu.bus.write(ABWCR, 0x01).unwrap();
        cpu.bus.write(ASTCR, 0xfe).unwrap();
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state(StateType::I, STATE).unwrap(), 4 * STATE);
        assert_eq!(cpu.calc_state(StateType::J, STATE).unwrap(), 4 * STATE);
        assert_eq!(cpu.calc_state(StateType::K, STATE).unwrap(), 4 * STATE);
        assert_eq!(cpu.calc_state(StateType::N, STATE).unwrap(), 1 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_external_8bit_3state() {
        let mut cpu = Cpu::new();
        cpu.operating_pc = AREA0_START_ADDR;
        cpu.bus.write(ABWCR, 0x01).unwrap();
        cpu.bus.write(ASTCR, 0x01).unwrap();
        cpu.bus.write(WCRL, 0x03).unwrap();
        const STATE: u8 = 2;
        const WAIT_STATE: u8 = 3;
        assert_eq!(cpu.calc_state(StateType::I, STATE).unwrap(), (6 + 2 * WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::J, STATE).unwrap(), (6 + 2 * WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::K, STATE).unwrap(), (6 + 2 * WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::N, STATE).unwrap(), 1 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_external_16bit_2state() {
        let mut cpu = Cpu::new();
        cpu.operating_pc = AREA0_START_ADDR;
        cpu.bus.write(ABWCR, 0xfe).unwrap();
        cpu.bus.write(ASTCR, 0xfe).unwrap();
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state(StateType::I, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::J, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::K, STATE).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state(StateType::N, STATE).unwrap(), 1 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_external_16bit_3state() {
        let mut cpu = Cpu::new();
        cpu.operating_pc = AREA0_START_ADDR;
        cpu.bus.write(ABWCR, 0xfe).unwrap();
        cpu.bus.write(ASTCR, 0x01).unwrap();
        cpu.bus.write(WCRL, 0x03).unwrap();
        const STATE: u8 = 2;
        const WAIT_STATE: u8 = 3;
        assert_eq!(cpu.calc_state(StateType::I, STATE).unwrap(), (3 + WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::J, STATE).unwrap(), (3 + WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::K, STATE).unwrap(), (3 + WAIT_STATE) * STATE);
        assert_eq!(cpu.calc_state(StateType::N, STATE).unwrap(), 1 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_with_addr_memory() {
        let cpu = Cpu::new();
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state_with_addr(StateType::L, STATE, MEMORY_START_ADDR).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state_with_addr(StateType::M, STATE, MEMORY_START_ADDR).unwrap(), 2 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_with_addr_external_8bit_2state() {
        let mut cpu = Cpu::new();
        cpu.bus.write(ABWCR, 0x01).unwrap();
        cpu.bus.write(ASTCR, 0xfe).unwrap();
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state_with_addr(StateType::L, STATE, AREA0_START_ADDR).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state_with_addr(StateType::M, STATE, AREA0_START_ADDR).unwrap(), 4 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_with_addr_external_8bit_3state() {
        let mut cpu = Cpu::new();
        cpu.bus.write(ABWCR, 0x01).unwrap();
        cpu.bus.write(ASTCR, 0x01).unwrap();
        cpu.bus.write(WCRL, 0x03).unwrap();
        const STATE: u8 = 2;
        const WAIT_STATE: u8 = 3;
        assert_eq!(
            cpu.calc_state_with_addr(StateType::L, STATE, AREA0_START_ADDR).unwrap(),
            (3 + WAIT_STATE) * STATE
        );
        assert_eq!(
            cpu.calc_state_with_addr(StateType::M, STATE, AREA0_START_ADDR).unwrap(),
            (6 + 2 * WAIT_STATE) * STATE
        );
    }

    #[tokio::test]
    async fn test_calc_state_with_addr_external_16bit_2state() {
        let mut cpu = Cpu::new();
        cpu.bus.write(ABWCR, 0xfe).unwrap();
        cpu.bus.write(ASTCR, 0xfe).unwrap();
        const STATE: u8 = 2;
        assert_eq!(cpu.calc_state_with_addr(StateType::L, STATE, AREA0_START_ADDR).unwrap(), 2 * STATE);
        assert_eq!(cpu.calc_state_with_addr(StateType::M, STATE, AREA0_START_ADDR).unwrap(), 2 * STATE);
    }

    #[tokio::test]
    async fn test_calc_state_with_addr_external_16bit_3state() {
        let mut cpu = Cpu::new();
        cpu.bus.write(ABWCR, 0xfe).unwrap();
        cpu.bus.write(ASTCR, 0x01).unwrap();
        cpu.bus.write(WCRL, 0x03).unwrap();
        const STATE: u8 = 2;
        const WAIT_STATE: u8 = 3;
        assert_eq!(
            cpu.calc_state_with_addr(StateType::L, STATE, AREA0_START_ADDR).unwrap(),
            (3 + WAIT_STATE) * STATE
        );
        assert_eq!(
            cpu.calc_state_with_addr(StateType::M, STATE, AREA0_START_ADDR).unwrap(),
            (3 + WAIT_STATE) * STATE
        );
    }
}

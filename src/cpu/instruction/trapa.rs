use crate::cpu::{Cpu, StateType, ADDRESS_MASK};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn trapa(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = self.read_rn_l(7)? & ADDRESS_MASK;
        let imm = Cpu::get_nibble_opcode(opcode, 3)?;

        if imm == 0 {
            self.trapa_emulate_mes2()?;
        } else {
            self.write_dec_ern_l(7, ((self.ccr as u32) << 24) | self.pc)?;
            let vec_addr: u32 = (0x20 + 4 * imm).into();
            let dest_addr = self.read_abs24_l(vec_addr)?;
            self.pc = dest_addr & ADDRESS_MASK;
            self.write_ccr(crate::cpu::CCR::I, 1);
            // TODO: set CCR::UI to 1 if it is used as interrupt mask bit.
        }

        // TODO: calc J
        Ok(self.calc_state(StateType::I, 2)?
            + self.calc_state_with_addr(StateType::K, 2, access_addr)?
            + self.calc_state(StateType::N, 4)?)
    }

    fn trapa_emulate_mes2(&mut self) -> Result<()> {
        let id = self.read_rn_l(0)?;
        match id {
            113 => {
                // set_handler
                let arg1 = self.read_rn_l(1)?;
                let arg2 = self.read_rn_l(2)?;

                if arg1 < 1 || arg1 >= 64 {
                    return Ok(());
                }

                let inst = arg2 + 0x5a000000;
                self.write_abs24_l(arg1 * 4, inst)?;
            }
            104 => {
                // __write
                let arg_addr = self.read_rn_l(1)?;
                let arg0 = self.read_abs24_l(arg_addr)?;
                let arg1 = self.read_abs24_l(arg_addr + 4)?;
                let arg2 = self.read_abs24_l(arg_addr + 8)?;

                let mut chars = Vec::<u8>::new();
                for i in 0..arg2 {
                    let char_addr = arg1 + i;
                    let char = self.read_abs24_b(char_addr)?;
                    chars.push(char);
                }
                let print_string = String::from_utf8(chars)?;

                // Print strings
                println!("[program] [__write] [fd: {}] {}", arg0, print_string);
            }
            _ => bail!("unsupported mes2 command id:{}", id),
        }
        Ok(())
    }
}

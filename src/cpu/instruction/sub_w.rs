use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub fn sub_w_calc(&mut self, dest: u16, src: u16) -> u16 {
        let (result, overflowed) = (dest as i16).overflowing_sub(src as i16);

        self.change_ccr(CCR::H, (dest & 0x0fff) < (src & 0x0fff));
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, dest < src);

        result as u16
    }

    pub(in super::super) fn sub_w(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x79 => return self.sub_w_imm(opcode),
            0x19 => return self.sub_w_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn sub_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = self.fetch();
        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_w(register)?;
            let result = self.sub_w_calc(dest, imm);
            self.write_rn_w(register, result)?;
            Ok(())
        };
        f().with_context(|| format!("imm(opcode2) [{:x}]", imm))?;
        Ok(self.calc_state(StateType::I, 2)?)
    }

    fn sub_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_w(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let src = self.read_rn_w(register_src)?;
        let result = self.sub_w_calc(dest, src);
        self.write_rn_w(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
}

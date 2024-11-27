use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn btst_imm_rn(&mut self, opcode: u16) -> Result<u8> {
        let rd = {
            let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
            self.read_rn_b(rd_i)?
        };
        let imm = Cpu::get_nibble_opcode(opcode, 3)?;
        self.change_ccr(CCR::Z, (rd >> imm) & 1 == 0);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn btst_imm_ern(&mut self, opcode1: u16, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode1, 3)?;
        let erd = self.read_ern_b(erd_i)?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)?;
        self.change_ccr(CCR::Z, (erd >> imm) & 1 == 0);

        let addr = self.get_addr_ern(erd_i)?;
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, addr)?)
    }

    pub(in super::super) fn btst_imm_abs(&mut self, opcode1: u16, opcode2: u16) -> Result<u8> {
        let abs8_addr = opcode1 as u8;
        let value = self.read_abs8_b(abs8_addr)?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)?;
        self.change_ccr(CCR::Z, (value >> imm) & 1 == 0);

        let addr = self.get_addr_abs8(abs8_addr);
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, addr)?)
    }

    pub(in super::super) fn btst_rn_rn(&mut self, opcode: u16) -> Result<u8> {
        let rn = {
            let rn_i = Cpu::get_nibble_opcode(opcode, 3)?;
            self.read_rn_b(rn_i)?
        };
        let rd = {
            let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
            self.read_rn_b(rd_i)?
        };
        self.change_ccr(CCR::Z, (rd >> rn) & 1 == 0);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn btst_rn_ern(&mut self, opcode1: u16, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode1, 3)?;
        let erd = self.read_ern_b(erd_i)?;
        let rn = {
            let rn_i = Cpu::get_nibble_opcode(opcode2, 3)?;
            self.read_rn_b(rn_i)?
        };
        self.change_ccr(CCR::Z, (erd >> rn) & 1 == 0);

        let addr = self.get_addr_ern(erd_i)?;
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, addr)?)
    }

    pub(in super::super) fn btst_rn_abs(&mut self, opcode1: u16, opcode2: u16) -> Result<u8> {
        let abs8_addr = opcode1 as u8;
        let value = self.read_abs8_b(abs8_addr)?;
        let rn = {
            let rn_i = Cpu::get_nibble_opcode(opcode2, 3)?;
            self.read_rn_b(rn_i)?
        };
        self.change_ccr(CCR::Z, (value >> rn) & 1 == 0);

        let addr = self.get_addr_abs8(abs8_addr);
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, addr)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::testhelper::{Abs8Mode, ErnMode, ImmMode, RnMode, TestHelper};

    #[test]
    fn test_btst_imm_rn() {
        TestHelper::build(ImmMode::new(vec![0, 7]), RnMode::new()).run(|_operator, imm, src_i| {
            let operator = _operator.set_opcode(&[0x73, (imm << 4) | src_i]).should_state(2);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 1).unwrap();
                })
                .should_ccr_z(imm != 0)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0x80).unwrap();
                })
                .should_ccr_z(imm != 7)
                .exec(|_| true);
        })
    }

    #[test]
    fn test_btst_imm_ern() {
        TestHelper::build(ImmMode::new(vec![0, 7]), ErnMode::new()).run(|_operator, imm, src_i| {
            let operator = _operator.set_opcode(&[0x7c, src_i << 4, 0x73, imm << 4]).should_state(6);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_ern_b(src_i, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_ern_b(src_i, 1).unwrap();
                })
                .should_ccr_z(imm != 0)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_ern_b(src_i, 0x80).unwrap();
                })
                .should_ccr_z(imm != 7)
                .exec(|_| true);
        })
    }

    #[test]
    fn test_btst_imm_abs() {
        TestHelper::build(ImmMode::new(vec![0, 7]), Abs8Mode::new()).run(|_operator, imm, abs| {
            let operator = _operator.set_opcode(&[0x7e, abs, 0x73, imm << 4]).should_state(6);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_abs8_b(abs, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_abs8_b(abs, 1).unwrap();
                })
                .should_ccr_z(imm != 0)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_abs8_b(abs, 0x80).unwrap();
                })
                .should_ccr_z(imm != 7)
                .exec(|_| true);
        })
    }

    #[test]
    fn test_btst_rn_rn() {
        TestHelper::build(RnMode::new(), RnMode::new()).run(|_operator, rn_i, src_i| {
            let operator = _operator
                .set_opcode(&[0x63, (rn_i << 4) | src_i])
                .should_state(2)
                .ignore(rn_i == src_i);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_b(src_i, 1).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_b(src_i, 0x80).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_b(src_i, 1).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_b(src_i, 0x80).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
        })
    }

    #[test]
    fn test_btst_rn_ern() {
        TestHelper::build(RnMode::new(), ErnMode::new()).run(|_operator, rn_i, src_i| {
            let operator = _operator
                .set_opcode(&[0x7c, src_i << 4, 0x63, rn_i << 4])
                .should_state(6)
                .ignore(rn_i % 8 == src_i);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 1).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 0x80).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 1).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                    cpu.write_abs24_b(0xffcf20, 0x80).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
        })
    }

    #[test]
    fn test_btst_rn_abs() {
        TestHelper::build(RnMode::new(), Abs8Mode::new()).run(|_operator, rn_i, abs| {
            let operator = _operator.set_opcode(&[0x7e, abs, 0x63, rn_i << 4]).should_state(6);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_abs8_b(abs, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_abs8_b(abs, 1).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 0).unwrap();
                    cpu.write_abs8_b(abs, 0x80).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_abs8_b(abs, 0).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_abs8_b(abs, 1).unwrap();
                })
                .should_ccr_z(true)
                .exec(|_| true);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(rn_i, 7).unwrap();
                    cpu.write_abs8_b(abs, 0x80).unwrap();
                })
                .should_ccr_z(false)
                .exec(|_| true);
        })
    }
}

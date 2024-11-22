use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    fn neg_b_proc(&mut self, value: u8) -> u8 {
        let (result, _) = 0u8.overflowing_sub(value);

        self.change_ccr(CCR::H, 0 < (value & 0x0f));
        self.change_ccr(CCR::N, (result as i8) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, value == 0x80);
        self.change_ccr(CCR::C, 0 < value);

        result
    }

    pub(in super::super) fn neg_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_b(rd_i)?;

        let result = self.neg_b_proc(rd);
        self.write_rn_b(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }

    fn neg_w_proc(&mut self, value: u16) -> u16 {
        let (result, _) = 0u16.overflowing_sub(value);

        self.change_ccr(CCR::H, 0 < value & 0x0fff);
        self.change_ccr(CCR::N, (result as i16) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, value == 0x8000);
        self.change_ccr(CCR::C, 0 < value);

        result
    }

    pub(in super::super) fn neg_w(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_w(rd_i)?;

        let result = self.neg_w_proc(rd);
        self.write_rn_w(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }

    fn neg_l_proc(&mut self, value: u32) -> u32 {
        let (result, _) = 0u32.overflowing_sub(value);

        self.change_ccr(CCR::H, 0 < value & 0x0fffffff);
        self.change_ccr(CCR::N, (result as i32) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, value == 0x80000000);
        self.change_ccr(CCR::C, 0 < value);

        result
    }

    pub(in super::super) fn neg_l(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_l(rd_i)?;

        let result = self.neg_l_proc(rd);
        self.write_rn_l(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::testhelper::{ErnMode, NoneMode, RnMode, TestHelper};

    #[test]
    fn test_neg_b() {
        TestHelper::build(RnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator.clone().set_opcode(&[0x17, 0x80 | src_i]).should_state(2);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(false)
                .should_ccr_z(true)
                .should_ccr_v(false)
                .should_ccr_c(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, -1i8 as u8).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(false)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 1);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 1).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), -1i8 as u8);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0x80).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(true)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0x80);
                    true
                });
        });
    }

    #[test]
    fn test_neg_w() {
        TestHelper::build(RnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator.clone().set_opcode(&[0x17, 0x90 | src_i]).should_state(2);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(false)
                .should_ccr_z(true)
                .should_ccr_v(false)
                .should_ccr_c(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, -1i16 as u16).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(false)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 1);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 512).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), -512i16 as u16);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0x8000).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(true)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0x8000);
                    true
                });
        });
    }

    #[test]
    fn test_neg_l() {
        TestHelper::build(ErnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator.clone().set_opcode(&[0x17, 0xb0 | src_i]).should_state(2);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(false)
                .should_ccr_z(true)
                .should_ccr_v(false)
                .should_ccr_c(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, -1i32 as u32).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(false)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 1);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 50000).unwrap();
                })
                .should_ccr_h(true)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(false)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), -50000i32 as u32);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0x80000000).unwrap();
                })
                .should_ccr_h(false)
                .should_ccr_n(true)
                .should_ccr_z(false)
                .should_ccr_v(true)
                .should_ccr_c(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0x80000000);
                    true
                });
        });
    }
}

use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn not_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_b(rd_i)?;
        let result = !rd;
        self.write_rn_b(rd_i, result)?;

        self.change_ccr(CCR::N, (result as i8) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, false);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn not_w(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_w(rd_i)?;
        let result = !rd;
        self.write_rn_w(rd_i, result)?;

        self.change_ccr(CCR::N, (result as i16) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, false);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn not_l(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_l(rd_i)?;
        let result = !rd;
        self.write_rn_l(rd_i, result)?;

        self.change_ccr(CCR::N, (result as i32) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, false);

        Ok(self.calc_state(StateType::I, 1)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::testhelper::{ErnMode, NoneMode, RnMode, TestHelper};

    #[test]
    fn test_not_b() {
        TestHelper::build(RnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator.clone().set_opcode(&[0x17, src_i]).should_state(2).should_ccr_v(false);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0xff);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0xff).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0x00);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0x0f).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0xf0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0xf0).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(src_i).unwrap(), 0x0f);
                    true
                });
        });
    }

    #[test]
    fn test_not_w() {
        TestHelper::build(RnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator
                .clone()
                .set_opcode(&[0x17, 0x10 | src_i])
                .should_state(2)
                .should_ccr_v(false);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0xffff);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0xffff).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0x0000);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0x000f).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0xfff0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0xf000).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_w(src_i).unwrap(), 0x0fff);
                    true
                });
        });
    }

    #[test]
    fn test_not_l() {
        TestHelper::build(ErnMode::new(), NoneMode::new()).run(|_operator, src_i, _| {
            let operator = _operator
                .clone()
                .set_opcode(&[0x17, 0x30 | src_i])
                .should_state(2)
                .should_ccr_v(false);

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0xffff_ffff);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0xffff_ffff).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0x0000_000f).unwrap();
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0xffff_fff0);
                    true
                });

            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(src_i, 0xf000_0000).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_l(src_i).unwrap(), 0x0fff_ffff);
                    true
                });
        });
    }
}

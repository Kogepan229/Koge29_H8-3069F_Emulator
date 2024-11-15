use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn mulxu_b(&mut self, opcode: u16) -> Result<u8> {
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rs = self.read_rn_b(rs_i)?;
        let rd = self.read_rn_w(rd_i)? & 0xff;

        let result = rd * u16::from(rs);
        self.write_rn_w(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 12)?)
    }

    pub(in super::super) fn mulxu_w(&mut self, opcode: u16) -> Result<u8> {
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;
        let erd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rs = self.read_rn_w(rs_i)?;
        let erd = self.read_rn_l(erd_i)? & 0xffff;

        let result = erd * u32::from(rs);
        self.write_rn_l(erd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 20)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::testhelper::{ErnMode, RnMode, TestHelper};

    #[test]
    fn test_mulxu_b() {
        TestHelper::build(RnMode::new(), RnMode::new()).run(|operator, src_i, target_i| {
            operator
                .clone()
                .set_opcode(&[0x50, (src_i << 4) | target_i])
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 9).unwrap();
                    cpu.write_rn_w(target_i, 8).unwrap();
                })
                .should_state(14)
                .exec(|cpu| {
                    // Ignore same registers
                    if src_i % 8 != target_i % 8 {
                        assert_eq!(cpu.read_rn_w(target_i).unwrap(), 72);
                    }
                    true
                });
            operator
                .clone()
                .set_opcode(&[0x50, (src_i << 4) | target_i])
                .access_cpu(|cpu| {
                    cpu.write_rn_b(src_i, 0xff).unwrap();
                    cpu.write_rn_w(target_i, 0x11ff).unwrap();
                })
                .should_state(14)
                .exec(|cpu| {
                    // Ignore same registers
                    if src_i % 8 != target_i % 8 {
                        assert_eq!(cpu.read_rn_w(target_i).unwrap(), 0xfe01);
                    }
                    true
                });
        });
    }

    #[test]
    fn test_mulxu_w() {
        TestHelper::build(RnMode::new(), ErnMode::new()).run(|operator, src_i, target_i| {
            operator
                .clone()
                .set_opcode(&[0x52, (src_i << 4) | target_i])
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 9).unwrap();
                    cpu.write_rn_l(target_i, 8).unwrap();
                })
                .should_state(22)
                .exec(|cpu| {
                    // Ignore same registers
                    if src_i % 8 != target_i % 8 {
                        assert_eq!(cpu.read_rn_l(target_i).unwrap(), 72);
                    }
                    true
                });
            operator
                .clone()
                .set_opcode(&[0x52, (src_i << 4) | target_i])
                .access_cpu(|cpu| {
                    cpu.write_rn_w(src_i, 0xffff).unwrap();
                    cpu.write_rn_l(target_i, 0x1234ffff).unwrap();
                })
                .should_state(22)
                .exec(|cpu| {
                    // Ignore same registers
                    if src_i % 8 != target_i % 8 {
                        assert_eq!(cpu.read_rn_l(target_i).unwrap(), 0xfffe0001);
                    }
                    true
                });
        });
    }
}

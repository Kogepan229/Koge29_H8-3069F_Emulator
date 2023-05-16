use super::super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn mov_l(&mut self, opcode: u16) -> usize {
        if opcode & 0xff00 == 0x0f00 {
            return self.mov_l_rn(opcode);
        }
        if opcode & 0xfff8 == 0x7a00 {
            return self.mov_l_imm(opcode);
        }
        let opcode2 = self.fetch();
        match (opcode2 >> 8) as u8 {
            0x69 => return self.mov_l_ern(opcode2),
            0x6f => return self.mov_l_disp16(opcode2),
            0x78 => return self.mov_l_disp24(opcode2),
            0x6d => return self.mov_l_inc(opcode2),
            0x6b => return self.mov_l_abs(opcode2),
            _ => panic!("invalid opcode [{::>04x} {::>04x}]", opcode, opcode2),
        }
    }

    fn mov_l_proc_pcc(&mut self, src: u32) {
        if (src as i32) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if src == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
    }

    fn mov_l_rn(&mut self, opcode: u16) -> usize {
        let value = self
            .read_rn_l(Cpu::get_nibble_opcode(opcode, 3) & 0x07)
            .unwrap();
        self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4), value)
            .unwrap();
        self.mov_l_proc_pcc(value);
        return 2;
    }

    fn mov_l_imm(&mut self, opcode: u16) -> usize {
        let imm = (self.fetch() as u32) << 16 | self.fetch() as u32;
        self.write_rn_l((opcode & 0x000f) as u8, imm).unwrap();
        self.mov_l_proc_pcc(imm);
        return 6;
    }

    fn mov_l_ern(&mut self, opcode2: u16) -> usize {
        if opcode2 & 0x0080 == 0 {
            let value = self.read_ern_l(Cpu::get_nibble_opcode(opcode2, 3)).unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)).unwrap();
            self.write_ern_l(Cpu::get_nibble_opcode(opcode2, 3) & 0x07, value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 8;
    }

    fn mov_l_disp16(&mut self, opcode2: u16) -> usize {
        let disp = self.fetch();
        if opcode2 & 0x0080 == 0 {
            let value = self
                .read_disp16_ern_l(Cpu::get_nibble_opcode(opcode2, 3), disp)
                .unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)).unwrap();
            self.write_disp16_ern_l(Cpu::get_nibble_opcode(opcode2, 3) & 0x07, disp, value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 10;
    }

    fn mov_l_disp24(&mut self, opcode2: u16) -> usize {
        let opcode3 = self.fetch();
        let disp = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        if opcode2 & 0x0080 == 0 {
            let value = self
                .read_disp24_ern_l(Cpu::get_nibble_opcode(opcode2, 3), disp)
                .unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode3, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode3, 4)).unwrap();
            self.write_disp24_ern_l(Cpu::get_nibble_opcode(opcode2, 3) & 0x07, disp, value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 14;
    }

    fn mov_l_inc(&mut self, opcode2: u16) -> usize {
        if opcode2 & 0x0080 == 0 {
            let value = self
                .read_inc_ern_l(Cpu::get_nibble_opcode(opcode2, 3))
                .unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)).unwrap();
            self.write_inc_ern_l(Cpu::get_nibble_opcode(opcode2, 3) & 0x07, value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 10;
    }

    fn mov_l_abs(&mut self, opcode2: u16) -> usize {
        match opcode2 & 0xfff0 {
            0x6b00 | 0x6b80 => return self.mov_l_abs16(opcode2),
            0x6b20 | 0x6ba0 => return self.mov_l_abs24(opcode2),
            _ => panic!("invalid opcode2 [{}]", opcode2),
        }
    }

    fn mov_l_abs16(&mut self, opcode2: u16) -> usize {
        let abs_addr = self.fetch();
        if opcode2 & 0xfff0 == 0x6b00 {
            let value = self.read_abs16_l(abs_addr).unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)).unwrap();
            self.write_abs16_l(abs_addr, value).unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 10;
    }

    fn mov_l_abs24(&mut self, opcode2: u16) -> usize {
        let abs_addr = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        if opcode2 & 0xfff0 == 0x6b20 {
            let value = self.read_abs24_l(abs_addr).unwrap();
            self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4), value)
                .unwrap();
            self.mov_l_proc_pcc(value);
        } else {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)).unwrap();
            self.write_abs24_l(abs_addr, value).unwrap();
            self.mov_l_proc_pcc(value);
        }
        return 12;
    }
}

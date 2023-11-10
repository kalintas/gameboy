#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::{MemoryMap, OamCorruption};
use crate::emulator::Cpu;

/// INC BC - 0x03 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn inc_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.bc(), OamCorruption::Write);
    cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));
    8
}

/// INC B - 0x04 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.inc(cpu.registers.b);
    4
}

/// DEC B - 0x05 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.dec(cpu.registers.b);
    4
}

/// ADD HL,BC - 0x09 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C
pub fn add_hl_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add_u16(cpu.registers.bc());
    8
}

/// DEC BC - 0x0B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn dec_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    memory_map.try_corrupt_oam(cpu.registers.bc(), OamCorruption::Write);
    cpu.registers.set_bc(cpu.registers.bc().wrapping_sub(1));
    8
}

/// INC C - 0x0C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.inc(cpu.registers.c);
    4
}

/// DEC C - 0x0D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.dec(cpu.registers.c);
    4
}

/// INC DE - 0x13 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn inc_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.de(), OamCorruption::Write);
    cpu.registers.set_de(cpu.registers.de().wrapping_add(1));
    8
}

/// INC D - 0x14 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.inc(cpu.registers.d);
    4
}

/// DEC D - 0x15 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.dec(cpu.registers.d);
    4
}

/// ADD HL,DE - 0x19 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C
pub fn add_hl_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add_u16(cpu.registers.de());
    8
}

/// DEC DE - 0x1B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn dec_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.de(), OamCorruption::Write);
    cpu.registers.set_de(cpu.registers.de().wrapping_sub(1));
    8
}

/// INC E - 0x1C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.inc(cpu.registers.e);
    4
}

/// DEC E - 0x1D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.dec(cpu.registers.e);
    4
}

/// INC HL - 0x23 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn inc_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.hl(), OamCorruption::Write);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    8
}

/// INC H - 0x24 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.inc(cpu.registers.h);
    4
}

/// DEC H - 0x25 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.dec(cpu.registers.h);
    4
}

/// DAA - 0x27 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z - 0 C
pub fn daa(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    /*
        DAA is intended to be run immediately after an addition or subtraction operation,
        where the operands were BCD encoded.
        So the result (stored in the A register) is the BCD encoded result of the previous operation.
    */

    let mut cy = 0;

    if cpu.registers.get_n() == 0 {
        // Addition
        if cpu.registers.get_cy() != 0 || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            cy = 1;
        }
        if cpu.registers.get_h() != 0 || (cpu.registers.a & 0xF) > 0x9 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x6);
        }
    } else {
        // Subtraction
        if cpu.registers.get_cy() != 0 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x60);
            cy = 1;
        }

        if cpu.registers.get_h() != 0 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x6);
        }
    }

    cpu.registers
        .set_flags((cpu.registers.a == 0) as u8, cpu.registers.get_n(), 0, cy);

    4
}

/// ADD HL,HL - 0x29 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C
pub fn add_hl_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add_u16(cpu.registers.hl());
    8
}

/// DEC HL - 0x2B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn dec_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.hl(), OamCorruption::Write);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    8
}

/// INC L - 0x2C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.inc(cpu.registers.l);
    4
}

/// DEC L - 0x2D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.dec(cpu.registers.l);
    4
}

/// CPL - 0x2F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 1 1 -
pub fn cpl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Complement A register. (Flip all bits.)
    cpu.registers.a = !cpu.registers.a;
    cpu.registers
        .set_flags(cpu.registers.get_z(), 1, 1, cpu.registers.get_cy());
    4
}

/// INC SP - 0x33 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn inc_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.sp, OamCorruption::Write);
    cpu.sp = cpu.sp.wrapping_add(1);
    8
}

/// INC (HL) - 0x34 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z 0 H -
pub fn inc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.hl(), OamCorruption::Write);
    memory_map.cpu_set(
        cpu.registers.hl(),
        cpu.inc(memory_map.cpu_get(cpu.registers.hl())),
    );
    12
}

/// DEC (HL) - 0x35 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z 1 H -
pub fn dec_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.registers.hl(), OamCorruption::Write);
    memory_map.cpu_set(
        cpu.registers.hl(),
        cpu.dec(memory_map.cpu_get(cpu.registers.hl())),
    );
    12
}

/// SCF - 0x37 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 0 0 1
pub fn scf(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Set Carry flag.
    cpu.registers.set_flags(cpu.registers.get_z(), 0, 0, 1);
    4
}

/// ADD HL,SP - 0x39 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C
pub fn add_hl_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add_u16(cpu.sp);
    8
}

/// DEC SP - 0x3B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn dec_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.try_corrupt_oam(cpu.sp, OamCorruption::Write);
    cpu.sp = cpu.sp.wrapping_sub(1);
    8
}

/// INC A - 0x3C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H -
pub fn inc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.inc(cpu.registers.a);
    4
}

/// DEC A - 0x3D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H -
pub fn dec_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.dec(cpu.registers.a);
    4
}

/// CCF - 0x3F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 0 0 C
pub fn ccf(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Complement carry flag.
    cpu.registers.set_flags(
        cpu.registers.get_z(),
        0,
        0,
        (cpu.registers.get_cy() == 0) as u8,
    );
    4
}

/// ADD A,B - 0x80 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.b);
    4
}

/// ADD A,C - 0x81 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.c);
    4
}

/// ADD A,D - 0x82 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.d);
    4
}

/// ADD A,E - 0x83 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.e);
    4
}

/// ADD A,H - 0x84 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.h);
    4
}

/// ADD A,L - 0x85 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.l);
    4
}

/// ADD A,(HL) - 0x86 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C
pub fn add_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// ADD A,A - 0x87 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn add_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.registers.a);
    4
}

/// ADC A,B - 0x88 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.b);
    4
}

/// ADC A,C - 0x89 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.c);
    4
}

/// ADC A,D - 0x8A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.d);
    4
}

/// ADC A,E - 0x8B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.e);
    4
}

/// ADC A,H - 0x8C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.h);
    4
}

/// ADC A,L - 0x8D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.l);
    4
}

/// ADC A,(HL) - 0x8E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// ADC A,A - 0x8F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.registers.a);
    4
}

/// SUB B - 0x90 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.b);
    4
}

/// SUB C - 0x91 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.c);
    4
}

/// SUB D - 0x92 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.d);
    4
}

/// SUB E - 0x93 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.e);
    4
}

/// SUB H - 0x94 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.h);
    4
}

/// SUB L - 0x95 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.l);
    4
}

/// SUB (HL) - 0x96 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn sub_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// SUB A - 0x97 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sub_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.registers.a);
    4
}

/// SBC A,B - 0x98 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.b);
    4
}

/// SBC A,C - 0x99 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.c);
    4
}

/// SBC A,D - 0x9A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.d);
    4
}

/// SBC A,E - 0x9B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.e);
    4
}

/// SBC A,H - 0x9C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.h);
    4
}

/// SBC A,L - 0x9D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.l);
    4
}

/// SBC A,(HL) - 0x9E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// SBC A,A - 0x9F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.registers.a);
    4
}

/// AND B - 0xA0 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.b);
    4
}

/// AND C - 0xA1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.c);
    4
}

/// AND D - 0xA2 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.d);
    4
}

/// AND E - 0xA3 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.e);
    4
}

/// AND H - 0xA4 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.h);
    4
}

/// AND L - 0xA5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.l);
    4
}

/// AND (HL) - 0xA6 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 0
pub fn and_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// AND A - 0xA7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0
pub fn and_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.registers.a);
    4
}

/// XOR B - 0xA8 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.b);
    4
}

/// XOR C - 0xA9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.c);
    4
}

/// XOR D - 0xAA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.d);
    4
}

/// XOR E - 0xAB <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.e);
    4
}

/// XOR H - 0xAC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.h);
    4
}

/// XOR L - 0xAD <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.l);
    4
}

/// XOR (HL) - 0xAE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn xor_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// XOR A - 0xAF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn xor_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.registers.a);
    4
}

/// OR B - 0xB0 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.b);
    4
}

/// OR C - 0xB1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.c);
    4
}

/// OR D - 0xB2 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.d);
    4
}

/// OR E - 0xB3 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.e);
    4
}

/// OR H - 0xB4 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.h);
    4
}

/// OR L - 0xB5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.l);
    4
}

/// OR (HL) - 0xB6 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn or_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(memory_map.cpu_get(cpu.registers.hl()));
    8
}

/// OR A - 0xB7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0
pub fn or_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.registers.a);
    4
}

/// CP B - 0xB8 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.b);
    cpu.registers.a = old_a;
    4
}

/// CP C - 0xB9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.c);
    cpu.registers.a = old_a;
    4
}

/// CP D - 0xBA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.d);
    cpu.registers.a = old_a;
    4
}

/// CP E - 0xBB <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.e);
    cpu.registers.a = old_a;
    4
}

/// CP H - 0xBC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.h);
    cpu.registers.a = old_a;
    4
}

/// CP L - 0xBD <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.l);
    cpu.registers.a = old_a;
    4
}

/// CP (HL) - 0xBE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn cp_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(memory_map.cpu_get(cpu.registers.hl()));
    cpu.registers.a = old_a;
    8
}

/// CP A - 0xBF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C
pub fn cp_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.registers.a);
    cpu.registers.a = old_a;
    4
}

/// ADD A,d8 - 0xC6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C
pub fn add_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.add(cpu.get_immediate_u8(memory_map));
    8
}

/// ADC A,d8 - 0xCE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C
pub fn adc_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.adc(cpu.get_immediate_u8(memory_map));
    8
}

/// SUB d8 - 0xD6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn sub_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sub(cpu.get_immediate_u8(memory_map));
    8
}

/// SBC A,d8 - 0xDE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn sbc_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sbc(cpu.get_immediate_u8(memory_map));
    8
}

/// AND d8 - 0xE6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 0
pub fn and_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.and(cpu.get_immediate_u8(memory_map));
    8
}

/// ADD SP,r8 - 0xE8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: 0 0 H C
pub fn add_sp_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8

    let val = cpu.get_immediate_u8(memory_map);

    let half_carry = ((cpu.sp & 0xF) + ((val as u16) & 0xF)) > 0xF;
    let carry = ((cpu.sp & 0xFF) + ((val as u16) & 0xFF)) > 0xFF;

    cpu.sp = (cpu.sp as i32 + (val as i8) as i32) as u16;

    cpu.registers.set_flags(0, 0, half_carry as u8, carry as u8);

    16
}

/// XOR d8 - 0xEE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn xor_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.xor(cpu.get_immediate_u8(memory_map));
    8
}

/// OR d8 - 0xF6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn or_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.or(cpu.get_immediate_u8(memory_map));
    8
}

/// CP d8 - 0xFE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C
pub fn cp_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    let old_a = cpu.registers.a;
    cpu.sub(cpu.get_immediate_u8(memory_map));
    cpu.registers.a = old_a;
    8
}

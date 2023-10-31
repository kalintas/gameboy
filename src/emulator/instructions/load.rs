#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::MemoryMap;
use crate::emulator::Cpu;

/// LD BC,d16 - 0x01 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_bc_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_bc(cpu.get_immediate_u16(memory_map));
    12
}

/// LD (BC),A - 0x02 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_bc_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.bc(), cpu.registers.a);
    8
}

/// LD B,d8 - 0x06 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_b_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.get_immediate_u8(memory_map);
    8
}

/// LD (a16),SP - 0x08 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 20 <br>
///  Flags affected: - - - -
pub fn ld_a16_addr_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set_u16(cpu.get_immediate_u16(memory_map), cpu.sp);
    20
}

/// LD A,(BC) - 0x0A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_bc_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.registers.bc());
    8
}

/// LD C,d8 - 0x0E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.get_immediate_u8(memory_map);
    8
}

/// LD DE,d16 - 0x11 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_de_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_de(cpu.get_immediate_u16(memory_map));
    12
}

/// LD (DE),A - 0x12 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_de_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.de(), cpu.registers.a);
    8
}

/// LD D,d8 - 0x16 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_d_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.get_immediate_u8(memory_map);
    8
}

/// LD A,(DE) - 0x1A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_de_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.registers.de());
    8
}

/// LD E,d8 - 0x1E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_e_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.get_immediate_u8(memory_map);
    8
}

/// LD HL,d16 - 0x21 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_hl_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_hl(cpu.get_immediate_u16(memory_map));
    12
}

/// LD (HL+),A - 0x22 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_plus_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    8
}

/// LD H,d8 - 0x26 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_h_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.get_immediate_u8(memory_map);
    8
}

/// LD A,(HL+) - 0x2A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_plus_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    8
}

/// LD L,d8 - 0x2E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_l_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.get_immediate_u8(memory_map);
    8
}

/// LD SP,d16 - 0x31 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_sp_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp = cpu.get_immediate_u16(memory_map);
    12
}

/// LD (HL-),A - 0x32 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_minus_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    8
}

/// LD (HL),d8 - 0x36 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.get_immediate_u8(memory_map));
    12
}

/// LD A,(HL-) - 0x3A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_minus_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    8
}

/// LD A,d8 - 0x3E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.get_immediate_u8(memory_map);
    8
}

/// LD B,B - 0x40 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.b;
    4
}

/// LD B,C - 0x41 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.c;
    4
}

/// LD B,D - 0x42 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.d;
    4
}

/// LD B,E - 0x43 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.e;
    4
}

/// LD B,H - 0x44 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.h;
    4
}

/// LD B,L - 0x45 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.l;
    4
}

/// LD B,(HL) - 0x46 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_b_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = memory_map.get(cpu.registers.hl());
    8
}

/// LD B,A - 0x47 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.registers.a;
    4
}

/// LD C,B - 0x48 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.b;
    4
}

/// LD C,C - 0x49 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.c;
    4
}

/// LD C,D - 0x4A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.d;
    4
}

/// LD C,E - 0x4B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.e;
    4
}

/// LD C,H - 0x4C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.h;
    4
}

/// LD C,L - 0x4D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.l;
    4
}

/// LD C,(HL) - 0x4E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = memory_map.get(cpu.registers.hl());
    8
}

/// LD C,A - 0x4F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.registers.a;
    4
}

/// LD D,B - 0x50 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.b;
    4
}

/// LD D,C - 0x51 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.c;
    4
}

/// LD D,D - 0x52 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.d;
    4
}

/// LD D,E - 0x53 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.e;
    4
}

/// LD D,H - 0x54 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.h;
    4
}

/// LD D,L - 0x55 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.l;
    4
}

/// LD D,(HL) - 0x56 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_d_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = memory_map.get(cpu.registers.hl());
    8
}

/// LD D,A - 0x57 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.registers.a;
    4
}

/// LD E,B - 0x58 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.b;
    4
}

/// LD E,C - 0x59 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.c;
    4
}

/// LD E,D - 0x5A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.d;
    4
}

/// LD E,E - 0x5B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.e;
    4
}

/// LD E,H - 0x5C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.h;
    4
}

/// LD E,L - 0x5D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.l;
    4
}

/// LD E,(HL) - 0x5E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_e_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = memory_map.get(cpu.registers.hl());
    8
}

/// LD E,A - 0x5F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.registers.a;
    4
}

/// LD H,B - 0x60 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.b;
    4
}

/// LD H,C - 0x61 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.c;
    4
}

/// LD H,D - 0x62 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.d;
    4
}

/// LD H,E - 0x63 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.e;
    4
}

/// LD H,H - 0x64 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.h;
    4
}

/// LD H,L - 0x65 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.l;
    4
}

/// LD H,(HL) - 0x66 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_h_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = memory_map.get(cpu.registers.hl());
    8
}

/// LD H,A - 0x67 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.registers.a;
    4
}

/// LD L,B - 0x68 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.b;
    4
}

/// LD L,C - 0x69 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.c;
    4
}

/// LD L,D - 0x6A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.d;
    4
}

/// LD L,E - 0x6B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.e;
    4
}

/// LD L,H - 0x6C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.h;
    4
}

/// LD L,L - 0x6D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.l;
    4
}

/// LD L,(HL) - 0x6E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_l_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = memory_map.get(cpu.registers.hl());
    8
}

/// LD L,A - 0x6F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.registers.a;
    4
}

/// LD (HL),B - 0x70 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.b);
    8
}

/// LD (HL),C - 0x71 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.c);
    8
}

/// LD (HL),D - 0x72 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.d);
    8
}

/// LD (HL),E - 0x73 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.e);
    8
}

/// LD (HL),H - 0x74 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.h);
    8
}

/// LD (HL),L - 0x75 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.l);
    8
}

/// LD (HL),A - 0x77 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.registers.hl(), cpu.registers.a);
    8
}

/// LD A,B - 0x78 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.b;
    4
}

/// LD A,C - 0x79 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.c;
    4
}

/// LD A,D - 0x7A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.d;
    4
}

/// LD A,E - 0x7B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.e;
    4
}

/// LD A,H - 0x7C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.h;
    4
}

/// LD A,L - 0x7D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.l;
    4
}

/// LD A,(HL) - 0x7E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.registers.hl());
    8
}

/// LD A,A - 0x7F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.registers.a;
    4
}

/// POP BC - 0xC1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_bc(memory_map.get_u16(cpu.sp));
    cpu.sp += 2;
    12
}

/// PUSH BC - 0xC5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp -= 2;
    memory_map.set_u16(cpu.sp, cpu.registers.bc());
    16
}

/// POP DE - 0xD1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_de(memory_map.get_u16(cpu.sp));
    cpu.sp += 2;
    12
}

/// PUSH DE - 0xD5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp -= 2;
    memory_map.set_u16(cpu.sp, cpu.registers.de());
    16
}

/// LDH (a8),A - 0xE0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ldh_a8_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        0xFF00 + cpu.get_immediate_u8(memory_map) as u16,
        cpu.registers.a,
    );
    12
}

/// POP HL - 0xE1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_hl(memory_map.get_u16(cpu.sp));
    cpu.sp += 2;
    12
}

/// LD (C),A - 0xE2 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(0xFF00 + cpu.registers.c as u16, cpu.registers.a);
    8
}

/// PUSH HL - 0xE5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp -= 2;
    memory_map.set_u16(cpu.sp, cpu.registers.hl());
    16
}

/// LD (a16),A - 0xEA <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ld_a16_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(cpu.get_immediate_u16(memory_map), cpu.registers.a);
    16
}

/// LDH A,(a8) - 0xF0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ldh_a_a8_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(0xFF00 + cpu.get_immediate_u8(memory_map) as u16);
    12
}

/// POP AF - 0xF1 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z N H C
pub fn pop_af(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.set_af(memory_map.get_u16(cpu.sp));
    cpu.sp += 2;
    12
}

/// LD A,(C) - 0xF2 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_c_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(0xFF00 + cpu.registers.c as u16);
    8
}

/// PUSH AF - 0xF5 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_af(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp -= 2;
    memory_map.set_u16(cpu.sp, cpu.registers.af());
    16
}

/// LD HL,SP+r8 - 0xF8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: 0 0 H C
pub fn ld_hl_sp_plusr8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // https://www.reddit.com/r/EmuDev/comments/y51i1c/game_boy_dealing_with_carry_flags_when_handling/

    let val = cpu.get_immediate_u8(memory_map);

    let half_carry = ((cpu.sp & 0xF) + ((val as u16) & 0xF)) > 0xF;
    let carry = ((cpu.sp & 0xFF) + ((val as u16) & 0xFF)) > 0xFF;

    cpu.registers
        .set_hl((cpu.sp as i32 + (val as i8) as i32) as u16);

    cpu.registers.set_flags(0, 0, half_carry as u8, carry as u8);

    12
}

/// LD SP,HL - 0xF9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_sp_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.sp = cpu.registers.hl();
    8
}

/// LD A,(a16) - 0xFA <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ld_a_a16_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = memory_map.get(cpu.get_immediate_u16(memory_map));
    16
}

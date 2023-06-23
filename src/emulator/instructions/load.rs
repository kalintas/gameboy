#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::MemoryMap;
use crate::emulator::Cpu;

/// LD BC,d16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_bc_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD (BC),A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_bc_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD B,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_b_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (a16),SP <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 20 <br>
///  Flags affected: - - - -
pub fn ld_a16_addr_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    20
}

/// LD A,(BC) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_bc_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD C,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD DE,d16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_de_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD (DE),A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_de_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD D,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_d_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,(DE) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_de_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD E,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_e_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD HL,d16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_hl_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD (HL+),A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_plus_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD H,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_h_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,(HL+) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_plus_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD L,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_l_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD SP,d16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_sp_d16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD (HL-),A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_minus_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD A,(HL-) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_minus_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD B,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD B,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_b_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD B,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_b_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD C,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD C,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_c_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD D,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_d_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD D,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_d_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD E,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_e_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD E,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_e_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD H,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_h_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD H,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_h_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD L,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_l_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD L,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_l_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD (HL),B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD (HL),A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_hl_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// LD A,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn ld_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// POP BC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// PUSH BC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// POP DE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// PUSH DE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// LDH (a8),A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ldh_a8_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// POP HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn pop_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD (C),A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_c_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// PUSH HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// LD (a16),A <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ld_a16_addr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// LDH A,(a8) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn ldh_a_a8_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// POP AF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z N H C
pub fn pop_af(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD A,(C) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_a_c_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// PUSH AF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn push_af(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// LD HL,SP+r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: 0 0 H C
pub fn ld_hl_sp_plusr8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// LD SP,HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn ld_sp_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// LD A,(a16) <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ld_a_a16_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

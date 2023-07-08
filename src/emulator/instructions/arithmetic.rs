
#![allow(dead_code, unused_variables)]
use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;


/// INC BC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn inc_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD HL,BC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C 
pub fn add_hl_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// DEC BC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn dec_bc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// INC DE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn inc_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD HL,DE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C 
pub fn add_hl_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// DEC DE <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn dec_de(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// INC HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn inc_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DAA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z - 0 C 
pub fn daa(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD HL,HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C 
pub fn add_hl_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// DEC HL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn dec_hl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CPL <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 1 1 - 
pub fn cpl(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// INC SP <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn inc_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z 0 H - 
pub fn inc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    12
}

/// DEC (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: Z 1 H - 
pub fn dec_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    12
}

/// SCF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 0 0 1 
pub fn scf(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD HL,SP <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - 0 H C 
pub fn add_hl_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// DEC SP <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn dec_sp(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// INC A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H - 
pub fn inc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DEC A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H - 
pub fn dec_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CCF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - 0 0 C 
pub fn ccf(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// ADD A,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADC A,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// ADC A,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SUB (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn sub_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SUB A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sub_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// SBC A,(HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SBC A,A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// AND (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 0 
pub fn and_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// AND A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 1 0 
pub fn and_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// XOR (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// XOR A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// OR (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn or_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// OR A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 0 0 0 
pub fn or_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP B <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP D <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP E <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP L <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// CP (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn cp_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// CP A <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: Z 1 H C 
pub fn cp_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// ADD A,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C 
pub fn add_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// ADC A,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 H C 
pub fn adc_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SUB d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn sub_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SBC A,d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn sbc_a_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// AND d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 0 
pub fn and_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// ADD SP,r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: 0 0 H C 
pub fn add_sp_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// XOR d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn xor_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// OR d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn or_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// CP d8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 1 H C 
pub fn cp_d8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

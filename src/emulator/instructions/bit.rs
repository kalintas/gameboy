
#![allow(dead_code, unused_variables)]
use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;


/// RLCA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C 
pub fn rlca(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// RRCA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C 
pub fn rrca(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// RLA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C 
pub fn rla(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// RRA <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C 
pub fn rra(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// RLC B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RLC (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RLC A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rlc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RRC (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RRC A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rrc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RL (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RL A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rl_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RR (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RR A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn rr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SLA (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SLA A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn sla_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRA (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SRA A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn sra_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SWAP (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SWAP A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0 
pub fn swap_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SRL (HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SRL A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C 
pub fn srl_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 0,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 0,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 1,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 1,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 2,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 2,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 3,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 3,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 4,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 4,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 5,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 5,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 6,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 6,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// BIT 7,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// BIT 7,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 - 
pub fn bit_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 0,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 0,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 1,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 1,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 2,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 2,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 3,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 3,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 4,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 4,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 5,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 5,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 6,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 6,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// RES 7,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn res_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// RES 7,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn res_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 0,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 0,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 1,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 1,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 2,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 2,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 3,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 3,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 4,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 4,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 5,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 5,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 6,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 6,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,H <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,L <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

/// SET 7,(HL) <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - - 
pub fn set_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    16
}

/// SET 7,A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - - 
pub fn set_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    8
}

#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::MemoryMap;
use crate::emulator::Cpu;

/// RLCA - 0x07 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C
pub fn rlca(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.circular_shift_left(cpu.registers.a, 1);
    // Reset the z flag because above instruction sets the z flag based on the result.
    cpu.registers.set_flags(0, 0, 0, cpu.registers.get_cy());
    4
}

/// RRCA - 0x0F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C
pub fn rrca(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.circular_shift_right(cpu.registers.a, 1);
    // Reset the z flag because above instruction sets the z flag based on the result.
    cpu.registers.set_flags(0, 0, 0, cpu.registers.get_cy());
    4
}

/// RLA - 0x17 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C
pub fn rla(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.rotate_left_cy(cpu.registers.a, 1);
    // Reset the z flag because above instruction sets the z flag based on the result.
    cpu.registers.set_flags(0, 0, 0, cpu.registers.get_cy());
    4
}

/// RRA - 0x1F <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: 0 0 0 C
pub fn rra(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.rotate_right_cy(cpu.registers.a, 1);
    // Reset the z flag because above instruction sets the z flag based on the result.
    cpu.registers.set_flags(0, 0, 0, cpu.registers.get_cy());
    4
}

/// RLC B - 0x00 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.circular_shift_left(cpu.registers.b, 1);
    8
}

/// RLC C - 0x01 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.circular_shift_left(cpu.registers.c, 1);
    8
}

/// RLC D - 0x02 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.circular_shift_left(cpu.registers.d, 1);
    8
}

/// RLC E - 0x03 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.circular_shift_left(cpu.registers.e, 1);
    8
}

/// RLC H - 0x04 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.circular_shift_left(cpu.registers.h, 1);
    8
}

/// RLC L - 0x05 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.circular_shift_left(cpu.registers.l, 1);
    8
}

/// RLC (HL) - 0x06 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.circular_shift_left(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// RLC A - 0x07 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rlc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.circular_shift_left(cpu.registers.a, 1);
    8
}

/// RRC B - 0x08 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.circular_shift_right(cpu.registers.b, 1);
    8
}

/// RRC C - 0x09 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.circular_shift_right(cpu.registers.c, 1);
    8
}

/// RRC D - 0x0A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.circular_shift_right(cpu.registers.d, 1);
    8
}

/// RRC E - 0x0B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.circular_shift_right(cpu.registers.e, 1);
    8
}

/// RRC H - 0x0C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.circular_shift_right(cpu.registers.h, 1);
    8
}

/// RRC L - 0x0D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.circular_shift_right(cpu.registers.l, 1);
    8
}

/// RRC (HL) - 0x0E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.circular_shift_right(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// RRC A - 0x0F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rrc_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.circular_shift_right(cpu.registers.a, 1);
    8
}

/// RL B - 0x10 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.rotate_left_cy(cpu.registers.b, 1);
    8
}

/// RL C - 0x11 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.rotate_left_cy(cpu.registers.c, 1);
    8
}

/// RL D - 0x12 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.rotate_left_cy(cpu.registers.d, 1);
    8
}

/// RL E - 0x13 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.rotate_left_cy(cpu.registers.e, 1);
    8
}

/// RL H - 0x14 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.rotate_left_cy(cpu.registers.h, 1);
    8
}

/// RL L - 0x15 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.rotate_left_cy(cpu.registers.l, 1);
    8
}

/// RL (HL) - 0x16 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn rl_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.rotate_left_cy(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// RL A - 0x17 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rl_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.rotate_left_cy(cpu.registers.a, 1);
    8
}

/// RR B - 0x18 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.rotate_right_cy(cpu.registers.b, 1);
    8
}

/// RR C - 0x19 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.rotate_right_cy(cpu.registers.c, 1);
    8
}

/// RR D - 0x1A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.rotate_right_cy(cpu.registers.d, 1);
    8
}

/// RR E - 0x1B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.rotate_right_cy(cpu.registers.e, 1);
    8
}

/// RR H - 0x1C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.rotate_right_cy(cpu.registers.h, 1);
    8
}

/// RR L - 0x1D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.rotate_right_cy(cpu.registers.l, 1);
    8
}

/// RR (HL) - 0x1E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn rr_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.rotate_right_cy(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// RR A - 0x1F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn rr_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.rotate_right_cy(cpu.registers.a, 1);
    8
}

/// SLA B - 0x20 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.shift_left_arithmetic(cpu.registers.b);
    8
}

/// SLA C - 0x21 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.shift_left_arithmetic(cpu.registers.c);
    8
}

/// SLA D - 0x22 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.shift_left_arithmetic(cpu.registers.d);
    8
}

/// SLA E - 0x23 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.shift_left_arithmetic(cpu.registers.e);
    8
}

/// SLA H - 0x24 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.shift_left_arithmetic(cpu.registers.h);
    8
}

/// SLA L - 0x25 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.shift_left_arithmetic(cpu.registers.l);
    8
}

/// SLA (HL) - 0x26 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn sla_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.shift_left_arithmetic(memory_map.get(cpu.registers.hl())),
    );
    16
}

/// SLA A - 0x27 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn sla_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.shift_left_arithmetic(cpu.registers.a);
    8
}

/// SRA B - 0x28 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.shift_right_arithmetic(cpu.registers.b);
    8
}

/// SRA C - 0x29 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.shift_right_arithmetic(cpu.registers.c);
    8
}

/// SRA D - 0x2A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.shift_right_arithmetic(cpu.registers.d);
    8
}

/// SRA E - 0x2B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.shift_right_arithmetic(cpu.registers.e);
    8
}

/// SRA H - 0x2C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.shift_right_arithmetic(cpu.registers.h);
    8
}

/// SRA L - 0x2D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.shift_right_arithmetic(cpu.registers.l);
    8
}

/// SRA (HL) - 0x2E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 0
pub fn sra_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.shift_right_arithmetic(memory_map.get(cpu.registers.hl())),
    );
    16
}

/// SRA A - 0x2F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn sra_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.shift_right_arithmetic(cpu.registers.a);
    8
}

/// SWAP B - 0x30 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.swap(cpu.registers.b);
    8
}

/// SWAP C - 0x31 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.swap(cpu.registers.c);
    8
}

/// SWAP D - 0x32 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.swap(cpu.registers.d);
    8
}

/// SWAP E - 0x33 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.swap(cpu.registers.e);
    8
}

/// SWAP H - 0x34 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.swap(cpu.registers.h);
    8
}

/// SWAP L - 0x35 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.swap(cpu.registers.l);
    8
}

/// SWAP (HL) - 0x36 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 0
pub fn swap_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.swap(memory_map.get(cpu.registers.hl())),
    );
    16
}

/// SWAP A - 0x37 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 0
pub fn swap_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.swap(cpu.registers.a);
    8
}

/// SRL B - 0x38 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.shift_right_logical(cpu.registers.b);
    8
}

/// SRL C - 0x39 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.shift_right_logical(cpu.registers.c);
    8
}

/// SRL D - 0x3A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.shift_right_logical(cpu.registers.d);
    8
}

/// SRL E - 0x3B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.shift_right_logical(cpu.registers.e);
    8
}

/// SRL H - 0x3C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.shift_right_logical(cpu.registers.h);
    8
}

/// SRL L - 0x3D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.shift_right_logical(cpu.registers.l);
    8
}

/// SRL (HL) - 0x3E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 0 C
pub fn srl_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.shift_right_logical(memory_map.get(cpu.registers.hl())),
    );
    16
}

/// SRL A - 0x3F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 0 C
pub fn srl_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.shift_right_logical(cpu.registers.a);
    8
}

/// BIT 0,B - 0x40 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 0);
    8
}

/// BIT 0,C - 0x41 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 0);
    8
}

/// BIT 0,D - 0x42 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 0);
    8
}

/// BIT 0,E - 0x43 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 0);
    8
}

/// BIT 0,H - 0x44 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 0);
    8
}

/// BIT 0,L - 0x45 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 0);
    8
}

/// BIT 0,(HL) - 0x46 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 0);
    16
}

/// BIT 0,A - 0x47 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 0);
    8
}

/// BIT 1,B - 0x48 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 1);
    8
}

/// BIT 1,C - 0x49 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 1);
    8
}

/// BIT 1,D - 0x4A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 1);
    8
}

/// BIT 1,E - 0x4B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 1);
    8
}

/// BIT 1,H - 0x4C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 1);
    8
}

/// BIT 1,L - 0x4D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 1);
    8
}

/// BIT 1,(HL) - 0x4E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 1);
    16
}

/// BIT 1,A - 0x4F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 1);
    8
}

/// BIT 2,B - 0x50 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 2);
    8
}

/// BIT 2,C - 0x51 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 2);
    8
}

/// BIT 2,D - 0x52 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 2);
    8
}

/// BIT 2,E - 0x53 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 2);
    8
}

/// BIT 2,H - 0x54 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 2);
    8
}

/// BIT 2,L - 0x55 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 2);
    8
}

/// BIT 2,(HL) - 0x56 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 2);
    16
}

/// BIT 2,A - 0x57 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 2);
    8
}

/// BIT 3,B - 0x58 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 3);
    8
}

/// BIT 3,C - 0x59 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 3);
    8
}

/// BIT 3,D - 0x5A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 3);
    8
}

/// BIT 3,E - 0x5B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 3);
    8
}

/// BIT 3,H - 0x5C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 3);
    8
}

/// BIT 3,L - 0x5D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 3);
    8
}

/// BIT 3,(HL) - 0x5E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 3);
    16
}

/// BIT 3,A - 0x5F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 3);
    8
}

/// BIT 4,B - 0x60 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 4);
    8
}

/// BIT 4,C - 0x61 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 4);
    8
}

/// BIT 4,D - 0x62 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 4);
    8
}

/// BIT 4,E - 0x63 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 4);
    8
}

/// BIT 4,H - 0x64 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 4);
    8
}

/// BIT 4,L - 0x65 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 4);
    8
}

/// BIT 4,(HL) - 0x66 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 4);
    16
}

/// BIT 4,A - 0x67 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 4);
    8
}

/// BIT 5,B - 0x68 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 5);
    8
}

/// BIT 5,C - 0x69 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 5);
    8
}

/// BIT 5,D - 0x6A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 5);
    8
}

/// BIT 5,E - 0x6B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 5);
    8
}

/// BIT 5,H - 0x6C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 5);
    8
}

/// BIT 5,L - 0x6D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 5);
    8
}

/// BIT 5,(HL) - 0x6E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 5);
    16
}

/// BIT 5,A - 0x6F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 5);
    8
}

/// BIT 6,B - 0x70 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 6);
    8
}

/// BIT 6,C - 0x71 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 6);
    8
}

/// BIT 6,D - 0x72 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 6);
    8
}

/// BIT 6,E - 0x73 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 6);
    8
}

/// BIT 6,H - 0x74 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 6);
    8
}

/// BIT 6,L - 0x75 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 6);
    8
}

/// BIT 6,(HL) - 0x76 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 6);
    16
}

/// BIT 6,A - 0x77 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 6);
    8
}

/// BIT 7,B - 0x78 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.b, 7);
    8
}

/// BIT 7,C - 0x79 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.c, 7);
    8
}

/// BIT 7,D - 0x7A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.d, 7);
    8
}

/// BIT 7,E - 0x7B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.e, 7);
    8
}

/// BIT 7,H - 0x7C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.h, 7);
    8
}

/// BIT 7,L - 0x7D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.l, 7);
    8
}

/// BIT 7,(HL) - 0x7E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(memory_map.get(cpu.registers.hl()), 7);
    16
}

/// BIT 7,A - 0x7F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: Z 0 1 -
pub fn bit_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.test_bit(cpu.registers.a, 7);
    8
}

/// RES 0,B - 0x80 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 0);
    8
}

/// RES 0,C - 0x81 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 0);
    8
}

/// RES 0,D - 0x82 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 0);
    8
}

/// RES 0,E - 0x83 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 0);
    8
}

/// RES 0,H - 0x84 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 0);
    8
}

/// RES 0,L - 0x85 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 0);
    8
}

/// RES 0,(HL) - 0x86 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 0),
    );
    16
}

/// RES 0,A - 0x87 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 0);
    8
}

/// RES 1,B - 0x88 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 1);
    8
}

/// RES 1,C - 0x89 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 1);
    8
}

/// RES 1,D - 0x8A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 1);
    8
}

/// RES 1,E - 0x8B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 1);
    8
}

/// RES 1,H - 0x8C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 1);
    8
}

/// RES 1,L - 0x8D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 1);
    8
}

/// RES 1,(HL) - 0x8E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// RES 1,A - 0x8F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 1);
    8
}

/// RES 2,B - 0x90 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 2);
    8
}

/// RES 2,C - 0x91 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 2);
    8
}

/// RES 2,D - 0x92 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 2);
    8
}

/// RES 2,E - 0x93 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 2);
    8
}

/// RES 2,H - 0x94 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 2);
    8
}

/// RES 2,L - 0x95 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 2);
    8
}

/// RES 2,(HL) - 0x96 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 2),
    );
    16
}

/// RES 2,A - 0x97 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 2);
    8
}

/// RES 3,B - 0x98 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 3);
    8
}

/// RES 3,C - 0x99 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 3);
    8
}

/// RES 3,D - 0x9A <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 3);
    8
}

/// RES 3,E - 0x9B <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 3);
    8
}

/// RES 3,H - 0x9C <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 3);
    8
}

/// RES 3,L - 0x9D <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 3);
    8
}

/// RES 3,(HL) - 0x9E <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 3),
    );
    16
}

/// RES 3,A - 0x9F <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 3);
    8
}

/// RES 4,B - 0xA0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 4);
    8
}

/// RES 4,C - 0xA1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 4);
    8
}

/// RES 4,D - 0xA2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 4);
    8
}

/// RES 4,E - 0xA3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 4);
    8
}

/// RES 4,H - 0xA4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 4);
    8
}

/// RES 4,L - 0xA5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 4);
    8
}

/// RES 4,(HL) - 0xA6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 4),
    );
    16
}

/// RES 4,A - 0xA7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 4);
    8
}

/// RES 5,B - 0xA8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 5);
    8
}

/// RES 5,C - 0xA9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 5);
    8
}

/// RES 5,D - 0xAA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 5);
    8
}

/// RES 5,E - 0xAB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 5);
    8
}

/// RES 5,H - 0xAC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 5);
    8
}

/// RES 5,L - 0xAD <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 5);
    8
}

/// RES 5,(HL) - 0xAE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 5),
    );
    16
}

/// RES 5,A - 0xAF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 5);
    8
}

/// RES 6,B - 0xB0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 6);
    8
}

/// RES 6,C - 0xB1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 6);
    8
}

/// RES 6,D - 0xB2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 6);
    8
}

/// RES 6,E - 0xB3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 6);
    8
}

/// RES 6,H - 0xB4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 6);
    8
}

/// RES 6,L - 0xB5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 6);
    8
}

/// RES 6,(HL) - 0xB6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 6),
    );
    16
}

/// RES 6,A - 0xB7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 6);
    8
}

/// RES 7,B - 0xB8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.reset_bit(cpu.registers.b, 7);
    8
}

/// RES 7,C - 0xB9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.reset_bit(cpu.registers.c, 7);
    8
}

/// RES 7,D - 0xBA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.reset_bit(cpu.registers.d, 7);
    8
}

/// RES 7,E - 0xBB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.reset_bit(cpu.registers.e, 7);
    8
}

/// RES 7,H - 0xBC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.reset_bit(cpu.registers.h, 7);
    8
}

/// RES 7,L - 0xBD <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.reset_bit(cpu.registers.l, 7);
    8
}

/// RES 7,(HL) - 0xBE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn res_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.reset_bit(memory_map.get(cpu.registers.hl()), 7),
    );
    16
}

/// RES 7,A - 0xBF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn res_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.reset_bit(cpu.registers.a, 7);
    8
}

/// SET 0,B - 0xC0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 0);
    8
}

/// SET 0,C - 0xC1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 0);
    8
}

/// SET 0,D - 0xC2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 0);
    8
}

/// SET 0,E - 0xC3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 0);
    8
}

/// SET 0,H - 0xC4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 0);
    8
}

/// SET 0,L - 0xC5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 0);
    8
}

/// SET 0,(HL) - 0xC6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_0_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 0),
    );
    16
}

/// SET 0,A - 0xC7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_0_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 0);
    8
}

/// SET 1,B - 0xC8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 1);
    8
}

/// SET 1,C - 0xC9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 1);
    8
}

/// SET 1,D - 0xCA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 1);
    8
}

/// SET 1,E - 0xCB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 1);
    8
}

/// SET 1,H - 0xCC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 1);
    8
}

/// SET 1,L - 0xCD <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 1);
    8
}

/// SET 1,(HL) - 0xCE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_1_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 1),
    );
    16
}

/// SET 1,A - 0xCF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_1_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 1);
    8
}

/// SET 2,B - 0xD0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 2);
    8
}

/// SET 2,C - 0xD1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 2);
    8
}

/// SET 2,D - 0xD2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 2);
    8
}

/// SET 2,E - 0xD3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 2);
    8
}

/// SET 2,H - 0xD4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 2);
    8
}

/// SET 2,L - 0xD5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 2);
    8
}

/// SET 2,(HL) - 0xD6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_2_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 2),
    );
    16
}

/// SET 2,A - 0xD7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_2_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 2);
    8
}

/// SET 3,B - 0xD8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 3);
    8
}

/// SET 3,C - 0xD9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 3);
    8
}

/// SET 3,D - 0xDA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 3);
    8
}

/// SET 3,E - 0xDB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 3);
    8
}

/// SET 3,H - 0xDC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 3);
    8
}

/// SET 3,L - 0xDD <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 3);
    8
}

/// SET 3,(HL) - 0xDE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_3_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 3),
    );
    16
}

/// SET 3,A - 0xDF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_3_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 3);
    8
}

/// SET 4,B - 0xE0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 4);
    8
}

/// SET 4,C - 0xE1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 4);
    8
}

/// SET 4,D - 0xE2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 4);
    8
}

/// SET 4,E - 0xE3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 4);
    8
}

/// SET 4,H - 0xE4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 4);
    8
}

/// SET 4,L - 0xE5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 4);
    8
}

/// SET 4,(HL) - 0xE6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_4_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 4),
    );
    16
}

/// SET 4,A - 0xE7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_4_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 4);
    8
}

/// SET 5,B - 0xE8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 5);
    8
}

/// SET 5,C - 0xE9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 5);
    8
}

/// SET 5,D - 0xEA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 5);
    8
}

/// SET 5,E - 0xEB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 5);
    8
}

/// SET 5,H - 0xEC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 5);
    8
}

/// SET 5,L - 0xED <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 5);
    8
}

/// SET 5,(HL) - 0xEE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_5_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 5),
    );
    16
}

/// SET 5,A - 0xEF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_5_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 5);
    8
}

/// SET 6,B - 0xF0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 6);
    8
}

/// SET 6,C - 0xF1 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 6);
    8
}

/// SET 6,D - 0xF2 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 6);
    8
}

/// SET 6,E - 0xF3 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 6);
    8
}

/// SET 6,H - 0xF4 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 6);
    8
}

/// SET 6,L - 0xF5 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 6);
    8
}

/// SET 6,(HL) - 0xF6 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_6_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 6),
    );
    16
}

/// SET 6,A - 0xF7 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_6_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 6);
    8
}

/// SET 7,B - 0xF8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_b(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.b = cpu.set_bit(cpu.registers.b, 7);
    8
}

/// SET 7,C - 0xF9 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.c = cpu.set_bit(cpu.registers.c, 7);
    8
}

/// SET 7,D - 0xFA <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_d(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.d = cpu.set_bit(cpu.registers.d, 7);
    8
}

/// SET 7,E - 0xFB <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_e(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.e = cpu.set_bit(cpu.registers.e, 7);
    8
}

/// SET 7,H - 0xFC <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.h = cpu.set_bit(cpu.registers.h, 7);
    8
}

/// SET 7,L - 0xFD <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_l(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.l = cpu.set_bit(cpu.registers.l, 7);
    8
}

/// SET 7,(HL) - 0xFE <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn set_7_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    memory_map.set(
        cpu.registers.hl(),
        cpu.set_bit(memory_map.get(cpu.registers.hl()), 7),
    );
    16
}

/// SET 7,A - 0xFF <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 8 <br>
///  Flags affected: - - - -
pub fn set_7_a(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.registers.a = cpu.set_bit(cpu.registers.a, 7);
    8
}

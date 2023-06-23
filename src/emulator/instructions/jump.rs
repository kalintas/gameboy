#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::MemoryMap;
use crate::emulator::Cpu;

/// JR r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn jr_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// JR NZ,r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_nz_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// JR Z,r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_z_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// JR NC,r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_nc_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// JR C,r8 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_c_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// RET NZ <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_nz(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// JP NZ,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_nz_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// JP a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn jp_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// CALL NZ,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_nz_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// RST 00H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_00h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RET Z <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_z(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// RET <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ret(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// JP Z,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_z_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// CALL Z,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_z_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// CALL a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24 <br>
///  Flags affected: - - - -
pub fn call_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    24
}

/// RST 08H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_08h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RET NC <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_nc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// JP NC,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_nc_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// CALL NC,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_nc_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// RST 10H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_10h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RET C <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    8
}

/// RETI <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn reti(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// JP C,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_c_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// CALL C,a16 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_c_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    12
}

/// RST 18H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_18h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RST 20H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_20h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// JP (HL) <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn jp_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// RST 28H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_28h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RST 30H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_30h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

/// RST 38H <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_38h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    16
}

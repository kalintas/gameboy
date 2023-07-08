
#![allow(dead_code, unused_variables)]
use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;


/// NOP <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn nop(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// STOP 0 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn stop_0(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// HALT <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn halt(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// PREFIX CB <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn prefix_cb(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// DI <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn di(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

/// EI <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn ei(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    
    4
}

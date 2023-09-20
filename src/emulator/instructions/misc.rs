
#![allow(dead_code, unused_variables)]
use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;


/// NOP - 0x00 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn nop(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // No operation.
    4
}

/// STOP 0 - 0x10 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn stop_0(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Halt CPU & LCD display until button pressed.
    unimplemented!();
    // 4
}

/// HALT - 0x76 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn halt(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Power down CPU until an interrupt occurs.
    unimplemented!();
    // 4
}

/// PREFIX CB - 0xCB <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn prefix_cb(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    4
}

/// DI - 0xF3 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn di(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    /*
        This instruction disables interrupts but not
        immediately. Interrupts are disabled after
        instruction after DI is executed.
    */
    cpu.disable_interrupts();

    4
}

/// EI - 0xFB <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - - 
pub fn ei(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    /*
        Enable interrupts. This intruction enables interrupts
        but not immediately. Interrupts are enabled after
        instruction after EI is executed.
    */

    cpu.enable_interrupts();

    4
}

#![allow(dead_code, unused_variables)]
use crate::emulator::memory_map::MemoryMap;
use crate::emulator::Cpu;

/// JR r8 - 0x18 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12 <br>
///  Flags affected: - - - -
pub fn jr_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.pc = (cpu.pc as i32 + (cpu.get_immediate_u8(memory_map) as i8) as i32) as u16;
    12
}

/// JR NZ,r8 - 0x20 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_nz_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 0 {
        return jr_r8(cpu, memory_map);
    }

    8
}

/// JR Z,r8 - 0x28 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_z_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 1 {
        return jr_r8(cpu, memory_map);
    }

    8
}

/// JR NC,r8 - 0x30 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_nc_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 0 {
        return jr_r8(cpu, memory_map);
    }

    8
}

/// JR C,r8 - 0x38 <br>
///  Length in bytes: 2 <br>
///  Duration in cycles: 12/8 <br>
///  Flags affected: - - - -
pub fn jr_c_r8(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 1 {
        return jr_r8(cpu, memory_map);
    }

    8
}

/// RET NZ - 0xC0 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_nz(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 0 {
        ret(cpu, memory_map);
        20
    } else {
        8
    }
}

/// JP NZ,a16 - 0xC2 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_nz_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 0 {
        return jp_a16(cpu, memory_map);
    }

    12
}

/// JP a16 - 0xC3 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn jp_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.pc = cpu.get_immediate_u16(memory_map);

    16
}

/// CALL NZ,a16 - 0xC4 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_nz_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 0 {
        return call_a16(cpu, memory_map);
    }

    12
}

/// RST 00H - 0xC7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_00h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x00);
    16
}

/// RET Z - 0xC8 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_z(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 1 {
        ret(cpu, memory_map);
        20
    } else {
        8
    }
}

/// RET - 0xC9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn ret(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.pc = cpu.pop(memory_map);
    16
}

/// JP Z,a16 - 0xCA <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_z_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 1 {
        return jp_a16(cpu, memory_map);
    }

    12
}

/// CALL Z,a16 - 0xCC <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_z_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_z() == 1 {
        return call_a16(cpu, memory_map);
    }

    12
}

/// CALL a16 - 0xCD <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24 <br>
///  Flags affected: - - - -
pub fn call_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, cpu.get_immediate_u16(memory_map));

    24
}

/// RST 08H - 0xCF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_08h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x08);
    16
}

/// RET NC - 0xD0 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_nc(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 0 {
        ret(cpu, memory_map);
        20
    } else {
        8
    }
}

/// JP NC,a16 - 0xD2 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_nc_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 0 {
        return jp_a16(cpu, memory_map);
    }

    12
}

/// CALL NC,a16 - 0xD4 <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_nc_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 0 {
        return call_a16(cpu, memory_map);
    }

    12
}

/// RST 10H - 0xD7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_10h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x10);
    16
}

/// RET C - 0xD8 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 20/8 <br>
///  Flags affected: - - - -
pub fn ret_c(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 1 {
        ret(cpu, memory_map);
        20
    } else {
        8
    }
}

/// RETI - 0xD9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn reti(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    // Return and enable interrupts (IME=1)
    cpu.enable_interrupts();
    ret(cpu, memory_map)
}

/// JP C,a16 - 0xDA <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 16/12 <br>
///  Flags affected: - - - -
pub fn jp_c_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 1 {
        return jp_a16(cpu, memory_map);
    }

    12
}

/// CALL C,a16 - 0xDC <br>
///  Length in bytes: 3 <br>
///  Duration in cycles: 24/12 <br>
///  Flags affected: - - - -
pub fn call_c_a16(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    if cpu.registers.get_cy() == 1 {
        return call_a16(cpu, memory_map);
    }

    12
}

/// RST 18H - 0xDF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_18h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x18);
    16
}

/// RST 20H - 0xE7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_20h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x20);
    16
}

/// JP (HL) - 0xE9 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 4 <br>
///  Flags affected: - - - -
pub fn jp_hl_addr(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.pc = cpu.registers.hl();
    4
}

/// RST 28H - 0xEF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_28h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x28);
    16
}

/// RST 30H - 0xF7 <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_30h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x30);
    16
}

/// RST 38H - 0xFF <br>
///  Length in bytes: 1 <br>
///  Duration in cycles: 16 <br>
///  Flags affected: - - - -
pub fn rst_38h(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    cpu.call(memory_map, 0x38);
    16
}

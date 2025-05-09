use super::{
    instructions::{Instruction, INSTRUCTIONS, PREFIX_CB_INSTRUCTIONS},
    memory_map::{Io, MemoryMap, OamCorruption},
    registers::Registers,
};

#[derive(Clone)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub clock_cycles: u32,
    pub registers: Registers,
    pub ime: bool,

    halt_mode: bool,
    stop_mode: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            clock_cycles: 0,
            registers: Registers::new(),
            ime: false,

            halt_mode: false,
            stop_mode: false,
        }
    }

    pub fn after_boot() -> Self {
        Self {
            pc: 0x100,
            sp: 0xFFFE,
            clock_cycles: 0,
            registers: Registers::after_boot(),
            ime: false,

            halt_mode: false,
            stop_mode: false,
        }
    }

    pub fn cycle(&mut self, memory_map: &mut MemoryMap) {
        if self.handle_interrupts(memory_map) {
            return;
        }

        // TODO: https://gbdev.io/pandocs/halt.html?highlight=halt#halt-bug
        // In halt mode CPU is powered down until an interrupt occurs.
        if self.halt_mode || self.stop_mode {
            return;
        }

        let instruction = {
            let opcode = Self::fetch(self.pc, memory_map);
            memory_map.mem_syncer.open_sync();

            if opcode == 0xCB {
                PREFIX_CB_INSTRUCTIONS[Self::fetch(self.pc + 1, memory_map) as usize]
            } else {
                INSTRUCTIONS[opcode as usize]
            }
        };

        self.execute(instruction, memory_map);
        memory_map.mem_syncer.close_sync();
    }

    fn fetch(pc: u16, memory_map: &MemoryMap) -> u8 {
        memory_map.cpu_get(pc)
    }

    fn execute(&mut self, instruction: Instruction, memory_map: &mut MemoryMap) {
        self.pc += instruction.length as u16;

        let instruction_cycles = (instruction.function)(self, memory_map) as u32;

        self.clock_cycles += instruction_cycles;
        // self.clock_cycles = self.clock_cycles.wrapping_add(instruction_cycles);
    }

    fn handle_interrupt(&self, memory_map: &MemoryMap) -> Option<(u16, u8)> {
        /*
            From pandocs:
            Provided that IME and IE allow the execution of more than one of the requested interrupts,
            then the interrupt with the highest priority becomes executed first.
            The priorities are ordered as the bits in the IE and IF registers,
            Bit 0 (V-Blank) having the highest priority, and Bit 4 (Joypad) having the lowest priority.
        */

        let ie_reg = memory_map.cpu_get_io(Io::IE);
        let if_reg = memory_map.cpu_get_io(Io::IF);
        let interrupt = ie_reg & if_reg;

        if interrupt & 0x1 != 0 {
            // V-Blank interrupt
            // println!("VBLANK");
            return Some((0x40, if_reg & 0x1E));
        } else if interrupt & 0x2 != 0 {
            // LCD STAT interrupt
            return Some((0x48, if_reg & 0xFD));
        } else if interrupt & 0x4 != 0 {
            // Timer interrupt
            return Some((0x50, if_reg & 0xFB));
        } else if interrupt & 0x8 != 0 {
            // Serial interrupt
            return Some((0x58, if_reg & 0xF7));
        } else if interrupt & 0x10 != 0 {
            // Joypad interrupt
            return Some((0x60, if_reg & 0xEF));
        }

        return None;
    }

    fn handle_interrupts(&mut self, memory_map: &mut MemoryMap) -> bool {
        if let Some((interrupt_address, new_if_reg)) = self.handle_interrupt(&memory_map) {
            // Halt mode is disabled regardless of whether the interrupt is handled or not.
            self.halt_mode = false;
            self.stop_mode = false;

            if !self.ime {
                return false;
            }

            self.ime = false;

            self.call(memory_map, interrupt_address);

            // Replace the if flag.
            memory_map.cpu_set_io(Io::IF, new_if_reg);

            // TODO: "The entire routine should last a total of 5 M-cycles." 5 M-cycles =? 20
            self.clock_cycles = self.clock_cycles.wrapping_add(20);

            true
        } else {
            false
        }
    }

    ///////////////////////////////////////////////////

    // Get immediate byte after an instruction.
    pub fn get_immediate_u8(&self, memory_map: &MemoryMap) -> u8 {
        memory_map.cpu_get(self.pc - 1)
    }

    // Get immediate two bytes after an instruction.
    pub fn get_immediate_u16(&self, memory_map: &MemoryMap) -> u16 {
        memory_map.cpu_get_u16(self.pc - 2)
    }

    pub fn add(&mut self, rhs: u8) {
        let lhs = self.registers.a;
        self.registers.a = self.registers.a.wrapping_add(rhs);

        self.registers.set_flags(
            (self.registers.a == 0) as u8,
            0,
            ((lhs & 0xF) + (rhs & 0xF) > 0xF) as u8,
            ((lhs as u16 + rhs as u16) > 0xFF) as u8,
        );
    }

    pub fn adc(&mut self, rhs: u8) {
        let cy = self.registers.get_cy(); // Get the carry flag
        let lhs = self.registers.a; // Get the value in register A

        // Perform the addition with carry
        let result = lhs.wrapping_add(rhs).wrapping_add(cy);

        // Set the flags
        self.registers.set_flags(
            (result == 0) as u8,                                  // Zero flag (Z)
            0,                                                    // Subtraction flag (N) is cleared
            ((lhs & 0x0F) + (rhs & 0x0F) + cy > 0x0F) as u8,      // Half carry flag (H)
            ((lhs as u16 + rhs as u16 + cy as u16) > 0xFF) as u8, // Carry flag (C)
        );

        // Store the result in register A
        self.registers.a = result;
    }

    pub fn add_u16(&mut self, rhs: u16) {
        let lhs = self.registers.hl();
        self.registers.set_hl(lhs.wrapping_add(rhs) as u16);

        self.registers.set_flags(
            self.registers.get_z(),
            0,
            ((lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF) as u8,
            ((lhs as u32 + rhs as u32) > 0xFFFF) as u8,
        );
    }

    pub fn sub(&mut self, rhs: u8) {
        let lhs = self.registers.a;
        let result = lhs.wrapping_sub(rhs);

        self.registers.set_flags(
            (result == 0) as u8,
            1,
            (lhs & 0xF < rhs & 0xF) as u8,
            (lhs < rhs) as u8,
        );

        self.registers.a = result;
    }

    pub fn sbc(&mut self, rhs: u8) {
        let cy = self.registers.get_cy(); // Get the carry flag
        let lhs = self.registers.a; // Get the value in register A

        // Perform the subtraction with carry
        let result = lhs.wrapping_sub(rhs).wrapping_sub(cy);

        // Set the flags
        self.registers.set_flags(
            (result == 0) as u8,                             // Zero flag (Z)
            1,                                               // Subtraction flag (N) is set
            ((lhs & 0x0F) < (rhs & 0x0F) + cy) as u8,        // Half carry flag (H)
            ((lhs as u16) < (rhs as u16 + cy as u16)) as u8, // Carry flag (C)
        );

        // Store the result in register A
        self.registers.a = result;
    }

    pub fn and(&mut self, rhs: u8) {
        self.registers.a &= rhs;

        self.registers
            .set_flags((self.registers.a == 0) as u8, 0, 1, 0);
    }

    pub fn or(&mut self, rhs: u8) {
        self.registers.a |= rhs;

        self.registers
            .set_flags((self.registers.a == 0) as u8, 0, 0, 0);
    }

    pub fn xor(&mut self, rhs: u8) {
        self.registers.a ^= rhs;

        self.registers
            .set_flags((self.registers.a == 0) as u8, 0, 0, 0);
    }

    pub fn inc(&mut self, val: u8) -> u8 {
        let result = val.wrapping_add(1);

        /*
            Flags affected:
            Z - Set if result is zero.
            N - Reset.
            H - Set if carry from bit 3.
            C - Not affected.
        */
        self.registers.set_flags(
            (result == 0) as u8,
            0,
            ((val & 0xF) + 1 > 0xF) as u8,
            self.registers.get_cy(),
        );
        result
    }
    pub fn dec(&mut self, val: u8) -> u8 {
        let result = val.wrapping_sub(1);

        /*
            Flags affected:
            Z - Set if result is zero.
            N - Set.
            H - Set if no borrow from bit 4.
            C - Not affected
        */
        self.registers.set_flags(
            (result == 0) as u8,
            1,
            (val & 0x10 != result & 0x10) as u8,
            self.registers.get_cy(),
        );
        result
    }

    // Bit operations.

    pub fn rotate_left_cy(&mut self, val: u8, n: u32) -> u8 {
        let result = (val << n) | self.registers.get_cy();
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn rotate_right_cy(&mut self, val: u8, n: u32) -> u8 {
        let result = (val >> n) | (self.registers.get_cy() << 7);
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn circular_shift_left(&mut self, val: u8, n: u32) -> u8 {
        let result = val.rotate_left(n);
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn circular_shift_right(&mut self, val: u8, n: u32) -> u8 {
        let result = val.rotate_right(n);
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn shift_left_arithmetic(&mut self, val: u8) -> u8 {
        let result = val << 1;
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn shift_right_arithmetic(&mut self, val: u8) -> u8 {
        let result = (val >> 1) | (val & 0x80); // MSB doesn't change.
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn shift_right_logical(&mut self, val: u8) -> u8 {
        let result = val >> 1; // MSB set to 0.
        self.registers
            .set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn swap(&mut self, val: u8) -> u8 {
        let result = (val << 4) | (val >> 4);
        self.registers.set_flags((result == 0) as u8, 0, 0, 0);

        result
    }

    pub fn test_bit(&mut self, val: u8, bit: u8) {
        let result = (val >> bit) & 0x1;
        self.registers
            .set_flags((result == 0) as u8, 0, 1, self.registers.get_cy());
    }

    pub fn set_bit(&mut self, val: u8, bit: u8) -> u8 {
        val | (1 << bit)
    }

    pub fn reset_bit(&mut self, val: u8, bit: u8) -> u8 {
        val & !(1 << bit)
    }

    pub fn push(&mut self, memory_map: &mut MemoryMap, value: u16) {
        memory_map.try_corrupt_oam(self.sp, OamCorruption::Read);
        self.sp -= 2;
        memory_map.cpu_set_u16(self.sp, value);
    }

    pub fn pop(&mut self, memory_map: &mut MemoryMap) -> u16 {
        memory_map.disable_oam_corruption();
        memory_map.try_corrupt_oam(self.sp, OamCorruption::IncDecRead);
        let lsb = memory_map.cpu_get(self.sp) as u16; // Get the least significant byte
        memory_map.try_corrupt_oam(self.sp + 1, OamCorruption::Read);
        let msb = memory_map.cpu_get(self.sp + 1) as u16; // Get the most significant byte
        memory_map.enable_oam_corruption();

        self.sp += 2;

        lsb | (msb << 8)
    }

    pub fn call(&mut self, memory_map: &mut MemoryMap, address: u16) {
        self.push(memory_map, self.pc);
        self.pc = address;
    }

    // Interrupts
    /*
        The effect of ei is delayed by one instruction.
        This means that ei followed immediately by di does not allow any interrupts between them.
        This interacts with the halt bug in an interesting way.
    */

    pub fn enable_interrupts(&mut self) {
        // memory_map.set(0xFFFF, 0x1F);
        self.ime = true;
    }

    pub fn disable_interrupts(&mut self) {
        // memory_map.set(0xFFFF, 0x00);
        self.ime = false;
    }

    pub fn halt(&mut self) {
        self.halt_mode = true;
    }

    // pub fn stop(&mut self) {
    //     self.stop_mode = true;
    // }

    pub fn is_stopped(&self) -> bool {
        self.stop_mode
    }
}

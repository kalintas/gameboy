use super::{
    instructions::{Instruction, INSTRUCTIONS, PREFIX_CB_INSTRUCTIONS},
    memory_map::{Io, MemoryMap},
    registers::Registers,
};

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
        if self.halt_mode {
            return;
        }

        let instruction = Self::decode(self.pc, &memory_map);

        self.execute(instruction, memory_map);
    }

    pub fn fetch(pc: u16, memory_map: &MemoryMap) -> u8 {
        memory_map.get(pc)
    }

    pub fn decode(pc: u16, memory_map: &MemoryMap) -> Instruction {
        let opcode = Self::fetch(pc, memory_map);

        if opcode == 0xCB {
            PREFIX_CB_INSTRUCTIONS[Self::fetch(pc + 1, memory_map) as usize]
        } else {
            INSTRUCTIONS[opcode as usize]
        }
    }

    pub fn execute(&mut self, instruction: Instruction, memory_map: &mut MemoryMap) {
        self.pc += instruction.length as u16;

        let instruction_cycles = (instruction.function)(self, memory_map) as u32;

        self.clock_cycles = self.clock_cycles.wrapping_add(instruction_cycles);
    }

    fn handle_interrupt(&self, memory_map: &MemoryMap) -> Option<(u16, u8)> {
        /*
            From pandocs:
            Provided that IME and IE allow the execution of more than one of the requested interrupts,
            then the interrupt with the highest priority becomes executed first.
            The priorities are ordered as the bits in the IE and IF registers,
            Bit 0 (V-Blank) having the highest priority, and Bit 4 (Joypad) having the lowest priority.
        */

        let ie_reg = memory_map.get_io(Io::IE);
        let if_reg = memory_map.get_io(Io::IF);
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

            if !self.ime {
                return false;
            }

            self.ime = false;

            self.call(memory_map, interrupt_address);

            // Replace the if flag.
            memory_map.set_io(Io::IF, new_if_reg);

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
        memory_map.get(self.pc - 1)
    }

    // Get immediate two bytes after an instruction.
    pub fn get_immediate_u16(&self, memory_map: &MemoryMap) -> u16 {
        memory_map.get_u16(self.pc - 2)
    }

    pub fn add(&mut self, rhs: u8) {
        let lhs = self.registers.a;
        let result = lhs as u16 + rhs as u16;

        // TODO: check H flag to see if its correct
        self.registers.set_flags(
            (result == 0) as u8,
            0,
            (lhs & 0xF + rhs & 0xF > 0xF) as u8,
            (result > 0xFF) as u8,
        );

        self.registers.a = (result % 0x100) as u8;
    }

    pub fn adc(&mut self, rhs: u8) {
        self.add(rhs.wrapping_add(self.registers.get_cy()));
    }

    pub fn add_u16(&mut self, rhs: u16) {
        let lhs = self.registers.hl();
        let result = lhs as u32 + rhs as u32;

        self.registers.set_flags(
            self.registers.get_z(),
            0,
            ((lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF) as u8,
            (result > 0xFFFF) as u8,
        );

        self.registers.set_hl((result % 0x10000) as u16);
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
        self.sub(rhs.wrapping_add(self.registers.get_cy()));
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
            (val & 0xF + 1 > 0xF) as u8,
            self.registers.get_cy(),
        );
        result
    }
    pub fn dec(&mut self, val: u8) -> u8 {
        let result = val.wrapping_sub(1);

        /*
            Flags affected:
            Z - Set if reselt is zero.
            N - Set.
            H - Set if no borrow from bit 4.
            C - Not affected
        */
        self.registers.set_flags(
            (result == 0) as u8,
            1,
            ((val & 0x10) != 0 && (result & 0x10) == 0) as u8,
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
        let result = (val >> 1) | (val & 0x80); // MSB doesnt change.
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

    pub fn call(&mut self, memory_map: &mut MemoryMap, address: u16) {
        self.sp -= 2;
        memory_map.set_u16(self.sp, self.pc);
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

    pub fn stop(&mut self) {
        self.stop_mode = true;
    }

    pub fn is_stopped(&self) -> bool {
        self.stop_mode
    }
}

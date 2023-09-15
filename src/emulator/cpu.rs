use super::{
    instructions::{Instruction, INSTRUCTIONS, PREFIX_CB_INSTRUCTIONS},
    memory_map::MemoryMap,
    registers::Registers,
};

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub clock_cycles: u32,
    pub registers: Registers,
    pub ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            clock_cycles: 0,
            registers: Registers::new(),
            ime: false
        }
    }

    pub fn after_boot() -> Self {

        Self {
            pc: 0x100,
            sp: 0xFFFE,
            clock_cycles: 0,
            registers: Registers::after_boot(),
            ime: false
        }
    }

    pub fn cycle(&mut self, memory_map: &mut MemoryMap) {

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
        
        // TODO: maybe fix this?
        let old_pc = self.pc;

        self.clock_cycles += (instruction.function)(self, memory_map) as u32;

        // Didn't jump to another location.
        if self.pc == old_pc {
            self.pc += instruction.length as u16;
        }

    }

    ///////////////////////////////////////////////////

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

        self.registers.set_flags((self.registers.a == 0) as u8, 0, 1, 0);
    }

    pub fn or(&mut self, rhs: u8) {
        self.registers.a |= rhs;

        self.registers.set_flags((self.registers.a == 0) as u8, 0, 0, 0);
    }

    pub fn xor(&mut self, rhs: u8) {
        self.registers.a ^= rhs;

        self.registers.set_flags((self.registers.a == 0) as u8, 0, 0, 0);
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
        self.registers.set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn rotate_right_cy(&mut self, val: u8, n: u32) -> u8 {

        let result = (val >> n) | (self.registers.get_cy() << 7);
        self.registers.set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn circular_shift_left(&mut self, val: u8, n: u32) -> u8 {

        let result = val.rotate_left(n);
        self.registers.set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn circular_shift_right(&mut self, val: u8, n: u32) -> u8 {
        
        let result = val.rotate_right(n);
        self.registers.set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn shift_left_arithmetic(&mut self, val: u8) -> u8 {

        let result = val << 1;
        self.registers.set_flags((result == 0) as u8, 0, 0, val >> 7);

        result
    }

    pub fn shift_right_arithmetic(&mut self, val: u8) -> u8 {

        let result = (val >> 1) | (val & 0x80); // MSB doesnt change.
        self.registers.set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn shift_right_logical(&mut self, val: u8) -> u8 {

        let result = val >> 1; // MSB set to 0.
        self.registers.set_flags((result == 0) as u8, 0, 0, val & 0x1);

        result
    }

    pub fn swap(&mut self, val: u8) -> u8 {

        let result = (val << 4) | (val >> 4);
        self.registers.set_flags((result == 0) as u8, 0, 0, 0);

        result
    }

    pub fn test_bit(&mut self, val: u8, bit: u8) {

        let result = (val >> bit) & 0x1;
        self.registers.set_flags((result == 0) as u8, 0, 1, self.registers.get_cy());
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

}

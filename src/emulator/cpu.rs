use super::{memory_map::MemoryMap, instructions::{Instruction, INSTRUCTIONS, self}, registers::Registers};


pub struct Cpu {

    pub pc: u16,
    pub sp: u16,
    pub clock_cycles: u32,
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        
        Self {
            pc: 0x100,
            sp: 0,
            clock_cycles: 0,
            registers: Registers::new(),
        }
    }

    pub fn cycle(&mut self, memory_map: &mut MemoryMap) {

        let opcode = self.fetch(memory_map);

        let instruction = self.decode(opcode);

        self.execute(instruction, memory_map);
    }

    fn fetch(&self, memory_map: &MemoryMap) -> u8 {
        
        memory_map.get(self.pc)
    }

    fn decode(&self, opcode: u8) -> Instruction {
        
        INSTRUCTIONS[opcode as usize]
    }

    fn execute(&mut self, instruction: Instruction, memory_map: &mut MemoryMap) {
        
        self.clock_cycles += (instruction.function)(self, memory_map) as u32;
        
        self.pc += instruction.length as u16;
    }
}

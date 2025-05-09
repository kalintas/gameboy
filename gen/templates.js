

function instructionFile(instructions, prefixCBInstructions, maxInstructionNameLength) {

return`
mod misc;
mod jump;
mod load;
mod arithmetic;
mod bit;

use crate::Cpu;
use crate::memory_map::MemoryMap;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: &'static str,
    pub length: u8,
    pub function: fn(&mut Cpu, &mut MemoryMap) -> u8,
}

impl Instruction {
    #[allow(dead_code)]
    pub const fn new(name: &'static str, length: u8, function: fn(&mut Cpu, &mut MemoryMap) -> u8) -> Self {
        Self {
            name,
            length,
            function,
        }
    }
}

#[allow(dead_code)]
fn undefined(_cpu: &mut Cpu, _memory_map: &mut MemoryMap) -> u8 {
    4
} 

const UNDEFINED: Instruction = Instruction::new("UNDEFINED", 1, undefined);

#[allow(dead_code)]
pub const MAX_INSTRUCTION_NAME_LENGTH: usize = ${maxInstructionNameLength};

pub const INSTRUCTIONS: [Instruction; 0x100] = [
${instructions}
];

pub const PREFIX_CB_INSTRUCTIONS: [Instruction; 0x100] = [
${prefixCBInstructions}
];`
}

function instructionFunctionFile(instructionFunctions) {
    return`#![allow(dead_code, unused_variables)]
use crate::memory_map::MemoryMap;
use crate::Cpu;
${instructionFunctions}`
}

function instructionFunction(opcode, name, lengthInBytes, durationInCycles, flagsAffected, functionName) {

    const durationCycleSplit = durationInCycles.split("/");
    let returningCycle = durationCycleSplit[durationCycleSplit.length - 1];

    return `
/// ${name} - 0x${opcode} <br>
///  Length in bytes: ${lengthInBytes} <br>
///  Duration in cycles: ${durationInCycles} <br>
///  Flags affected: ${flagsAffected}
pub fn ${functionName}(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    ${returningCycle}
}\n`
}

module.exports = { instructionFile, instructionFunction, instructionFunctionFile }
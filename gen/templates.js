

function instructionFile(instructions, prefixCBInstructions) {

return`
mod misc;
mod jump;
mod load;
mod arithmetic;
mod bit;

use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;

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

pub const INSTRUCTIONS: [Instruction; 0x100] = [
${instructions}
];

pub const PREFIX_CB_INSTRUCTIONS: [Instruction; 0x100] = [
${prefixCBInstructions}
];`
}

function instructionFunctionFile(instructionFunctions) {
    return`
#![allow(dead_code, unused_variables)]
use crate::emulator::Cpu;
use crate::emulator::memory_map::MemoryMap;

${instructionFunctions}`
}

function getFunctionContent(name) {
    const loadRegex = /ld \(?([^|)]*)\)?,\(?([^|)]*)\)?/;

    const loadMatch = name.toLowerCase().match(loadRegex);

    if (loadMatch && loadMatch.length == 3 && name != "LD HL,SP+r8") {


        if (loadMatch[1] == "a" && loadMatch[2] == "c") {
            return "cpu.registers.a = memory_map.get(0xFF00 + cpu.registers.c as u16);"
        } else if (loadMatch[1] == "c" && loadMatch[2] == "a") {
            return "memory_map.set(0xFF00 + cpu.registers.c as u16, cpu.registers.a);";   
        }

        let rhs;

        if (loadMatch[2].length > 1) {

            if (loadMatch[2] == "d8") {
                rhs = "memory_map.get(cpu.pc + 1)";
            } else if (loadMatch[2] == "d16") {
                rhs = "memory_map.get_u16(cpu.pc + 1)";
            } else if (loadMatch[2] == "sp") {
                rhs = "cpu.sp";
            } else if (loadMatch[2] == "a16") {
                rhs = "memory_map.get(memory_map.get_u16(cpu.pc + 1))";
            } else if (!name.includes("(")) {

                rhs = `cpu.registers.${loadMatch[2]}()`;
            } else {
                rhs = `memory_map.get(cpu.registers.${loadMatch[2].replace(/\W/g, "")}())`;
            }

        } else {
            rhs = `cpu.registers.${loadMatch[2]}`;


            if (loadMatch[1] == "a16") {
                rhs += " as u16";
            }
        }

        let result;

        if (loadMatch[1].length > 1) {

            if (loadMatch[1] == "sp") {
                return `cpu.sp = ${rhs};`;
            }

            let setterFunc;

            if (loadMatch[2] == "d16" || loadMatch[1] == "a16") {
                setterFunc = "set_u16";
            } else {
                setterFunc = "set";
            }

            let lhs;

            if (loadMatch[1] == "a16") {
                lhs = "memory_map.get_u16(cpu.pc + 1)";
            } else {
                lhs = `cpu.registers.${loadMatch[1].replace(/\W/g, "")}()`;
            }

            if (name.includes("(")) {
                result = `memory_map.${setterFunc}(${lhs}, ${rhs});`;
            } else {
                result = `cpu.registers.set_${loadMatch[1]}(${rhs});`;
            }

        } else {
            result = `cpu.registers.${loadMatch[1]} = ${rhs};`;
        }

        const signRegex = /(\+|-)+/;
        
        const handleSign = (match) => {

            const signMatch = match.match(signRegex);

            if (!signMatch) {
                return result;
            }

            return `${result}\n    cpu.registers.set_hl(cpu.registers.hl() ${signMatch[1]} 1);`;
        };

        result = handleSign(loadMatch[1]);
        result = handleSign(loadMatch[2]);

        return result;
    }

    return "";
}


function instructionFunction(name, lengthInBytes, durationInCycles, flagsAffected, functionName) {

    const durationCycleSplit = durationInCycles.split("/");
    let returningCycle = durationCycleSplit[durationCycleSplit.length - 1];

    return `
/// ${name} <br>
///  Length in bytes: ${lengthInBytes} <br>
///  Duration in cycles: ${durationInCycles} <br>
///  Flags affected: ${flagsAffected} 
pub fn ${functionName}(cpu: &mut Cpu, memory_map: &mut MemoryMap) -> u8 {
    ${getFunctionContent(name)}
    ${returningCycle}
}\n`
}

module.exports = { instructionFile, instructionFunction, instructionFunctionFile }
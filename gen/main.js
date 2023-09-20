
const fs = require("fs");
const { JSDOM } = require("jsdom");

const path = require("path");
const { instructionFile, instructionFunction, instructionFunctionFile } = require("./templates");

const opcodeFile = fs.readFileSync(path.join(__dirname, "./opcodes.html")).toString();

const { document } = (new JSDOM(opcodeFile)).window;

const instructionFunctions = { misc: "", jump: "", load: "", arithmetic: "", bit: "" };

function processTable(tableId) {

    const opcodeTable = document.getElementById(tableId);

    const cellRegex = /(.*)<br>([0-9]*)&nbsp;&nbsp;([0-9]*\/?[0-9]*)<br>(.*)/;

    let instructions = "";

    for (let i = 1; i < opcodeTable.rows.length; ++i) {

        const row = opcodeTable.rows[i];

        for (let j = 1; j < row.cells.length; ++j) {

            const cellValue = row.cells[j].innerHTML;

            const opcode = (((i - 1) << 4) | (j - 1)).toString(16).padStart(2, '0').toUpperCase();

            const match = cellValue.match(cellRegex);

            if (match) {

                let functionName = 
                    match[1]
                    .replace(/(\s+|,+)/g, "_")
                    .replace("+", "_plus")
                    .replace("-", "_minus")
                    .replace(")", "_addr")
                    .replace("(", "")
                    .toLowerCase();


                const instructionName = match[1];
                let lengthInBytes = match[2];
                const durationInCycles = match[3];
                const flagsAffected = match[4];
                
                // https://stackoverflow.com/questions/41353869/length-of-instruction-ld-a-c-in-gameboy-z80-processor
                // There is a error in the Pastraiser opcode table that is 
                // LD (C) A and LD A (C) instructions are written as 2 bytes long instead of 1. 
                // Besides that STOP 0 instructions lengthInBytes also should be 1 explained above.
                if (instructionName === "LD (C),A" || instructionName === "LD A,(C)" || instructionName === "STOP 0") {
                    lengthInBytes = 1;
                }

                const backgroundColor = row.cells[j].bgColor;

                let instructionType;

                if (backgroundColor === "#ff99cc") {
                    instructionType = "misc";
                } else if (backgroundColor === "#ffcc99") {
                    instructionType = "jump";
                } else if (backgroundColor === "#ccccff" || backgroundColor === "#ccffcc") {
                    instructionType = "load";
                } else if (backgroundColor === "#ffff99" || backgroundColor === "#ffcccc") {
                    instructionType = "arithmetic";
                } else if (backgroundColor === "#80ffff") {
                    instructionType = "bit";
                }

                instructionFunctions[instructionType] += instructionFunction(opcode, instructionName, lengthInBytes, durationInCycles, flagsAffected, functionName);

                instructions += `    Instruction::new("${instructionName}", ${lengthInBytes}, ${instructionType}::${functionName}),\n`;
            } else {
                instructions += '    UNDEFINED,\n'
            }
        }
    }

    return instructions;
}

const instructions = processTable('opcodeTable');
const prefixCBInstructions = processTable('prefixCBTable');

const instructionsDirPath = path.join(__dirname, "../src/emulator/instructions/");

fs.writeFileSync(path.join(instructionsDirPath, "./mod.rs"), instructionFile(instructions, prefixCBInstructions));


// Regex to find and capture function contents.
const functionContentsRegex = /pub fn (.*)\(.*\{([\s\S]*?)\}(?:(?:[\n\r]*\/\/\/)|[\n\r]*$)/g;

for (const instructionType in instructionFunctions) {

    const filePath = path.join(instructionsDirPath, instructionType + ".rs");
    let file = instructionFunctionFile(instructionFunctions[instructionType]);

    // Dont overwrite everything if these file already exist.
    try {
        const oldFile = fs.readFileSync(filePath).toString();

        let matches;
        let functionContents = [];
    
        while (matches = functionContentsRegex.exec(oldFile)) {
            // First group is the name of the function.
            // Second group is the content of the function.
            functionContents.push([matches[1], matches[2]]);
        }
    
        functionContents.forEach(functionContent => {
    
            // Create a regex to replace new files contents with the old files contents.
            const replaceRegex = new RegExp(`(pub fn ${functionContent[0]}\\(.*\{)((?:\n|.)*?)(\}\n)`);
                
            file = file.replace(replaceRegex, `$1${functionContent[1]}$3`);
        });

    } catch (error) {}

    fs.writeFileSync(filePath, file);
}


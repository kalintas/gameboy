
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

            const opcode = (((i - 1) << 4) | (j - 1)).toString(16);

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
                const lengthInBytes = match[2];
                const durationInCycles = match[3];
                const flagsAffected = match[4];

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

                instructionFunctions[instructionType] += instructionFunction(instructionName, lengthInBytes, durationInCycles, flagsAffected, functionName);

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

for (const instructionType in instructionFunctions) {

    const filePath = path.join(instructionsDirPath, instructionType + ".rs");
    const file = instructionFunctionFile(instructionFunctions[instructionType]);

    fs.writeFileSync(filePath, file);
}


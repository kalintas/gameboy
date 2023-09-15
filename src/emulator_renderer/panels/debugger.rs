use imgui::{SelectableFlags, StyleColor};

use crate::emulator::{cpu::Cpu, Emulator};

use super::Panel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Breakpoint {
    row: i32,
    pub pointer: u16,
}

pub struct DebuggerPanel {
    strings: Vec<String>,

    pub breakpoints: Vec<Breakpoint>,

    continued_breakpoint: Option<Breakpoint>,
    old_toggled_breakpoint: Option<Breakpoint>,
    pub toggled_breakpoint: Option<Breakpoint>,

    runinng_next_line: bool,
}

impl DebuggerPanel {

    pub fn new() -> Self {
        let mut strings = Vec::with_capacity(10000);

        for i in 0..0x10000 {
            strings.push(format!("{:04x}: 00             NOP", i));
        }

        Self {
            strings,
            breakpoints: Vec::new(),

            continued_breakpoint: None,
            old_toggled_breakpoint: None,
            toggled_breakpoint: None,

            runinng_next_line: false,
        }
    }

    pub fn cycle(&mut self, emulator: &mut Emulator) {

        if self.toggled_breakpoint.is_none() {

            let breakpoint = self
                .breakpoints
                .iter()
                .position(|point| point.pointer == emulator.cpu.pc);

            if let Some(breakpoint_index) = breakpoint {

                self.toggle_breakpoint(self.breakpoints[breakpoint_index]);
            }
        }
        
        if self.toggled_breakpoint.is_none() {

            // TODO: works only in Vsync
            while emulator.cpu.clock_cycles < 69905 {
                emulator.cycle();
            }

            emulator.cpu.clock_cycles = 0;

        } else if self.runinng_next_line {

            emulator.cycle();

            self.pause(&emulator);
            
            self.runinng_next_line = false;
        }
    }

    pub fn toggle_breakpoint(&mut self, new_breakpoint: Breakpoint) {

        if self.continued_breakpoint.is_some_and(|last_breakpoint| last_breakpoint.pointer == new_breakpoint.pointer) {
            self.continued_breakpoint = None;
            return;
        }

        self.toggled_breakpoint = Some(new_breakpoint);
    }

    pub fn continue_execution(&mut self) {
        // Save the last breakpoint to make sure to not hit the same breakpoint after continue.
        self.continued_breakpoint = self.toggled_breakpoint; 
        self.toggled_breakpoint = None;
    }

    pub fn run_to_next_line(&mut self) {

        if self.toggled_breakpoint.is_some() {
            self.runinng_next_line = true;
        }
    }

    pub fn pause(&mut self, emulator: &Emulator) {

        self.toggled_breakpoint = Some(Breakpoint { row: self.get_pc_row(emulator), pointer: emulator.cpu.pc });
    }

    fn get_pc_row(&self, emulator: &Emulator) -> i32 {

        let mut row = 0;

        let mut pointer: u16 = 0;

        while pointer < emulator.cpu.pc {
            pointer += Cpu::decode(pointer, &emulator.memory_map).length as u16;
            row += 1;
        }

        row
    }
}

impl Panel for DebuggerPanel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]) {
        if changes.is_empty() {
            return;
        }

        self.strings.clear();

        let mut index: usize = 0;

        while index < 0x10000 {
            let instruction = Cpu::decode(index as u16, &emulator.memory_map);

            let mut line = format!("{:04x}:", index);

            for i in 0..3 {
                if i < instruction.length as usize {
                    line = format!(
                        "{} {:02x}",
                        line,
                        emulator.memory_map.get((index + i) as u16)
                    );
                } else {
                    line = format!("{}   ", line);
                }
            }   
            
            let mut instruction_name = instruction.name.to_string();

            let mut replace_signature = |signature: &'static str, bytes: usize| {
                
                if instruction.name.contains(signature) {

                    // // Instrcutions with relavite data format
                    // if instruction_name.starts_with("ADD SP") || 
                    // instruction_name.starts_with("JR") ||
                    // instruction_name.starts_with("LD HL,SP") {

                    // }

                    let mut data: u16 = 0;

                    for i in 0..bytes {

                        data = (data << 8) | emulator.memory_map.get((index + (instruction.length as usize) - i - 1) as u16) as u16;
                    }
                    
                    instruction_name = instruction.name.replace(signature, format!("0x{:02X}", data).as_str());

                    true
                } else {
                    false
                }
            };

            let _ = 
                replace_signature("d16", 2) ||
                replace_signature("a16", 2) ||
                replace_signature("r8", 1) ||
                replace_signature("d8", 1) ||
                replace_signature("a8", 1);

            self.strings
                .push(format!("{}       {}", line, instruction_name));

            index += instruction.length as usize;
        }
    }

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut Emulator, width: f32, height: f32) {
        ui.window("Debugger")
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .menu_bar(true)
            .position([width * 0.5, 19.0], imgui::Condition::Always)
            .size([width * 0.5, height * 0.5], imgui::Condition::Always)
            .build(|| {
                ui.set_window_font_scale(1.2);

                let scroll_to_row = |row| {
                    ui.set_scroll_y((row - 7) as f32 * ui.current_font_size());
                };

                ui.menu_bar(|| {
                    // Continue button
                    if ui.arrow_button("##1", imgui::Direction::Right)
                        || ui.is_key_pressed(imgui::Key::F5)
                    {
                        self.continue_execution();
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Continue(F5)");
                        });
                    }
                    // Run to next line button
                    ui.same_line();
                    if ui.arrow_button("##2", imgui::Direction::Down)
                        || ui.is_key_pressed(imgui::Key::F10)
                    {
                        self.run_to_next_line();
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Run to next line(F10)");
                        });
                    }

                    // Pause button
                    ui.same_line();
                    if ui.button("||") || ui.is_key_pressed(imgui::Key::F6) {
                        self.pause(emulator);
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Pause(F6)");
                        });
                    }

                    // Go to PC button
                    ui.same_line();
                    if ui.button("PC") {
                        scroll_to_row(self.get_pc_row(emulator));
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Go to PC");
                        })
                    }
                    
                    // // Clear breakpoints button
                    // ui.same_line();
                    // if ui.button("Clear all breakpoints") {
                    //     self.breakpoints = Vec::new();
                    // }

                });

                if let Some(breakpoint) = self.toggled_breakpoint {

                    if self.old_toggled_breakpoint.map_or(true, |old_breakpoint: Breakpoint| breakpoint.row != old_breakpoint.row) {

                        scroll_to_row(breakpoint.row);
                        self.old_toggled_breakpoint = self.toggled_breakpoint;
                    }
                }

                // Use a list clipper for efficient rendering.
                // This will only render visible lines of the text.
                let clipper = imgui::ListClipper::new(self.strings.len() as i32)
                    .items_height(ui.current_font_size())
                    .begin(ui);

                clipper.iter().for_each(|current_row| {
                    let breakpoint = self
                        .breakpoints
                        .iter()
                        .position(|point| point.row == current_row);
                    let mut text_color = None;

                    if breakpoint.is_some() {
                        text_color =
                            Some(ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.0, 1.0]));
                    }

                    if ui.selectable_config(&self.strings[current_row as usize])
                        .selected(self.toggled_breakpoint.is_some_and(|breakpoint| breakpoint.row == current_row))
                        .build() {
                        if let Some(breakpoint_index) = breakpoint {
                            self.breakpoints.remove(breakpoint_index);
                        } else {
                            let mut pointer = 0;

                            for _ in 0..current_row {
                                pointer += Cpu::decode(pointer, &emulator.memory_map).length as u16;
                            }

                            self.breakpoints.push(Breakpoint {
                                row: current_row,
                                pointer,
                            });
                        }
                    }

                    if let Some(color) = text_color {
                        color.pop();
                    }
                });
            });
    }
}

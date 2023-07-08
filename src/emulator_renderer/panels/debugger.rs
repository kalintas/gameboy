use imgui::{SelectableFlags, StyleColor};

use crate::emulator::{instructions::{INSTRUCTIONS}, Emulator};

use super::Panel;

#[derive(Debug, Clone, Copy)]
pub struct Breakpoint {
    row: i32,
    pub pointer: u16,
}

pub struct DebuggerPanel {
    strings: Vec<String>,

    pub breakpoints: Vec<Breakpoint>,

    old_toggled_breakpoint: Option<Breakpoint>,
    pub toggled_breakpoint: Option<Breakpoint>,

    runinng_next_line: bool,
}

impl DebuggerPanel {

    pub fn new() -> Self {
        let mut strings = Vec::with_capacity(0x10000);

        for i in 0..0x10000 {
            strings.push(format!("{:04x}: 00             NOP", i));
        }

        Self {
            strings,
            breakpoints: Vec::new(),

            old_toggled_breakpoint: None,
            toggled_breakpoint: Some(Breakpoint { row: 195, pointer: 0x100 }),

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
            emulator.cycle();
        } else if self.runinng_next_line {

            emulator.cycle();

            self.pause(&emulator);
            
            self.runinng_next_line = false;
        }
    }

    pub fn toggle_breakpoint(&mut self, new_breakpoint: Breakpoint) {

        if self.old_toggled_breakpoint.is_some_and(|old_breakpoint| old_breakpoint.pointer == new_breakpoint.pointer) {
            return;
        }

        self.toggled_breakpoint = Some(new_breakpoint);
    }

    pub fn continue_execution(&mut self) {
        self.toggled_breakpoint = None;
    }

    pub fn run_to_next_line(&mut self) {

        self.runinng_next_line = true;
    }

    pub fn pause(&mut self, emulator: &Emulator) {

        let mut row = 0;

        let mut pointer = 0;

        while pointer < emulator.cpu.pc {
            pointer += INSTRUCTIONS[emulator.memory_map.get(pointer) as usize].length as u16;
            row += 1;
        }

        assert_eq!(pointer, emulator.cpu.pc);

        self.toggled_breakpoint = Some(Breakpoint { row, pointer });
    }
}

impl Panel for DebuggerPanel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]) {
        if changes.is_empty() {
            return;
        }

        self.strings = Vec::new();

        let mut index: usize = 0;

        while index < 0x10000 {
            let instruction = INSTRUCTIONS[emulator.memory_map.get(index as u16) as usize];

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

            self.strings
                .push(format!("{}       {}", line, instruction.name));

            index += instruction.length as usize;
        }
    }

    fn render(&mut self, ui: &imgui::Ui, emulator: &Emulator, width: f32, height: f32) {
        ui.window("Debugger")
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .menu_bar(true)
            .position([width * 0.5, 19.0], imgui::Condition::Always)
            .size([width * 0.5, height * 0.5], imgui::Condition::Always)
            .build(|| {
                ui.set_window_font_scale(1.2);

                ui.menu_bar(|| {
                    // Continue button
                    if ui.arrow_button("##1", imgui::Direction::Right)
                        || ui.is_key_down(imgui::Key::F5)
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
                        || ui.is_key_down(imgui::Key::F10)
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
                    if ui.button("||") || ui.is_key_down(imgui::Key::F6) {
                        self.pause(emulator);
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Pause(F6)");
                        });
                    }
                    
                    // // Clear breakpoints button
                    // ui.same_line();
                    // if ui.button("Clear all breakpoints") {
                    //     self.breakpoints = Vec::new();
                    // }

                });

                if let Some(breakpoint) = self.toggled_breakpoint {

                    if self.old_toggled_breakpoint.map_or(true, |old_breakpoint| breakpoint.row != old_breakpoint.row) {

                        ui.set_scroll_y((breakpoint.row - 7) as f32 * ui.current_font_size());
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
                                pointer += INSTRUCTIONS[emulator.memory_map.get(pointer) as usize]
                                    .length as u16;
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

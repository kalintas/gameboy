use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};

use imgui::StyleColor;
use strum::IntoEnumIterator;

use crate::emulator::{cpu::Cpu, memory_map::Io, Emulator};

use super::{GoToLinePopup, Panel};

// BIG TODO:
// Disassembly could end up wrong in some cases.
// Do we need to store all Strings?
// Find a way to create the display on the fly. -> This will solve all updating costs and will make it real-time updatable
// Currently it works but this update is required immediately.

struct DebuggerWindow {
    opened: bool,
    current_item: i32,
    strings: Vec<String>,

    name: &'static str,
    size: [f32; 2],
}

impl DebuggerWindow {
    fn new(name: &'static str, size: [f32; 2]) -> Self {
        Self {
            opened: false,
            current_item: 0,

            strings: Vec::new(),
            name,
            size,
        }
    }

    fn render(&mut self, ui: &imgui::Ui, func: impl FnOnce(i32, &mut Vec<String>)) {
        self.opened |= ui.button(self.name);

        if self.opened {
            ui.window(self.name)
                .opened(&mut self.opened)
                .size(self.size, imgui::Condition::FirstUseEver)
                .build(|| {
                    let strings: Vec<&str> = self.strings.iter().map(|s| &**s).collect();

                    let window_size = ui.window_size();

                    ui.set_next_item_width(window_size[0] - 15.0);

                    ui.list_box("###breakpoints", &mut self.current_item, &strings, 10);

                    func(self.current_item, &mut self.strings);
                });
        }
    }
}

struct MemoryWatchWindow {
    debugger_window: DebuggerWindow,

    current_address: i32,
    current_io_address: String,
    current_value: i32,
    value_dont_care: bool,
}

impl MemoryWatchWindow {
    fn new() -> Self {
        Self {
            debugger_window: DebuggerWindow::new("Memory Watch", [300.0, 320.0]),

            current_address: 0,
            current_io_address: String::new(),
            current_value: 0,
            value_dont_care: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Breakpoint {
    row: i32,
    pub pointer: u16,
}

pub struct DebuggerPanel {
    strings: Vec<String>,

    breakpoints: Vec<Breakpoint>,

    continued_breakpoint: Option<Breakpoint>,
    old_toggled_breakpoint: Option<Breakpoint>,
    pub toggled_breakpoint: Option<Breakpoint>,

    runinng_next_line: bool,
    clock_timer: Instant,
    update_required: bool,

    go_to_line_popup: GoToLinePopup,

    breakpoints_window: DebuggerWindow,
    memory_watch_window: MemoryWatchWindow,
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
            clock_timer: Instant::now(),
            update_required: true,

            go_to_line_popup: GoToLinePopup::new("Go To Line###debugger"),

            breakpoints_window: DebuggerWindow::new("Breakpoints", [390.0, 250.0]),
            memory_watch_window: MemoryWatchWindow::new(),
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
            let now = Instant::now();
            let elapsed_time = (now - self.clock_timer).as_secs_f32();

            emulator.debug_cycle(
                elapsed_time,
                |pc| self.breakpoints.iter().any(|point| point.pointer == pc),
            );

            if emulator.memory_map.triggered_watch.is_some() {
                self.pause(emulator);
            }

            self.clock_timer = now;
        } else if self.runinng_next_line {
            emulator.cycle_once();

            self.pause(&emulator);

            self.runinng_next_line = false;
        }
    }

    pub fn toggle_breakpoint(&mut self, new_breakpoint: Breakpoint) {
        if self
            .continued_breakpoint
            .is_some_and(|last_breakpoint| last_breakpoint.pointer == new_breakpoint.pointer)
        {
            self.continued_breakpoint = None;
            return;
        }

        self.toggled_breakpoint = Some(new_breakpoint);
    }

    pub fn continue_execution(&mut self) {
        // Save the last breakpoint to make sure to not hit the same breakpoint after continue.
        self.continued_breakpoint = self.toggled_breakpoint;
        self.toggled_breakpoint = None;
        self.clock_timer = Instant::now();
    }

    pub fn run_to_next_line(&mut self) {
        if self.toggled_breakpoint.is_some() {
            self.runinng_next_line = true;
        }
    }

    pub fn pause(&mut self, emulator: &Emulator) {
        self.toggled_breakpoint = Some(Breakpoint {
            row: self.get_pc_row(emulator),
            pointer: emulator.cpu.pc,
        });

        if self.update_required {
            self.update_strings(emulator);
            self.update_required = false;
        }
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
        self.breakpoints_window.strings.clear();
    }

    pub fn dump_strings(&self) {
        let file = File::create("c:/Users/kerem/Desktop/disassembly_dump.txt").unwrap();
        let mut file = BufWriter::new(file);

        self.strings
            .iter()
            .for_each(|string| write!(file, "{}\n", string).unwrap());
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

    fn update_strings(&mut self, emulator: &Emulator) {
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
                        data = (data << 8)
                            | emulator
                                .memory_map
                                .get((index + (instruction.length as usize) - i - 1) as u16)
                                as u16;
                    }

                    instruction_name = instruction
                        .name
                        .replace(signature, format!("0x{:02X}", data).as_str());

                    true
                } else {
                    false
                }
            };

            let _ = replace_signature("d16", 2)
                || replace_signature("a16", 2)
                || replace_signature("r8", 1)
                || replace_signature("d8", 1)
                || replace_signature("a8", 1);

            self.strings
                .push(format!("{}       {}", line, instruction_name));

            index += instruction.length as usize;
        }
    }
}

impl Panel for DebuggerPanel {
    fn update(&mut self, emulator: &Emulator, changes: &HashMap<u16, u8>) {
        // TODO: make this work in realtime.

        // Debugger panel only updated when the debugger is paused.
        // This is done because updating the panel is very expensive.

        if !changes.is_empty() {
            if self.toggled_breakpoint.is_some() {
                self.update_strings(emulator);
            } else {
                self.update_required = true;
            }
        }
    }

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut Emulator, width: f32, height: f32) {
        ui.window("Debugger")
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .menu_bar(true)
            .bring_to_front_on_focus(false)
            .position([width * 0.5, 19.0], imgui::Condition::Always)
            .size([width * 0.5, height * 0.5], imgui::Condition::Always)
            .build(|| {
                ui.set_window_font_scale(1.2);

                let mut row_scroll = None;

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
                        row_scroll = Some(self.get_pc_row(emulator));
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text("Go to PC");
                        })
                    }

                    ui.same_line();

                    // Breakpoints window
                    self.breakpoints_window.render(ui, |current_item, strings| {
                        if current_item >= 0 {
                            let current_item = current_item as usize;

                            if current_item < self.breakpoints.len() {
                                if ui.button("Delete Breakpoint") {
                                    self.breakpoints.remove(current_item);
                                    strings.remove(current_item);
                                }

                                ui.same_line();

                                if ui.button("Show") {
                                    row_scroll = Some(self.breakpoints[current_item].row);
                                }
                            }
                        }
                    });

                    // Memory Watch Window
                    let mut deleted_string_index = None;

                    self.memory_watch_window
                        .debugger_window
                        .render(ui, |current_item, strings| {
                            if current_item >= 0 {
                                let current_item = current_item as usize;

                                if current_item < emulator.memory_map.memory_watches.len() {
                                    if ui.button("Delete Memory Watch") {
                                        emulator.memory_map.memory_watches.remove(current_item);
                                        deleted_string_index = Some(current_item);
                                    }
                                }
                            }

                            let width_token = ui.push_item_width(100.0);
                            if super::hex_input_u16(
                                ui,
                                "Address",
                                &mut self.memory_watch_window.current_address,
                            ) {
                                if let Some(io) = Io::iter().find(|io| {
                                    *io as i32 == self.memory_watch_window.current_address
                                }) {
                                    // Check if there is an io register by this address.
                                    self.memory_watch_window.current_io_address =
                                        io.as_ref().to_owned();
                                } else {
                                    self.memory_watch_window.current_io_address = String::new();
                                }
                            }
                            ui.same_line();

                            ui.text("or");
                            ui.same_line();
                            if ui
                                .input_text("Io", &mut self.memory_watch_window.current_io_address)
                                .build()
                            {
                                if let Some(io) = Io::iter().find(|io| {
                                    io.as_ref()
                                        == self
                                            .memory_watch_window
                                            .current_io_address
                                            .to_uppercase()
                                }) {
                                    self.memory_watch_window.current_address = io as i32;
                                } else {
                                    // Not a valid io name. Clear the string.
                                    self.memory_watch_window.current_io_address = String::new();
                                    self.memory_watch_window.current_address = 0;
                                }
                            }

                            super::hex_input_u8(
                                ui,
                                "Value",
                                &mut self.memory_watch_window.current_value,
                            );
                            ui.same_line();
                            width_token.end();
                            ui.checkbox(
                                "Don't Care",
                                &mut self.memory_watch_window.value_dont_care,
                            );

                            if self.memory_watch_window.value_dont_care {
                                self.memory_watch_window.current_value = 0;
                            }

                            if ui.button("Add Memory Watch") {
                                let address = self.memory_watch_window.current_address;
                                let value = self.memory_watch_window.current_value;

                                let (value, value_string) =
                                    if self.memory_watch_window.value_dont_care {
                                        (None, "XX".to_owned())
                                    } else {
                                        (Some(value as u8), format!("{:2x}", value))
                                    };

                                emulator
                                    .memory_map
                                    .memory_watches
                                    .push((address as u16, value));

                                let io_string =
                                    if self.memory_watch_window.current_io_address.is_empty() {
                                        String::new()
                                    } else {
                                        format!(
                                            "({})",
                                            self.memory_watch_window
                                                .current_io_address
                                                .to_uppercase()
                                        )
                                    };

                                strings.push(format!(
                                    "Address: {:04x}{}, Value: {}",
                                    address, io_string, value_string
                                ));

                                self.memory_watch_window.current_address = 0;
                                self.memory_watch_window.current_value = 0;
                                self.memory_watch_window.current_io_address = String::new();
                            }
                        });

                    if let Some(deleted_string_index) = deleted_string_index {
                        self.memory_watch_window
                            .debugger_window
                            .strings
                            .remove(deleted_string_index);
                    }

                    // // Clear breakpoints button
                    // ui.same_line();
                    // if ui.button("Clear all breakpoints") {
                    //     self.breakpoints = Vec::new();
                    // }
                });

                if let Some(breakpoint) = self.toggled_breakpoint {
                    if self
                        .old_toggled_breakpoint
                        .map_or(true, |old_breakpoint: Breakpoint| {
                            breakpoint.row != old_breakpoint.row
                        })
                    {
                        row_scroll = Some(breakpoint.row);
                        self.old_toggled_breakpoint = self.toggled_breakpoint;
                    }
                }

                // Go to line popup
                self.go_to_line_popup.render(
                    ui,
                    |scroll| {
                        let line = (self.strings.len() as f32 * scroll) as u32;
                        let mut pointer = 0;

                        for _ in 0..line {
                            pointer +=
                                Cpu::decode(pointer as _, &emulator.memory_map).length as u32;
                        }

                        pointer
                    },
                    |pointer| {
                        let mut line = 0;
                        let mut current_pointer = 0;

                        while current_pointer < pointer {
                            current_pointer +=
                                Cpu::decode(current_pointer as _, &emulator.memory_map).length
                                    as u32;
                            line += 1;
                        }

                        line as f32 / self.strings.len() as f32
                    },
                );

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

                    if ui
                        .selectable_config(&self.strings[current_row as usize])
                        .selected(
                            self.toggled_breakpoint
                                .is_some_and(|breakpoint| breakpoint.row == current_row),
                        )
                        .build()
                    {
                        if let Some(breakpoint_index) = breakpoint {
                            self.breakpoints.remove(breakpoint_index);
                            self.breakpoints_window.strings.remove(breakpoint_index);
                        } else {
                            let mut pointer = 0;

                            for _ in 0..current_row {
                                pointer += Cpu::decode(pointer, &emulator.memory_map).length as u16;
                            }

                            self.breakpoints.push(Breakpoint {
                                row: current_row,
                                pointer,
                            });

                            let breakpoint_string = format!(
                                "Line: {}, Address: {:04x}, Instruction: {}",
                                current_row,
                                pointer,
                                Cpu::decode(pointer, &emulator.memory_map).name
                            );

                            self.breakpoints_window.strings.push(breakpoint_string);
                        }
                    }

                    if let Some(color) = text_color {
                        color.pop();
                    }
                });

                // Scroll to desired location.

                if let Some(row) = row_scroll {
                    ui.set_scroll_y((row - 7) as f32 * ui.current_font_size());
                }
            });
    }
}

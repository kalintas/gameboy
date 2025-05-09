use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    ptr::copy_nonoverlapping,
    time::Instant,
};

use imgui::StyleColor;
use strum::IntoEnumIterator;

use gameboy::{
    instructions::{Instruction, MAX_INSTRUCTION_NAME_LENGTH},
    memory_map::Io,
    Gameboy,
};

use super::{GoToLinePopup, Panel};

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
    current_start_row: i32,
    current_start_address: u16,
    line_count: i32,

    breakpoints: Vec<Breakpoint>,

    continued_breakpoint: Option<Breakpoint>,
    old_toggled_breakpoint: Option<Breakpoint>,
    pub toggled_breakpoint: Option<Breakpoint>,

    runinng_next_line: bool,
    clock_timer: Instant,

    go_to_line_popup: GoToLinePopup,

    breakpoints_window: DebuggerWindow,
    memory_watch_window: MemoryWatchWindow,
}

impl DebuggerPanel {
    pub fn new() -> Self {
        Self {
            current_start_row: 0,
            current_start_address: 0,
            line_count: 0,

            breakpoints: Vec::new(),

            continued_breakpoint: None,
            old_toggled_breakpoint: None,
            toggled_breakpoint: None,

            runinng_next_line: false,
            clock_timer: Instant::now(),

            go_to_line_popup: GoToLinePopup::new("Go To Line###debugger"),

            breakpoints_window: DebuggerWindow::new("Breakpoints", [390.0, 250.0]),
            memory_watch_window: MemoryWatchWindow::new(),
        }
    }

    pub fn cycle(&mut self, emulator: &mut Gameboy) {
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
            let elapsed_time = now - self.clock_timer;

            emulator.debug_cycle(elapsed_time, |emulator| {
                self.breakpoints
                    .iter()
                    .any(|point| point.pointer == emulator.cpu.pc)
            });

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

    pub fn pause(&mut self, emulator: &Gameboy) {
        self.toggled_breakpoint = Some(Breakpoint {
            row: self.get_line_at_address(emulator, emulator.cpu.pc as _),
            pointer: emulator.cpu.pc,
        });

        self.line_count = self.get_line_at_address(emulator, 0x10000);
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
        self.breakpoints_window.strings.clear();
    }

    pub fn dump_to_file(
        &self,
        path: impl AsRef<Path>,
        emulator: &Gameboy,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut file = BufWriter::new(file);

        let mut pointer: usize = 0;

        while pointer < 0x10000 {
            let instruction = emulator.decode_instr(pointer as _);

            let mut buffer = [' ' as u8; 30 + MAX_INSTRUCTION_NAME_LENGTH];
            Self::format_instruction_name(&mut buffer, emulator, instruction, pointer as _);
            buffer[buffer.len() - 1] = '\n' as u8;
            file.write(&buffer)?;

            pointer += instruction.length as usize;
        }

        Ok(())
    }

    fn get_line_at_address(&mut self, emulator: &Gameboy, address: usize) -> i32 {
        let mut line = 0;
        let mut pointer: usize = 0;

        while pointer < address {
            let instruction = emulator.decode_instr(pointer as _);
            line += 1;
            pointer += instruction.length as usize;
        }

        line
    }

    fn get_address_at_line(&mut self, emulator: &Gameboy, line: i32) -> usize {
        let mut pointer = 0;
        for _ in 0..line {
            let instruction = emulator.decode_instr(pointer as u16);
            pointer += instruction.length as usize;
        }

        pointer
    }

    fn format_instruction_name<'a>(
        buffer: &'a mut [u8],
        emulator: &Gameboy,
        instruction: Instruction,
        address: u16,
    ) -> &'a str {
        assert!(buffer.len() >= 21 + MAX_INSTRUCTION_NAME_LENGTH);

        super::format_in_place(buffer, address, 0);
        buffer[4] = ':' as u8;

        for i in 0..instruction.length as _ {
            super::format_in_place(
                buffer,
                emulator.memory_map.get(address + i),
                6 + i as usize * 3,
            );
        }

        unsafe {
            copy_nonoverlapping(
                instruction.name.as_bytes().as_ptr(),
                buffer.as_mut_ptr().add(21),
                instruction.name.len(),
            );
        }

        let mut replace_signature = |signature: &'static str, bytes: u16| {
            if let Some(signature_location) = instruction.name.find(signature) {
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
                            .get(address + instruction.length as u16 - i - 1)
                            as u16;
                }

                let index = 21 + signature_location;

                let difference = (bytes as usize * 2 + 2) - signature.len();

                for i in (signature_location + signature.len()..instruction.name.len()).rev() {
                    buffer[21 + i + difference] = buffer[21 + i];
                }

                buffer[index] = '0' as u8;
                buffer[index + 1] = 'x' as u8;

                if bytes == 1 {
                    super::format_in_place::<u8>(buffer, data as _, index + 2);
                } else {
                    super::format_in_place(buffer, data, index + 2);
                }

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

        unsafe { std::str::from_utf8_unchecked(buffer) }
    }
}

impl Panel for DebuggerPanel {
    fn update(&mut self, _: &Gameboy) {}

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut Gameboy, width: f32, height: f32) {
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
                        row_scroll = Some(self.get_line_at_address(emulator, emulator.cpu.pc as _));
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
                        let line = (self.line_count as f32 * scroll) as u32;
                        let mut pointer = 0;

                        for _ in 0..line {
                            pointer += emulator.decode_instr(pointer as _).length as u32;
                        }

                        pointer
                    },
                    |pointer| {
                        let mut line = 0;
                        let mut current_pointer = 0;

                        while current_pointer < pointer {
                            current_pointer +=
                                emulator.decode_instr(current_pointer as _).length as u32;
                            line += 1;
                        }

                        line as f32 / self.line_count as f32
                    },
                );

                // Use a list clipper for efficient rendering.
                // This will only render visible lines of the text.
                let clipper = imgui::ListClipper::new(self.line_count)
                    .items_height(ui.current_font_size())
                    .begin(ui);

                let mut is_first_row = true;

                let mut current_address = self.current_start_address;

                clipper.iter().for_each(|current_row| {
                    if is_first_row {
                        // User scrolled the list. Update start address and row.
                        if self.current_start_row != current_row {
                            self.current_start_address =
                                self.get_address_at_line(emulator, current_row) as u16;
                            self.current_start_row = current_row;
                        }

                        is_first_row = false;
                    } else if current_row == self.line_count {
                        // Reaching this branch means the value in the self.line_count is stale and should be updated.
                        self.line_count = self.get_line_at_address(emulator, 0x10000);
                    }

                    let breakpoint = self
                        .breakpoints
                        .iter()
                        .position(|point| point.pointer == current_address);
                    let mut text_color = None;

                    if breakpoint.is_some() {
                        text_color =
                            Some(ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.0, 1.0]));
                    }

                    let instruction = emulator.decode_instr(current_address);

                    let mut buffer = [' ' as u8; 30 + MAX_INSTRUCTION_NAME_LENGTH];

                    if ui
                        .selectable_config(Self::format_instruction_name(
                            &mut buffer,
                            emulator,
                            instruction,
                            current_address,
                        ))
                        .selected(
                            self.toggled_breakpoint
                                .is_some_and(|breakpoint| breakpoint.pointer == current_address),
                        )
                        .build()
                    {
                        if let Some(breakpoint_index) = breakpoint {
                            self.breakpoints.remove(breakpoint_index);
                            self.breakpoints_window.strings.remove(breakpoint_index);
                        } else {
                            let pointer = self.get_address_at_line(emulator, current_row) as u16;

                            self.breakpoints.push(Breakpoint {
                                row: current_row,
                                pointer,
                            });

                            let breakpoint_string = format!(
                                "Line: {}, Address: {:04x}, Instruction: {}",
                                current_row, pointer, instruction.name
                            );

                            self.breakpoints_window.strings.push(breakpoint_string);
                        }
                    }

                    current_address += instruction.length as u16;

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

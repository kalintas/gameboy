use std::{cell::RefCell, rc::Rc};

use imgui::InputTextFlags;
use sdl2::keyboard::Scancode;

use super::{Emulator, memory_strings::{MemoryString, DisassemblyString}, memory_map};

pub struct EmulatorRenderer {
    ctrl_g_pressed: bool,
    memory_window_scroll: i32,

    memory_string: MemoryString,
    disassembly_string: DisassemblyString,
}

impl EmulatorRenderer {
    pub fn new(emulator: &mut Emulator) -> Rc<RefCell<Self>> {

        let emulator_renderer = Rc::new(RefCell::new(Self {
            ctrl_g_pressed: false,
            memory_window_scroll: 0,

            memory_string: MemoryString::new(),
            disassembly_string: DisassemblyString::new()
        }));

        let emulator_renderer_clone = emulator_renderer.clone(); 
        let memory_map_clone = emulator.memory_map.clone();

        emulator.memory_map.borrow_mut().set_on_change(move |address, value| {
            
            emulator_renderer_clone.borrow_mut().memory_string.update(address, value);

            // emulator_renderer_clone.borrow_mut().disassembly_string.update(&memory_map_clone.borrow());
        });

        emulator_renderer
    }

    // pub fn init(mut self, emulator: &mut Emulator) {
    //     emulator.memory_map.set_on_change(move |change| {

    //         if let Some((address, value)) = change {

    //             // println!("{}", self.memory_string.as_ref());

    //             self.memory_string.update(address, value);

    //         } else {

    //         }
    //     });
    // }

    pub fn render(&mut self, emulator: &mut Emulator) {
        let width = emulator.renderer.window_width as f32;
        let height = emulator.renderer.window_height as f32;

        let keyboard_state = emulator.renderer.event_pump.keyboard_state();

        self.ctrl_g_pressed = keyboard_state.is_scancode_pressed(Scancode::LCtrl)
            && keyboard_state.is_scancode_pressed(Scancode::G);

        emulator.renderer.render(|ui| {
            let imgui_window = |title, position, size| {
                ui.window(title)
                    .resizable(false)
                    .collapsible(false)
                    .movable(false)
                    .position(position, imgui::Condition::Always)
                    .size(size, imgui::Condition::Always)
            };

            imgui_window("Disassembly", [width * 0.5, 0.0], [width * 0.5, height * 0.5]).build(|| {
                ui.set_window_font_scale(1.2);

                // let strings = &emulator.memory_map.memory_string.disassembly.strings;

                // strings.iter().for_each(|line| ui.text(line));
            });

            imgui_window("Memory", [0.0, height * 0.5], [width * 2.0 / 3.0, height * 0.5]).build(|| {
                ui.set_window_font_scale(1.2);

                // if self.memory_window_scroll > 0 {
                //     ui.set_scroll_y((self.memory_window_scroll / 0x10) as f32 * 10.0);
                // }

                // ui.modal_popup_config("Jump to line")
                //     .resizable(false)
                //     .collapsible(false)
                //     .movable(false)
                //     .build(|| {

                //         ui.input_int("Line number", &mut self.memory_window_scroll)
                //             .auto_select_all(true)
                //             .step(0x10)
                //             .flags(InputTextFlags::CHARS_SCIENTIFIC)
                //             .build();

                //         if ui.button("Go") {
                //             ui.close_current_popup();
                //             self.memory_window_scroll = 0;
                //         }
                //     });

                // if ui.is_window_focused() && self.ctrl_g_pressed {
                //     ui.open_popup("Jump to line");
                // }
                ui.text(self.memory_string.as_ref());
            });

            imgui_window("Registers", [width * 2.0 / 3.0, height * 0.5], [width / 3.0, height * 0.5]).build(|| {
                ui.set_window_font_scale(1.2);

                ui.text(format!("pc: {:04x}", emulator.cpu.pc));
                ui.text(format!("sp: {:04x}", emulator.cpu.sp));
                ui.text(format!("af: {:04x}", emulator.cpu.registers.af()));
                ui.text(format!("bc: {:04x}", emulator.cpu.registers.bc()));
                ui.text(format!("de: {:04x}", emulator.cpu.registers.de()));
                ui.text(format!("hl: {:04x}", emulator.cpu.registers.hl()));

            });
        });
    }
}

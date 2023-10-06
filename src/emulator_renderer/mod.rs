use std::time::Instant;

use crate::emulator::ppu;

use self::panels::Panels;

use super::emulator::Emulator;
use super::renderer::{framebuffer::Framebuffer, Renderer};

mod panels;

use panels::Panel;
use rfd::FileDialog;
use sdl2::keyboard::Scancode;

pub struct EmulatorRenderer {
    running: bool,

    panels: Panels,

    renderer: Renderer,
}

impl EmulatorRenderer {
    pub fn new() -> Self {
        Self {
            running: true,

            panels: Panels::new(),

            renderer: Renderer::new("Gameboy", 800, 739).unwrap(),
        }
    }

    pub fn render(&mut self, emulator: &mut Emulator) {
        self.panels.debugger.pause(emulator);

        let framebuffer = Framebuffer::new();

        let mut seconds_timer = Instant::now();

        while self.running && !self.renderer.poll_events() {
            let keyboard_state = self.renderer.event_pump.keyboard_state();

            if keyboard_state.is_scancode_pressed(Scancode::LCtrl)
                && keyboard_state.is_scancode_pressed(Scancode::W)
            {
                break;
            }

            emulator.update_joypad_keys(
                self.panels.keyboard_map
                    .update_keys(keyboard_state.pressed_scancodes().collect()),
            );

            self.panels.debugger.cycle(emulator);

            // emulator.cycle();

            let now = Instant::now();

            if (now - seconds_timer).as_secs() >= 1 {
                if emulator.cpu.clock_cycles != 0 {
                    self.renderer
                        .window
                        .set_title(
                            format!(
                                "Gameboy {}%",
                                (emulator.cpu.clock_cycles as f32 / 4_194_304.0) * 100.0
                            )
                            .as_str(),
                        )
                        .unwrap();
                }

                emulator.cpu.clock_cycles = 0;
                seconds_timer = now;
            }

            panels::call_all_panels!(self.panels, update, emulator, &emulator.memory_map.changes);

            emulator.memory_map.changes.clear();

            // Render frame.
            self.renderer.clear_screen();

            framebuffer.update_buffer(
                ppu::SCREEN_WIDTH as _,
                ppu::SCREEN_HEIGHT as _,
                emulator.ppu.screen_buffer.as_ptr() as _,
                gl::RGBA8,
                gl::RGBA,
            );
            framebuffer.draw_buffer(
                (0, 0),
                (ppu::SCREEN_WIDTH as _, ppu::SCREEN_HEIGHT as _),
                (0, self.renderer.window_height as i32 - 19),
                (
                    self.renderer.window_width as i32 / 2,
                    (self.renderer.window_height as i32 - 19) / 2,
                ),
            );

            let width = self.renderer.window_width as f32;
            let height = (self.renderer.window_height - 19) as f32;

            let mut reset_emulator = false;

            self.renderer.render(|ui| {
                ui.main_menu_bar(|| {
                    ui.menu("File", || {
                        
                        if ui.menu_item("Load Cartidage") {
                            let file = FileDialog::new().pick_file();

                            if let Some(file_path) = file {

                                *emulator = Emulator::after_boot();
                                emulator.load_cartidge(file_path);
                                reset_emulator = true;
                            } 
                        }

                        if ui.menu_item("Reload Cartidage") {
                            *emulator = Emulator::after_boot();
                            emulator.load_cartidge("./roms/tetris.gb");
                            reset_emulator = true;
                        }

                        if ui.menu_item("Quit") {
                            self.running = false;
                            return;
                        }
                    });

                    ui.menu("Window", || {

                        let small_panel = |panel: &mut dyn Panel| {
                            if ui
                                .menu_item_config(panel.get_name())
                                .selected(panel.is_opened())
                                .build()
                            {
                                panel.set_opened(!panel.is_opened());
                            }
                        };

                        small_panel(&mut self.panels.io_map);
                        small_panel(&mut self.panels.keyboard_map);
                    });

                    ui.menu("Debug", || {
                        if ui.menu_item("Continue                F5") {
                            self.panels.debugger.continue_execution();
                        }

                        if ui.menu_item("Run To Next Line        F10") {
                            self.panels.debugger.run_to_next_line();
                        }

                        if ui.menu_item("Pause                   F6") {
                            self.panels.debugger.pause(emulator);
                        }

                        if ui.menu_item("Clear All Breakpoints") {
                            self.panels.debugger.breakpoints.clear();
                        }

                        if ui.menu_item("Dump disassembly") {
                            self.panels.debugger.dump_strings();
                        }
                    });

                    ui.show_metrics_window(&mut true);
                });

                panels::call_all_panels!(self.panels, render, ui, emulator, width, height);

                // ui.show_demo_window(&mut true);
            });

            if reset_emulator {
                self.panels = Panels::new();
                self.panels.debugger.pause(emulator);
            }
        }
    }
}

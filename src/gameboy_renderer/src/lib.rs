mod panels;
mod renderer;

use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use gameboy::{self, ppu};

use self::panels::Panels;

use gameboy::Gameboy;
use renderer::{framebuffer::Framebuffer, Renderer};

use panels::Panel;
use rfd::FileDialog;
use sdl2::keyboard::Scancode;

pub struct GameboyRenderer {
    running: bool,
    current_rom_path: PathBuf,
    current_boot_rom_path: Option<PathBuf>,

    panels: Panels,

    renderer: Renderer,
}

impl GameboyRenderer {
    pub fn new() -> Self {
        let renderer = Renderer::new("Gameboy", 800, 739).unwrap();
        let panels = Panels::new(); // Panels must be initialized after creating OpenGL context.

        Self {
            running: true,
            current_rom_path: PathBuf::from_str("./roms/zelda.gb").unwrap(),
            current_boot_rom_path: None,

            panels,
            renderer,
        }
    }

    pub fn run(&mut self) {
        let emulator = &mut Gameboy::after_boot();

        emulator.load_cartidge(&self.current_rom_path);

        self.run_with(emulator);
    }

    pub fn run_with(&mut self, emulator: &mut Gameboy) {
        self.panels.debugger.pause(emulator);

        let framebuffer = Framebuffer::new();

        let mut seconds_timer = Instant::now();

        while self.running && !self.renderer.poll_events() {
            // Process events.
            let keyboard_state = self.renderer.event_pump.keyboard_state();

            if keyboard_state.is_scancode_pressed(Scancode::LCtrl)
                && keyboard_state.is_scancode_pressed(Scancode::W)
            {
                break;
            }

            emulator.update_joypad_keys(
                self.panels
                    .keyboard_map
                    .update_keys(keyboard_state.pressed_scancodes().collect()),
            );

            emulator.memory_map.vram_changed = false;
            emulator.memory_map.oam_changed = false;

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

            panels::call_all_panels!(self.panels, update, emulator);

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
                    // (((ppu::SCREEN_WIDTH as f32) / (ppu::SCREEN_HEIGHT as f32)) * ((self.renderer.window_height as f32 - 19.0) * 0.5)) as i32
                    self.renderer.window_width as i32 / 2,
                    (self.renderer.window_height as i32 - 19) / 2,
                ),
            );

            let width = self.renderer.window_width as f32;
            let height = (self.renderer.window_height - 19) as f32;

            let mut reset_emulator = false;
            let mut enter_game_mode = false;

            self.renderer.render(|ui| {
                let small_panel = |panel: &mut dyn Panel| {
                    if ui
                        .menu_item_config(panel.get_name())
                        .selected(panel.is_opened())
                        .build()
                    {
                        panel.set_opened(!panel.is_opened());
                    }
                };

                ui.main_menu_bar(|| {
                    ui.menu("File", || {
                        if ui.menu_item("Load Cartidage") {
                            let file =
                                FileDialog::set_directory(FileDialog::new(), "./roms").pick_file();

                            if let Some(file_path) = file {
                                *emulator = Gameboy::after_boot();
                                emulator.load_cartidge(&file_path);
                                self.current_rom_path = file_path;
                                reset_emulator = true;
                            }
                        }

                        if ui.menu_item("Reload Cartidage") {
                            if let Some(boot_rom_path) = &self.current_boot_rom_path {
                                *emulator = Gameboy::new(boot_rom_path).unwrap();
                            } else {
                                *emulator = Gameboy::after_boot();
                            }

                            emulator.load_cartidge(&self.current_rom_path);

                            reset_emulator = true;
                        }

                        if ui.menu_item("Enter Game Mode        F11")
                            || ui.is_key_down(imgui::Key::F11)
                        {
                            enter_game_mode = true;
                        }

                        if ui.menu_item("Quit") {
                            self.running = false;
                            return;
                        }
                    });

                    ui.menu("Window", || {
                        small_panel(&mut self.panels.io_map);
                        small_panel(&mut self.panels.keyboard_map);
                        small_panel(&mut self.panels.bg_map);
                    });

                    ui.menu("Boot Rom", || {
                        if ui
                            .menu_item_config("No Boot Rom")
                            .selected(self.current_boot_rom_path.is_none())
                            .build()
                        {
                            self.current_boot_rom_path = None;
                        }

                        if let Some(boot_rom_path) = &self.current_boot_rom_path {
                            ui.menu_item_config(
                                boot_rom_path.file_name().unwrap().to_str().unwrap(),
                            )
                            .selected(true)
                            .build();
                        }

                        if ui.menu_item("Load Boot Rom") {
                            let file = FileDialog::set_directory(FileDialog::new(), "./roms/boot")
                                .pick_file();

                            if let Some(file_path) = file {
                                self.current_boot_rom_path = Some(file_path);
                            }
                        }
                    });

                    ui.menu("Debug", || {
                        if ui.menu_item("Continue                F5") {
                            self.panels.debugger.continue_execution();
                        }

                        if ui.menu_item("Run To Next Line       F10") {
                            self.panels.debugger.run_to_next_line();
                        }

                        if ui.menu_item("Pause                   F6") {
                            self.panels.debugger.pause(emulator);
                        }

                        if ui.menu_item("Clear All Breakpoints") {
                            self.panels.debugger.clear_breakpoints();
                        }
                    });

                    ui.menu("Tools", || {
                        if ui.menu_item("Dump disassembly") {
                            let path = FileDialog::new()
                                .set_file_name("disassembly_dump.txt")
                                .save_file();
                            if let Some(path) = path {
                                if let Err(error) =
                                    self.panels.debugger.dump_to_file(path, &emulator)
                                {
                                    ui.modal_popup("Cannot dump disassembly", || {
                                        ui.text(format!("Error: {}", error));
                                    });
                                }
                            }
                        }

                        if ui.menu_item("Dump memory") {
                            let path = FileDialog::new()
                                .set_file_name("memory_dump.txt")
                                .save_file();
                            if let Some(path) = path {
                                if let Err(error) = self.panels.memory.dump_to_file(path, &emulator)
                                {
                                    println!("{}", error);
                                    ui.modal_popup("Cannot dump memory", || {
                                        ui.text(format!("Error: {}", error));
                                    });
                                }
                            }
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

            if enter_game_mode {
                let (width, height) = (self.renderer.window_width, self.renderer.window_height);

                self.renderer.window.set_title("Gameboy").unwrap();

                self.run_game_mode(emulator, &framebuffer);

                if !self.running {
                    break;
                }
                self.renderer.resize(width, height);

                // TOOD: memory_map
                self.panels.debugger.pause(emulator);
            }
        }
    }

    fn run_game_mode(&mut self, emulator: &mut Gameboy, framebuffer: &Framebuffer) {
        // Resize window.
        self.renderer.resize(400, 360);

        let mut timer = Instant::now();

        while self.running && !self.renderer.poll_events() {
            // Process events.
            let keyboard_state = self.renderer.event_pump.keyboard_state();

            if keyboard_state.is_scancode_pressed(Scancode::LCtrl)
                && keyboard_state.is_scancode_pressed(Scancode::W)
            {
                self.running = false;
                return;
            }

            if keyboard_state.is_scancode_pressed(Scancode::Escape) {
                return;
            }

            emulator.update_joypad_keys(
                self.panels
                    .keyboard_map
                    .update_keys(keyboard_state.pressed_scancodes().collect()),
            );

            let now = Instant::now();
            let elapsed_time = now - timer;
            emulator.cycle(elapsed_time);
            timer = now;

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
                (0, self.renderer.window_height as i32),
                (self.renderer.window_width as i32, 0),
            );

            self.renderer.render(|_| {});
        }
    }
}

use std::time::Instant;

use crate::emulator::ppu;

use self::panels::debugger::DebuggerPanel;
use self::panels::memory::MemoryPanel;
use self::panels::registers::RegistersPanel;
use self::panels::io_map::IoMap;
use self::panels::keyboard_map::KeyboardMap;

use super::emulator::Emulator;
use super::renderer::{framebuffer::Framebuffer, Renderer};

mod panels;

use panels::Panel;
use sdl2::keyboard::Scancode;

pub struct EmulatorRenderer {
    running: bool,

    debugger_panel: DebuggerPanel,
    keyboard_map_panel: KeyboardMap,

    panels: Vec<Box<dyn Panel>>,
    small_panels: Vec<Box<dyn Panel>>,

    renderer: Renderer,
}

impl EmulatorRenderer {
    pub fn new() -> Self {

        Self {
            running: true,

            debugger_panel: DebuggerPanel::new(),
            keyboard_map_panel: KeyboardMap::new(),
            
            panels: Vec::new(),
            small_panels: Vec::new(),
            
            renderer: Renderer::new("Gameboy", 800, 739).unwrap(),
        }
    }
    
    fn set_default_panels(&mut self) {
        self.panels = vec![
            Box::new(MemoryPanel::new()) as Box<dyn Panel>,
            Box::new(RegistersPanel::new()) as Box<dyn Panel>,
        ];

        self.small_panels = vec![
            Box::new(IoMap::new()) as Box<dyn Panel>
        ];
    }

    pub fn render(&mut self, emulator: &mut Emulator) {
        
        self.set_default_panels();
        self.debugger_panel.pause(emulator);

        let framebuffer = Framebuffer::new();

        let mut seconds_timer = Instant::now();
            
        while self.running && !self.renderer.poll_events() {

            let keyboard_state = self.renderer.event_pump.keyboard_state();

            if keyboard_state.is_scancode_pressed(Scancode::LCtrl) &&
                keyboard_state.is_scancode_pressed(Scancode::W) {   
                break;
            }

            emulator.update_joypad_keys(
                self.keyboard_map_panel.update_keys(keyboard_state.pressed_scancodes().collect()));

            self.debugger_panel.cycle(emulator);

            // emulator.cycle();

            let now = Instant::now();

            if (now - seconds_timer).as_secs() >= 1 {

                if emulator.cpu.clock_cycles != 0 {
                    
                    self.renderer.window
                        .set_title(format!("Gameboy {}%", (emulator.cpu.clock_cycles as f32 / 4_194_304.0) * 100.0).as_str())
                        .unwrap();
                }

                emulator.cpu.clock_cycles = 0;
                seconds_timer = now;
            }

            self.panels
                .iter_mut()
                .chain(self.small_panels.iter_mut())
                .for_each(|panel| panel.update(&emulator, &emulator.memory_map.changes));
            self.debugger_panel
                .update(&emulator, &emulator.memory_map.changes);
            self.keyboard_map_panel
                .update(&emulator, &emulator.memory_map.changes);

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
                    (self.renderer.window_height as i32 - 19) / 2 ,
                ),
            );

            let width = self.renderer.window_width as f32;
            let height = (self.renderer.window_height - 19) as f32;

            self.renderer.render(|ui| {
                ui.main_menu_bar(|| {
                    ui.menu("File", || {
                        ui.menu_item("Load Cartidage");
                        if ui.menu_item("Reload Cartidage") {
                            *emulator = Emulator::after_boot();
                            emulator.load_cartidge("./roms/tetris.gb");

                            self.debugger_panel.toggled_breakpoint = None;
                        }

                        if ui.menu_item("Quit") {
                            self.running = false;
                            return;
                        }
                    });

                    ui.menu("Window", || {

                        self.small_panels
                            .iter_mut()
                            .for_each(|panel| {
                                if ui.menu_item_config(panel.get_name())
                                    .selected(panel.is_opened())
                                    .build() {
                                    panel.set_opened(!panel.is_opened());
                                }
                            });
                        
                        if ui.menu_item_config(self.keyboard_map_panel.get_name())
                            .selected(self.keyboard_map_panel.is_opened())
                            .build() {
                            self.keyboard_map_panel.set_opened(!self.keyboard_map_panel.is_opened());
                        }                            
                    });

                    ui.menu("Debug", || {
                        
                        if ui.menu_item("Continue                F5") {
                            self.debugger_panel.continue_execution();
                        }

                        if ui.menu_item("Run To Next Line        F10") {
                            self.debugger_panel.run_to_next_line();
                        }

                        if ui.menu_item("Pause                   F6") {
                            self.debugger_panel.pause(emulator);
                        }

                        if ui.menu_item("Clear All Breakpoints") {

                            self.debugger_panel.breakpoints.clear();
                        }

                    });

                    ui.show_metrics_window(&mut true);
                });

                self.panels
                    .iter_mut()
                    .chain(self.small_panels.iter_mut())
                    .for_each(|panel| panel.render(ui, emulator, width, height));

                self.debugger_panel.render(ui, emulator, width, height);
                self.keyboard_map_panel.render(ui, emulator, width, height);
            
                // ui.show_demo_window(&mut true);
            });
        }
    }
}

use self::panels::debugger::DebuggerPanel;
use self::panels::memory::MemoryPanel;
use self::panels::registers::RegistersPanel;

use super::emulator::Emulator;
use super::renderer::{framebuffer::Framebuffer, Renderer};

mod panels;

use panels::Panel;

pub struct EmulatorRenderer {
    running: bool,

    debugger_panel: DebuggerPanel,
    panels: Vec<Box<dyn Panel>>,

    renderer: Renderer,
}

impl EmulatorRenderer {
    pub fn new() -> Self {
        Self {
            running: true,

            debugger_panel: DebuggerPanel::new(),
            panels: vec![
                Box::new(MemoryPanel::new()) as Box<dyn Panel>,
                Box::new(RegistersPanel::new()) as Box<dyn Panel>,
            ],
            renderer: Renderer::new("Gameboy", 800, 739).unwrap(),
        }
    }

    pub fn render(&mut self, emulator: &mut Emulator) {
        // emulator.memory_map.on_change = Box::new(|memory_map, change| {
        //     self.panels.iter_mut().for_each(|panel| panel.update(memory_map, change));
        // });

        // emulator.memory_map.set_on_change(|memory_map, change| {

        //     self.panels.iter().for_each(|panel| panel.update(emulator, change));
        // });

        let framebuffer = Framebuffer::new();

        while self.running && !self.renderer.poll_events() {

            self.debugger_panel.cycle(emulator);

            self.panels
                .iter_mut()
                .for_each(|panel| panel.update(&emulator, &emulator.memory_map.changes));
            self.debugger_panel
                .update(&emulator, &emulator.memory_map.changes);

            emulator.memory_map.changes.clear();

            // Render frame.
            self.renderer.clear_screen();

            framebuffer.update_buffer(
                160 as _,
                144 as _,
                [0xFF0000FFu32; 160 * 144].as_ptr() as _,
                gl::RGBA8,
                gl::RGBA,
            );
            framebuffer.draw_buffer(
                (0, 0),
                (160 as _, 144 as _),
                (0, self.renderer.window_height as i32),
                (
                    self.renderer.window_width as i32 / 2,
                    self.renderer.window_height as i32 / 2 - 19,
                ),
            );

            let width = self.renderer.window_width as f32;
            let height = self.renderer.window_height as f32;

            self.renderer.render(|ui| {
                ui.main_menu_bar(|| {
                    ui.menu("File", || {
                        ui.menu_item("Load Cartidage");
                        if ui.menu_item("Reload Cartidage") {
                            *emulator = Emulator::new();
                            emulator.load_cartidge("./roms/tetris.gb");

                            self.debugger_panel.toggled_breakpoint = None;
                        }

                        if ui.menu_item("Quit") {
                            self.running = false;
                            return;
                        }
                    });

                    ui.menu("Window", || {
                        ui.menu_item("deneme");
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

                            self.debugger_panel.breakpoints = Vec::new();
                        }

                    });
                });

                self.panels
                    .iter_mut()
                    .for_each(|panel| panel.render(ui, emulator, width, height));
                self.debugger_panel.render(ui, &emulator, width, height);

                // ui.show_demo_window(&mut true);
            });
        }
    }
}
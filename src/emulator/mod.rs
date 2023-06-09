use std::{rc::Rc, cell::RefCell};

use crate::renderer::{self, framebuffer::Framebuffer, Renderer};

mod instructions;
mod cpu;
mod registers;
mod emulator_renderer;
mod memory_map;
mod memory_strings;

use cpu::Cpu;
use memory_map::MemoryMap;

use self::{emulator_renderer::EmulatorRenderer, memory_strings::{MemoryString, DisassemblyString}};

pub struct Emulator {
    cpu: Cpu,

    memory_map: Rc<RefCell<MemoryMap>>,

    renderer: Renderer,
}

impl Emulator {
    pub fn new() -> Emulator {
        
        Self {
            cpu: Cpu::new(),
            memory_map: Rc::new(RefCell::new(MemoryMap::new())),
            renderer: Renderer::new("Gameboy", 800, 720).unwrap()
        }
    }

    pub fn run(mut self) {

        let emulator_renderer = EmulatorRenderer::new(&mut self);

        self.memory_map.borrow_mut().load_rom("./roms/tetris.gb");

        let framebuffer = Framebuffer::new();

        while !self.renderer.poll_events() {
            // Execute actual emulator code.

            self.cpu.cycle(&mut self.memory_map.borrow_mut());

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
                (0, self.renderer.window_height as _),
                (
                    self.renderer.window_width as i32 / 2,
                    self.renderer.window_height as i32 / 2,
                ),
            );

            emulator_renderer.borrow_mut().render(&mut self);
        }
    }
}

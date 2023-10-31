#![feature(const_mut_refs)]

mod emulator;
mod emulator_renderer;
mod renderer;

fn main() {
    emulator_renderer::EmulatorRenderer::new().run();
}

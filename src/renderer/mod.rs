use std::{error::Error, time::Instant};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    video::{GLContext, GLProfile, SwapInterval},
    EventPump,
};

pub mod framebuffer;
mod utils;

use crate::gl_call;

use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
pub struct Renderer {
    last_frame: Instant,

    pub window_width: u32,
    pub window_height: u32,

    pub imgui: imgui::Context,
    imgui_sdl: SdlPlatform,
    imgui_renderer: AutoRenderer,

    pub event_pump: EventPump,

    pub window: sdl2::video::Window,
    pub video_subsys: sdl2::VideoSubsystem,
    pub sdl: sdl2::Sdl,

    _gl_context: GLContext,
}

impl Renderer {
    pub fn new(title: impl AsRef<str>, width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let sdl = sdl2::init()?;
        let video_subsys = sdl.video()?;

        let gl_attr = video_subsys.gl_attr();

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsys
            .window(title.as_ref(), width, height)
            .opengl()
            .resizable()
            .position_centered()
            .build()?;

        let event_pump = sdl.event_pump()?;

        let _gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsys.gl_get_proc_address(s) as _);

        video_subsys.gl_set_swap_interval(SwapInterval::VSync)?;

        let mut imgui = imgui::Context::create();

        imgui.set_ini_filename(None);

        let glow_context = unsafe {
            imgui_glow_renderer::glow::Context::from_loader_function(|s| {
                video_subsys.gl_get_proc_address(s) as _
            })
        };

        let imgui_renderer = AutoRenderer::initialize(glow_context, &mut imgui)?;

        let imgui_sdl = SdlPlatform::init(&mut imgui);

        Ok(Self {
            window_width: width,
            window_height: height,

            last_frame: Instant::now(),

            imgui,
            imgui_sdl,
            imgui_renderer,
            event_pump,
            window,
            video_subsys,
            sdl,
            _gl_context,
        })
    }

    pub fn poll_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            self.imgui_sdl.handle_event(&mut self.imgui, &event);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        self.window_width = width as u32;
                        self.window_height = height as u32;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        false
    }

    pub fn clear_screen(&mut self) {
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
        gl_call!(gl::ClearColor(0.0, 0.0, 0.0, 1.0));
    }

    pub fn get_frame(&mut self) -> &imgui::Ui {
        // Update imgui's delta time.
        self.imgui.io_mut().delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        // Create the imgui frame.
        self.imgui_sdl
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        self.imgui.frame()
    }

    pub fn render_frame(&mut self) {
        let draw_data = self.imgui.render();

        self.imgui_renderer.render(draw_data).unwrap();

        // Swap buffers.
        self.window.gl_swap_window();
    }

    pub fn render(&mut self, func: impl FnOnce(&imgui::Ui)) {
        // Update imgui's delta time.
        self.imgui.io_mut().delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        // Create the imgui frame.
        self.imgui_sdl
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.frame();

        // Give the user imgui's ui to render.
        func(&ui);

        let draw_data = self.imgui.render();

        self.imgui_renderer.render(draw_data).unwrap();

        // Swap buffers.
        self.window.gl_swap_window();
    }
}

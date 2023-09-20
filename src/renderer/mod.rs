use std::{error::Error, time::Instant};

use sdl2::{
    event::{Event, WindowEvent},
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

        // imgui.style_mut().window_padding = [15.0, 15.0];
        // imgui.style_mut().window_rounding = 0.0;
        // imgui.style_mut().frame_padding = [5.0, 5.0];
        // imgui.style_mut().frame_rounding = 4.0;
        // imgui.style_mut().item_spacing = [12.0, 8.0];
        // imgui.style_mut().item_inner_spacing = [8.0, 6.0];
        // imgui.style_mut().indent_spacing = 25.0;
        // imgui.style_mut().scrollbar_size = 20.0;
        // imgui.style_mut().scrollbar_rounding = 2.0;
        // imgui.style_mut().grab_min_size = 5.0;
        // imgui.style_mut().grab_rounding = 3.0;
     
        // imgui.style_mut().colors[StyleColor::Text as usize] = [0.80, 0.80, 0.83, 1.00];
        // imgui.style_mut().colors[StyleColor::TextDisabled as usize] = [0.24, 0.23, 0.29, 1.00];
        // imgui.style_mut().colors[StyleColor::WindowBg as usize] = [0.06, 0.05, 0.07, 1.00];
        // imgui.style_mut().colors[StyleColor::ChildBg as usize] = [0.07, 0.07, 0.09, 1.00];
        // imgui.style_mut().colors[StyleColor::PopupBg as usize] = [0.07, 0.07, 0.09, 1.00];
        // imgui.style_mut().colors[StyleColor::Border as usize] = [0.80, 0.80, 0.83, 0.88];
        // imgui.style_mut().colors[StyleColor::BorderShadow as usize] = [0.92, 0.91, 0.88, 0.00];
        // imgui.style_mut().colors[StyleColor::FrameBg as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::FrameBgHovered as usize] = [0.24, 0.23, 0.29, 1.00];
        // imgui.style_mut().colors[StyleColor::FrameBgActive as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::TitleBg as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::TitleBgCollapsed as usize] = [1.00, 0.98, 0.95, 0.75];
        // imgui.style_mut().colors[StyleColor::TitleBgActive as usize] = [0.07, 0.07, 0.09, 1.00];
        // imgui.style_mut().colors[StyleColor::MenuBarBg as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::ScrollbarBg as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::ScrollbarGrab as usize] = [0.80, 0.80, 0.83, 0.31];
        // imgui.style_mut().colors[StyleColor::ScrollbarGrabHovered as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::ScrollbarGrabActive as usize] = [0.06, 0.05, 0.07, 1.00];
        // // imgui.style_mut().colors[StyleColor::Combo as usize] = [0.19, 0.18, 0.21, 1.00];
        // imgui.style_mut().colors[StyleColor::CheckMark as usize] = [0.80, 0.80, 0.83, 0.31];
        // imgui.style_mut().colors[StyleColor::SliderGrab as usize] = [0.80, 0.80, 0.83, 0.31];
        // imgui.style_mut().colors[StyleColor::SliderGrabActive as usize] = [0.06, 0.05, 0.07, 1.00];
        // imgui.style_mut().colors[StyleColor::Button as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::ButtonHovered as usize] = [0.24, 0.23, 0.29, 1.00];
        // imgui.style_mut().colors[StyleColor::ButtonActive as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::Header as usize] = [0.10, 0.09, 0.12, 1.00];
        // imgui.style_mut().colors[StyleColor::HeaderHovered as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::HeaderActive as usize] = [0.06, 0.05, 0.07, 1.00];
        // // imgui.style_mut().colors[StyleColor::Column as usize] = [0.56, 0.56, 0.58, 1.00];
        // // imgui.style_mut().colors[StyleColor::ColumnHovered as usize] = [0.24, 0.23, 0.29, 1.00];
        // // imgui.style_mut().colors[StyleColor::ColumnActive as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::ResizeGrip as usize] = [0.00, 0.00, 0.00, 0.00];
        // imgui.style_mut().colors[StyleColor::ResizeGripHovered as usize] = [0.56, 0.56, 0.58, 1.00];
        // imgui.style_mut().colors[StyleColor::ResizeGripActive as usize] = [0.06, 0.05, 0.07, 1.00];
        // // imgui.style_mut().colors[StyleColor::CloseButton as usize] = [0.40, 0.39, 0.38, 0.16];
        // // imgui.style_mut().colors[StyleColor::CloseButtonHovered as usize] = [0.40, 0.39, 0.38, 0.39];
        // // imgui.style_mut().colors[StyleColor::CloseButtonActive as usize] = [0.40, 0.39, 0.38, 1.00];
        // imgui.style_mut().colors[StyleColor::PlotLines as usize] = [0.40, 0.39, 0.38, 0.63];
        // imgui.style_mut().colors[StyleColor::PlotLinesHovered as usize] = [0.25, 1.00, 0.00, 1.00];
        // imgui.style_mut().colors[StyleColor::PlotHistogram as usize] = [0.40, 0.39, 0.38, 0.63];
        // imgui.style_mut().colors[StyleColor::PlotHistogramHovered as usize] = [0.25, 1.00, 0.00, 1.00];
        // imgui.style_mut().colors[StyleColor::TextSelectedBg as usize] = [0.25, 1.00, 0.00, 0.43];
        // // imgui.style_mut().colors[StyleColor::ModalWindowDarkening as usize] = [1.00, 0.98, 0.95, 0.73];

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
                Event::Quit { .. } => return true,
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

    #[allow(dead_code)]
    pub fn get_frame(&mut self) -> &imgui::Ui {
        // Update imgui's delta time.
        self.imgui.io_mut().delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        // Create the imgui frame.
        self.imgui_sdl
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        self.imgui.frame()
    }

    #[allow(dead_code)]
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

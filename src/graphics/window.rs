use beryllium::*;
use video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval, GlWindow};

const WINDOW_TITLE: &str = "Hello, Beryllium!";

pub fn create_window(sdl: &Sdl) -> GlWindow{
    sdl.set_gl_context_major_version(4).unwrap();
    sdl.set_gl_context_minor_version(6).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    let mut flags = GlContextFlags::default();
    if cfg!(target_os = "macos") {
        flags |= GlContextFlags::FORWARD_COMPATIBLE;
    }
    if cfg!(debug_assertions) {
        flags |= GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();
    let window = sdl
        .create_gl_window(CreateWinArgs {
            title: WINDOW_TITLE,
            width: 800,
            height: 600,
            ..Default::default()
        })
        .expect("couldn't make a window and context");
    //OpenGL
    window.set_swap_interval(GlSwapInterval::Vsync).unwrap();
    window
}

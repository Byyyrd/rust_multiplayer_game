use beryllium::init::InitFlags;
use beryllium::*;
use events::Event;
type Vertex = [f32; 3];
use crate::graphics::shader;
use crate::graphics::window;

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
pub fn start() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    let window = window::create_window(&sdl);
    unsafe {
        gl::load_with(|name| {
            // let mut result: Vec<u8> = name.to_string().into_bytes();
            // result.append(&mut vec![0 as u8]);
            // let name2 = String::from_utf8(result).unwrap();

            let mut str: String = String::new();
            str += name;
            str += "\0";
            let addr = window.get_proc_address(str.as_ptr());
            if addr.is_null() {
                eprintln!("Could not load OpenGL function: {:?}", name);
            }
            addr
        });

        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        //Vao
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

        //Vbo
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        let shader_program = shader::create_program();
        gl::UseProgram(shader_program);
    }
    'main_loop: loop {
        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_window();
    }
}

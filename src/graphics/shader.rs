
const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
  
    void main() {
      gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
  "#;

const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;
  
    void main() {
      final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
  "#;
  
pub unsafe fn create_program() -> u32 {
    let vertex_shader = create_shader(VERT_SHADER, gl::VERTEX_SHADER);
    let fragment_shader = create_shader(FRAG_SHADER, gl::FRAGMENT_SHADER);

    let shader_program = gl::CreateProgram();
    assert_ne!(shader_program, 0);

    link_shader(shader_program, vertex_shader, fragment_shader);
    shader_program
}

unsafe fn create_shader(source: &str, usage: u32) -> u32 {
    let shader = gl::CreateShader(usage);
    assert_ne!(shader, 0);
    gl::ShaderSource(
        shader,
        1,
        &(source.as_bytes().as_ptr().cast()),
        &(source.len().try_into().unwrap()),
    );
    gl::CompileShader(shader);
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
    }
    shader
}
unsafe fn link_shader(shader_program: u32, vertex_shader: u32, fragment_shader: u32) -> u32 {
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    let mut success = 0;
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
    }
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
    shader_program
}

// let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
// assert_ne!(fragment_shader, 0);
// gl::ShaderSource(
//   fragment_shader,
//   1,
//   &(FRAG_SHADER.as_bytes().as_ptr().cast()),
//   &(FRAG_SHADER.len().try_into().unwrap()),
// );
// gl::CompileShader(fragment_shader);
// let mut success = 0;
// gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
// if success == 0 {
//   let mut v: Vec<u8> = Vec::with_capacity(1024);
//   let mut log_len = 0_i32;
//   gl::GetShaderInfoLog(
//     fragment_shader,
//     1024,
//     &mut log_len,
//     v.as_mut_ptr().cast(),
//   );
//   v.set_len(log_len.try_into().unwrap());
//   panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
// }

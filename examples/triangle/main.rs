extern crate rgl;
extern crate gl;
extern crate glutin;

use glutin::ContextTrait;
use gl::types::*;


static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

fn main() {
    //Set up the window
    let mut events_loop = glutin::EventsLoop::new();
    let win_builder = glutin::WindowBuilder::new();
    let ctx_builder = glutin::ContextBuilder::new();

    let windowed_context = ctx_builder.build_windowed(win_builder, &events_loop).unwrap();

    unsafe {
        // It is essential to make the context current before calling `gl::load_with`.
        windowed_context.make_current().unwrap()
    };

    //Load gl library
    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    //Create a vertex array object and a vertex buffer object
    let mut vao = rgl::gen_vertex_array();;
    let mut vbo = rgl::gen_buffer();

    //Generate and bind the VAO
    rgl::bind_vertex_array(vao);

    //Generate and bind the VBO
    rgl::bind_buffer(rgl::Target::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::Target::ArrayBuffer, &VERTEX_DATA, rgl::Usage::StaticDraw);
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_pointer(0, 2, rgl::Type::Float, false, 0);

    //Main loop
    let mut is_running = true;
    while is_running {
        //Poll window events
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => is_running = false,
                    _ => (),
                },
                _ => ()
            }
        });

        //Draw stuff
        rgl::clear_color(0.5, 0.5, 0.8, 1.0);
        unsafe {  gl::Clear(gl::COLOR_BUFFER_BIT); }
        rgl::draw_arrays(rgl::Primitive::Triangles, 0, 3);

        windowed_context.swap_buffers().unwrap();
    }

    //Cleanup
    rgl::delete_buffer(&mut vbo);
    rgl::delete_vertex_array(&mut vao);
}

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra;

use gfx::traits::FactoryExt;
use gfx::Device;

pub mod camera;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        position: [f32; 3] = "a_position",
        color:    [f32; 4] = "a_color",
    }

    constant Transform {
        combined: [[f32; 4]; 4] = "u_combined",
    }

    pipeline pipe {
        vbuf:      gfx::VertexBuffer<Vertex>      = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out:       gfx::RenderTarget<ColorFormat> = "f_color",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { position: [ -0.5, -0.5, 0.0 ], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { position: [  0.5, -0.5, 0.0 ], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { position: [  0.0,  0.5, 0.0 ], color: [0.0, 0.0, 1.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let events_loop = glutin::EventsLoop::new();

    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let res = factory.create_pipeline_simple(
        include_bytes!("../shaders/triangle.330.vert"),
        include_bytes!("../shaders/triangle.330.frag"),
        pipe::new()
    );

    let pso = match res {
        Err(err) => {
            println!("Error: {:?}", err);
            panic!("Failed to compile triangle shader");
        },
        Ok(pipeline) => {
            pipeline
        }
    };

    let mut camera = camera::OrthoCamera::new(1024.0, 768.0);

    let mut transform = Transform {
        combined: camera.combined(),
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let transform_buffer = factory.create_constant_buffer(1);

    let mut data = pipe::Data {
        vbuf:      vertex_buffer,
        transform: transform_buffer,
        out:       main_color
    };

    let mut running = true;
    while running {
        camera.translate(0.001, 0.001);
        camera.rotate(0.01);

        // fetch events
        events_loop.poll_events(|glutin::Event::WindowEvent{window_id: _, event}| {
            match event {
                glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) |
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                },
                _ => {},
            }
        });

        transform.combined = camera.combined();

        // Update constant buffesr
        match encoder.update_buffer(&data.transform, &[transform], 0) {
            Err(err) => {
                println!("Error: {:?}", err);
            },
            _ => {}
        }

        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

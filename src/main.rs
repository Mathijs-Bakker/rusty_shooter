use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    engine::{resource_manager, Engine},
    event::{Event, VirtualKeyCode, WindowEvent},
    // window::WindowBuilder,
    event_loop::{ControlFlow, EventLoop},
    gui::window::{WindowBuilder, WindowTitle},
    resource::texture::TextureWrapMode,
    scene::{
        base::BaseBuilder,
        camera::{CameraBuilder, SkyBox, SkyBoxBuilder},
        collider::{ColliderBuilder, ColliderShape},
        node::Node,
        rigidbody::RigidBodyBuilder,
        transform::TransformBuilder,
        Scene,
    },
};
use std::time;

// Update game logic rate in Hz.
const TIMESTEP: f32 = 1.0 / 60.0;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self) {
        // Todo: Gamelogic
    }
}

fn main() {
    // 1.1. Configure app window:
    let window_builder =
        fyrox::window::WindowBuilder::new().with_title("Rusty Shooter".to_string());
    // 1.2. OS event listener: Receives events from the operating system (close, resize window etc.)
    let event_loop = EventLoop::new();
    // 1.3. Create an instance of the Fyrox game engine. With WindowBuilder and EventLoop as args.
    let mut engine = Engine::new(window_builder, &event_loop, false).unwrap();

    // Initialize game instance.
    // Todo: It is empty for now.
    let mut game = Game::new();

    // Run the event loop of the main window. which will respond to OS and window events and update
    // engine's state accordingly. Engine lets you to decide which event should be handled,

    // Time vars for game loop:
    let clock = time::Instant::now();
    let mut elapsed_time = 0.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // The core of the game loop:
                // Fixed time step, makes sure that code runs at a fixed speed.
                // even if the renderer can't keep up the fps.
                let mut delta_time = clock.elapsed().as_secs_f32() - elapsed_time;
                while delta_time >= TIMESTEP {
                    delta_time -= TIMESTEP;
                    elapsed_time += TIMESTEP;

                    // Run our game's logic.
                    game.update();

                    // Update engine each frame.
                    engine.update(TIMESTEP);
                }

                // Rendering must be explicitly requested and handled after RedrawRequested event is received.
                engine.get_window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Render at max speed (it is not tied to the game code).
                engine.render().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                // Handle window events:
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    // Exit game by hitting Escape.
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit
                    }
                }
                WindowEvent::Resized(size) => {
                    // The renderer knows nothing about window size - it must be notified
                    // directly when window size has changed.
                    engine.set_frame_size(size.into()).unwrap();
                }
                _ => (),
            },
            // Continue listening:
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}

use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    engine::{resource_manager, Engine},
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
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
    window::WindowBuilder,
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
    // 1.a - Configure app window:
    let window_builder = WindowBuilder::new(fyrox::gui::widget::WidgetBuilder::new())
        .with_title(WindowTitle::Text("Rusty Shooter".to_string()));
    // 1.b - OS event listener:
    let event_loop = EventLoop::new();
}

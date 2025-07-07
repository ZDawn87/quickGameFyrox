// Cargo.toml dependencies needed:
// [dependencies]
// fyrox = "0.34"
// nalgebra = "0.32"

use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        color::Color,
        math::{vector::Vec3Ext, Matrix4, SmoothAngle},
        pool::Handle,
    },
    engine::{Engine, EngineInitParams, SerializationContext},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    material::{Material, PropertyValue},
    scene::{
        base::BaseBuilder,
        camera::{CameraBuilder, SkyBox, SkyBoxBuilder},
        light::{directional::DirectionalLightBuilder, BaseLightBuilder},
        mesh::{
            surface::{SurfaceBuilder, SurfaceData},
            MeshBuilder, RenderPath,
        },
        node::Node,
        transform::TransformBuilder,
        Scene, SceneBuilder,
    },
    resource::{
        model::{ModelResourceExtension, ModelResourceLoader},
        texture::{
            CompressionOptions, TextureImportOptions, TextureMinificationFilter,
            TextureMagnificationFilter, TextureResource, TextureResourceExtension,
        },
    },
    utils::translate_event,
    window::WindowBuilder,
};
use std::{collections::HashSet, time::Instant};

// Game state structure
pub struct Game {
    scene: Handle<Scene>,
    player: Handle<Node>,
    camera: Handle<Node>,
    input_state: InputState,
    last_time: Instant,
}

#[derive(Default)]
struct InputState {
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    mouse_delta: Vector3<f32>,
    camera_yaw: f32,
    camera_pitch: f32,
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {
        let mut scene = create_scene(engine);
        
        // Create player (a simple colored cube)
        let player = create_player(&mut scene, engine);
        
        // Create camera
        let camera = create_camera(&mut scene);
        
        // Add the scene to the engine
        let scene_handle = engine.scenes.add(scene);
        
        Self {
            scene: scene_handle,
            player,
            camera,
            input_state: InputState::default(),
            last_time: Instant::now(),
        }
    }
    
    pub fn update(&mut self, engine: &mut Engine) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;
        
        // Get the scene
        let scene = &mut engine.scenes[self.scene];
        
        // Update player movement
        self.update_player_movement(scene, dt);
        
        // Update camera
        self.update_camera(scene, dt);
    }
    
    fn update_player_movement(&mut self, scene: &mut Scene, dt: f32) {
        let speed = 5.0; // units per second
        let mut movement = Vector3::new(0.0, 0.0, 0.0);
        
        // Calculate movement direction based on input
        if self.input_state.move_forward {
            movement.z -= 1.0;
        }
        if self.input_state.move_backward {
            movement.z += 1.0;
        }
        if self.input_state.move_left {
            movement.x -= 1.0;
        }
        if self.input_state.move_right {
            movement.x += 1.0;
        }
        
        // Normalize movement vector if not zero
        if movement.magnitude() > 0.0 {
            movement = movement.normalize() * speed * dt;
            
            // Apply movement to player
            if let Some(player_node) = scene.graph.try_get_mut(self.player) {
                let transform = player_node.local_transform_mut();
                let current_position = **transform.position();
                transform.set_position(current_position + movement);
            }
        }
    }
    
    fn update_camera(&mut self, scene: &mut Scene, dt: f32) {
        // Camera follows player with some offset
        if let Some(player_node) = scene.graph.try_get(self.player) {
            let player_position = **player_node.local_transform().position();
            
            // Camera position offset (behind and above the player)
            let camera_offset = Vector3::new(0.0, 3.0, 5.0);
            let target_position = player_position + camera_offset;
            
            if let Some(camera_node) = scene.graph.try_get_mut(self.camera) {
                let transform = camera_node.local_transform_mut();
                
                // Smoothly move camera to target position
                let current_position = **transform.position();
                let new_position = current_position.lerp(&target_position, dt * 2.0);
                transform.set_position(new_position);
                
                // Look at player
                let look_direction = (player_position - new_position).normalize();
                let rotation = UnitQuaternion::look_at_rh(&look_direction, &Vector3::y());
                transform.set_rotation(rotation);
            }
        }
    }
    
    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = device_event {
            self.input_state.mouse_delta.x = delta.0 as f32;
            self.input_state.mouse_delta.y = delta.1 as f32;
        }
    }
    
    pub fn handle_key_input(&mut self, input: &KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let is_pressed = input.state == ElementState::Pressed;
            
            match key_code {
                VirtualKeyCode::W => self.input_state.move_forward = is_pressed,
                VirtualKeyCode::S => self.input_state.move_backward = is_pressed,
                VirtualKeyCode::A => self.input_state.move_left = is_pressed,
                VirtualKeyCode::D => self.input_state.move_right = is_pressed,
                _ => {}
            }
        }
    }
}

fn create_scene(engine: &mut Engine) -> Scene {
    let mut scene = SceneBuilder::new()
        .build(&mut engine.resource_manager.state());
    
    // Add lighting
    DirectionalLightBuilder::new(BaseLightBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 6.0, 0.0))
                .with_local_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    -45.0f32.to_radians(),
                ))
                .build(),
        ),
    ))
    .with_color(Color::opaque(255, 255, 255))
    .build(&mut scene.graph);
    
    // Create ground plane
    create_ground_plane(&mut scene, engine);
    
    // Create some obstacles/decorations
    create_obstacles(&mut scene, engine);
    
    scene
}

fn create_player(scene: &mut Scene, engine: &mut Engine) -> Handle<Node> {
    // Create a simple colored cube for the player
    let mut material = Material::standard();
    
    // Set player color to bright blue
    material
        .set_property(
            &fyrox::material::DIFFUSE_COLOR,
            PropertyValue::Color(Color::opaque(0, 100, 255)),
        )
        .unwrap();
    
    let material_handle = engine.resource_manager.state().containers_mut().materials.add(material);
    
    // Create cube geometry
    let surface_data = SurfaceData::make_cube(Matrix4::identity());
    let surface = SurfaceBuilder::new(surface_data)
        .with_material(material_handle)
        .build();
    
    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 1.0, 0.0))
                .with_local_scale(Vector3::new(0.5, 1.0, 0.5))
                .build(),
        ),
    )
    .with_surfaces(vec![surface])
    .with_render_path(RenderPath::Forward)
    .build(&mut scene.graph)
}

fn create_camera(scene: &mut Scene) -> Handle<Node> {
    // Create a skybox for better visual appeal
    let skybox = SkyBoxBuilder::new(BaseBuilder::new())
        .build(&mut scene.graph);
    
    CameraBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 3.0, 5.0))
                .build(),
        ),
    )
    .with_skybox(skybox)
    .build(&mut scene.graph)
}

fn create_ground_plane(scene: &mut Scene, engine: &mut Engine) {
    // Create ground material
    let mut ground_material = Material::standard();
    ground_material
        .set_property(
            &fyrox::material::DIFFUSE_COLOR,
            PropertyValue::Color(Color::opaque(100, 150, 100)),
        )
        .unwrap();
    
    let ground_material_handle = engine.resource_manager.state().containers_mut().materials.add(ground_material);
    
    // Create large plane for ground
    let surface_data = SurfaceData::make_plane(Matrix4::identity());
    let surface = SurfaceBuilder::new(surface_data)
        .with_material(ground_material_handle)
        .build();
    
    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, 0.0))
                .with_local_scale(Vector3::new(20.0, 1.0, 20.0))
                .build(),
        ),
    )
    .with_surfaces(vec![surface])
    .with_render_path(RenderPath::Forward)
    .build(&mut scene.graph);
}

fn create_obstacles(scene: &mut Scene, engine: &mut Engine) {
    // Create material for obstacles
    let mut obstacle_material = Material::standard();
    obstacle_material
        .set_property(
            &fyrox::material::DIFFUSE_COLOR,
            PropertyValue::Color(Color::opaque(200, 100, 50)),
        )
        .unwrap();
    
    let obstacle_material_handle = engine.resource_manager.state().containers_mut().materials.add(obstacle_material);
    
    // Create several cube obstacles around the scene
    let positions = vec![
        Vector3::new(3.0, 0.5, 2.0),
        Vector3::new(-2.0, 0.5, -3.0),
        Vector3::new(5.0, 0.5, -1.0),
        Vector3::new(-4.0, 0.5, 4.0),
        Vector3::new(1.0, 0.5, -5.0),
    ];
    
    for position in positions {
        let surface_data = SurfaceData::make_cube(Matrix4::identity());
        let surface = SurfaceBuilder::new(surface_data)
            .with_material(obstacle_material_handle.clone())
            .build();
        
        MeshBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_position(position)
                    .with_local_scale(Vector3::new(1.0, 1.0, 1.0))
                    .build(),
            ),
        )
        .with_surfaces(vec![surface])
        .with_render_path(RenderPath::Forward)
        .build(&mut scene.graph);
    }
}

fn main() {
    // Create event loop and window
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Simple Fyrox Game - Player Movement")
        .with_resizable(true)
        .with_inner_size(fyrox::winit::dpi::LogicalSize::new(1024, 768));
    
    // Initialize engine
    let serialization_context = SerializationContext::new();
    let mut engine = Engine::new(EngineInitParams {
        window_builder,
        resource_manager: Default::default(),
        serialization_context,
    })
    .unwrap();
    
    // Set up the graphics context
    engine.renderer.set_backbuffer_clear_color(Color::opaque(100, 150, 200));
    
    // Create our game
    let mut game = Game::new(&mut engine);
    
    // Game loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                // Update game logic
                game.update(&mut engine);
                
                // Render the frame
                engine.update(1.0 / 60.0, control_flow, &mut game, Default::default());
            }
            Event::MainEventsCleared => {
                engine.get_window().request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        game.handle_key_input(&input);
                    }
                    WindowEvent::Resized(size) => {
                        if let Some(scene) = engine.scenes.try_get_mut(game.scene) {
                            scene.rendering_options.frame_size = (size.width, size.height);
                        }
                    }
                    _ => {}
                }
                
                if let Some(os_event) = translate_event(&event) {
                    engine.user_interface.process_os_event(&os_event);
                }
            }
            Event::DeviceEvent { event, .. } => {
                game.handle_device_event(&event);
            }
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}

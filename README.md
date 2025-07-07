# quickGameFyrox

# Simple Fyrox Game - Player Movement

A basic 3D game built with the Fyrox engine demonstrating fundamental game development concepts including player movement, camera controls, and 3D scene management.

![Game Screenshot](https://via.placeholder.com/600x400/4CAF50/FFFFFF?text=Fyrox+Game+Demo)

## Features

- **3D Player Movement**: Control a blue cube character using WASD keys
- **Dynamic Camera**: Third-person camera that smoothly follows the player
- **3D Environment**: Ground plane with orange cube obstacles scattered around
- **Lighting**: Directional lighting for realistic 3D rendering
- **Smooth Controls**: Responsive movement with proper delta-time handling

## Controls

- **W** - Move forward
- **S** - Move backward  
- **A** - Move left
- **D** - Move right
- **ESC** - Close game (or close window)

## Getting Started

### Prerequisites

- Rust 1.70+ installed ([rustup.rs](https://rustup.rs/))
- A graphics card that supports OpenGL 3.3+ or Vulkan

### Installation & Running

1. **Clone or create the project**:
   ```bash
   cargo new simple-fyrox-game --bin
   cd simple-fyrox-game
   ```

2. **Copy the game files**:
   - Replace `src/main.rs` with the provided game code
   - Replace `Cargo.toml` with the provided dependencies

3. **Run the game**:
   ```bash
   cargo run
   ```

   For better performance:
   ```bash
   cargo run --release
   ```

### First Run

The game will:
1. Open a 1024x768 window with a blue sky background
2. Create a green ground plane
3. Spawn orange cube obstacles around the scene
4. Place your blue player cube at the center
5. Position the camera behind and above the player

## Code Structure

### Game Architecture

- **`Game` struct**: Main game state containing scene handles and input state
- **Scene Creation**: Procedural 3D scene with lighting, ground, and obstacles
- **Player System**: Simple movement with WASD input handling
- **Camera System**: Third-person follow camera with smooth interpolation

### Key Components

```rust
// Main game state
pub struct Game {
    scene: Handle<Scene>,     // Reference to the 3D scene
    player: Handle<Node>,     // Reference to player object
    camera: Handle<Node>,     // Reference to camera
    input_state: InputState,  // Current input states
    last_time: Instant,       // For delta-time calculations
}
```

### Learning Objectives

This simple game demonstrates:
- **Fyrox Engine Setup**: Basic engine initialization and game loop
- **3D Scene Management**: Creating and managing nodes in a scene graph
- **Input Handling**: Keyboard input processing and state management
- **Transform Operations**: Moving objects in 3D space
- **Material System**: Creating and applying colored materials
- **Camera Controls**: Implementing follow cameras
- **Delta Time**: Smooth, frame-rate independent movement

## Extending the Game

### Easy Modifications

1. **Change Player Speed**: Modify the `speed` variable in `update_player_movement()`
2. **Add More Obstacles**: Extend the `positions` vector in `create_obstacles()`
3. **Change Colors**: Modify the color values in material creation
4. **Adjust Camera**: Change the `camera_offset` in `update_camera()`

### Advanced Features to Add

- **Collision Detection**: Prevent player from moving through obstacles
- **Jumping**: Add Y-axis movement with gravity
- **Score System**: Collect items for points
- **Enemy AI**: Moving obstacles or chasers
- **Textures**: Replace solid colors with image textures
- **Sound Effects**: Add audio for movement and interactions
- **Particle Effects**: Add visual flair with particle systems

## Troubleshooting

### Common Issues

**Game won't compile**:
- Ensure Rust 1.70+ is installed: `rustc --version`
- Check that all dependencies are properly specified in `Cargo.toml`

**Poor performance**:
- Run with `cargo run --release` for optimized build
- Update graphics drivers
- Lower window resolution by modifying `with_inner_size()`

**Black screen**:
- Ensure your graphics card supports OpenGL 3.3+
- Try updating graphics drivers

**Controls not working**:
- Make sure the game window has focus
- Check that keyboard input events are being received

## Dependencies

- **fyrox**: 3D game engine with rendering, scene management, and input handling
- **nalgebra**: Mathematical operations for 3D transformations

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Next Steps

1. **Follow Fyrox Tutorials**: Check the [Fyrox Book](https://fyrox-book.github.io/) for advanced features
2. **Experiment**: Try modifying colors, positions, and behaviors
3. **Add Features**: Implement collision detection or jumping mechanics
4. **Use FyroxEd**: Try the visual editor for more complex scenes
5. **Join Community**: Participate in Fyrox Discord or forums for help and inspiration

Happy game development! 🎮

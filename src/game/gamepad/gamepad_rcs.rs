use bevy::prelude::*;

#[derive(Resource)]
pub struct MyGamepad {
    pub gamepad: Gamepad,
    pub sensitivity: (f32, f32),
    pub deadzone: f32,
}

impl Default for MyGamepad {
    fn default() -> Self {
        MyGamepad {
            gamepad: Gamepad::new(0),
            sensitivity: (5.0, 3.0),
            deadzone: 0.5,
        }
    }
}

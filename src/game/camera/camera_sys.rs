use bevy::{
    input::gamepad::GamepadButtonType::*,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use super::camera_cmps::OrbitCamera;
use crate::game::gamepad::gamepad_rcs::MyGamepad;

// heavily referenced https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&OrbitCamera, &mut Transform), With<OrbitCamera>>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.iter() {
        rotation = ev.delta;
    }

    for (cam, mut transform) in cam_q.iter_mut() {
        if rotation.length_squared() > 0.0 {
            let window = window_q.get_single().unwrap();
            let delta_x = {
                let delta = rotation.x / window.width() * std::f32::consts::PI * 2.0;
                if cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation.y / window.height() * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch;
        }

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
    }
}

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&OrbitCamera, &mut Transform), With<OrbitCamera>>,
    axis: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp
    } else {
        return;
    };

    // get X & Y axis of right joystick
    let x_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickY);

    let mut rotation = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        if x.abs() > gamepad.deadzone || y.abs() > gamepad.deadzone {
            rotation = Vec2::new(x, y);
        }
    }

    for (cam, mut transform) in cam_q.iter_mut() {
        if rotation.length_squared() > 0.0 {
            let window = window_q.get_single().unwrap();
            let delta_x = {
                let delta = rotation.x / window.width()
                    * std::f32::consts::PI
                    * 2.0
                    * gamepad.sensitivity.0;
                if cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y =
                -rotation.y / window.height() * std::f32::consts::PI * gamepad.sensitivity.1;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch;
        }

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
    }
}

pub fn zoom_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cam_q: Query<&mut OrbitCamera, With<OrbitCamera>>,
) {
    let mut scroll = 0.0;
    for ev in scroll_evr.iter() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() > 0.0 {
            cam.radius -= scroll * cam.radius * 0.1;
        }
    }
}

pub fn zoom_gamepad(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut cam_q: Query<&mut OrbitCamera, With<OrbitCamera>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    if let Ok(mut cam) = cam_q.get_single_mut() {
        let right_thumb = GamepadButton::new(gamepad, RightThumb);
        if btns.pressed(right_thumb) {
            let d_pad_down = GamepadButton::new(gamepad, DPadDown);
            let d_pad_up = GamepadButton::new(gamepad, DPadUp);

            // zoom out
            if btns.pressed(d_pad_down) {
                cam.radius += cam.radius * 0.01;
            // zoom in
            } else if btns.pressed(d_pad_up) {
                cam.radius -= cam.radius * 0.01;
            }
        }
    }
}

// centers the cursor in the center of the screen
pub fn _lock_cursor(mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_q.get_single_mut().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;
    window.set_cursor_position(Some(Vec2::new(x, y)));
}

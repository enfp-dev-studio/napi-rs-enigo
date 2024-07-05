#![deny(clippy::all)]

use napi_derive::napi;
use enigo::{
    Axis::{Horizontal, Vertical},
    Button,
    Coordinate::{Abs, Rel},
    Direction::Click,
    Enigo, Settings, Mouse
};

// 공통 Enigo 인스턴스 생성 함수
fn create_enigo() -> Option<Enigo> {
    match Enigo::new(&Settings::default()) {
        Ok(enigo) => Some(enigo),
        Err(e) => {
            eprintln!("Failed to create Enigo: {:?}", e);
            None
        }
    }
}

#[napi]
fn move_mouse_rel(x: i32, y: i32) {
    if let Some(mut enigo) = create_enigo() {
        if let Err(e) = enigo.move_mouse(x, y, Rel) {
            eprintln!("Failed to move mouse: {:?}", e);
        }
    }
}

#[napi]
fn move_mouse_abs(x: i32, y: i32) {
    if let Some(mut enigo) = create_enigo() {
        if let Err(e) = enigo.move_mouse(x, y, Abs) {
            eprintln!("Failed to move mouse: {:?}", e);
        }
    }
}

#[napi]
fn mouse_click(button: String) {
    if let Some(mut enigo) = create_enigo() {
        let button = match button.as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => {
                eprintln!("Invalid button specified");
                return;
            }
        };
        if let Err(e) = enigo.button(button, Click) {
            eprintln!("Failed to click mouse: {:?}", e);
        }
    }
}

#[napi]
fn mouse_down(button: String) {
    if let Some(mut enigo) = create_enigo() {
        let button = match button.as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => {
                eprintln!("Invalid button specified");
                return;
            }
        };
        if let Err(e) = enigo.button(button, enigo::Direction::Press) {
            eprintln!("Failed to press mouse button: {:?}", e);
        }
    }
}

#[napi]
fn mouse_up(button: String) {
    if let Some(mut enigo) = create_enigo() {
        let button = match button.as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => {
                eprintln!("Invalid button specified");
                return;
            }
        };
        if let Err(e) = enigo.button(button, enigo::Direction::Release) {
            eprintln!("Failed to release mouse button: {:?}", e);
        }
    }
}

#[napi]
fn mouse_scroll(length: i32, is_vertical: bool) {
    if let Some(mut enigo) = create_enigo() {
        let axis = if is_vertical { Vertical } else { Horizontal };
        if let Err(e) = enigo.scroll(length, axis) {
            eprintln!("Failed to scroll: {:?}", e);
        }
    }
}
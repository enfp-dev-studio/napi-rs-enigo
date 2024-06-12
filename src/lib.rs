#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use enigo::{
  Axis::{Horizontal, Vertical},
  Button,
  Coordinate::{Abs, Rel},
  Direction::{Click, Press, Release},
  Enigo, Mouse, Settings,
};
// use std::thread;
// use std::time::Duration;

#[napi]
fn move_mouse_rel(x: i32, y: i32) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  let _ = enigo.move_mouse(x, y, Rel);
}

#[napi]
fn move_mouse_abs(x: i32, y: i32) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  let _ = enigo.move_mouse(x, y, Abs);
}

#[napi]
fn mouse_click(button: String) {
  // let wait_time = Duration::from_secs(2);
  // // let mut enigo = Enigo::new(&Settings::default()).unwrap();

  // thread::sleep(Duration::from_secs(4));
  // println!("screen dimensions: {:?}", enigo.main_display().unwrap());
  // // println!("mouse location: {:?}", enigo.location().unwrap());

  // thread::sleep(wait_time);

  // enigo.move_mouse(500, 200, Abs).unwrap();
  // thread::sleep(wait_time);

  // enigo.button(Button::Left, Press).unwrap();
  // thread::sleep(wait_time);

  // enigo.move_mouse(100, 100, Rel).unwrap();
  // thread::sleep(wait_time);

  // enigo.button(Button::Left, Release).unwrap();
  // thread::sleep(wait_time);

  // enigo.button(Button::Left, Click).unwrap();
  // thread::sleep(wait_time);

  // enigo.scroll(2, Horizontal).unwrap();
  // thread::sleep(wait_time);

  // enigo.scroll(-2, Horizontal).unwrap();
  // thread::sleep(wait_time);

  // enigo.scroll(2, Vertical).unwrap();
  // thread::sleep(wait_time);

  // enigo.scroll(-2, Vertical).unwrap();
  // thread::sleep(wait_time);

  // println!("mouse location: {:?}", enigo.location().unwrap());
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  if button == "left" {
    let _ = enigo.button(Button::Left, Click);
  } else if button == "right" {
    let _ = enigo.button(Button::Right, Click);
  } else if button == "middle" {
    let _ = enigo.button(Button::Middle, Click);
  } else {
  }
}

#[napi]
fn mouse_down(button: String) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  if button == "left" {
    let _ = enigo.button(Button::Left, Press);
  } else if button == "right" {
    let _ = enigo.button(Button::Right, Press);
  } else if button == "middle" {
    let _ = enigo.button(Button::Middle, Press);
  }
}

#[napi]
fn mouse_up(button: String) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  if button == "left" {
    let _ = enigo.button(Button::Left, Release);
  } else if button == "right" {
    let _ = enigo.button(Button::Right, Release);
  } else if button == "middle" {
    let _ = enigo.button(Button::Middle, Release);
  }
}

#[napi]
fn mouse_scroll(length: i32, is_vertical: bool) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  if is_vertical {
    let _ = enigo.scroll(length, Vertical);
  } else {
    let _ = enigo.scroll(length, Horizontal);
  }
}

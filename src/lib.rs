#![deny(clippy::all)]

use enigo::{Enigo as EnigoImpl, Keyboard, Mouse, Settings};
use napi::{Error, Result, Status};
use napi_derive::napi;

mod keys;
#[cfg(target_os = "macos")]
mod macos_input;
pub use keys::{Key, MouseButton};

fn to_napi_err(e: impl std::fmt::Display) -> Error {
  Error::new(Status::GenericFailure, e.to_string())
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Press,
  Release,
  Click,
}

impl From<Direction> for enigo::Direction {
  fn from(direction: Direction) -> Self {
    match direction {
      Direction::Press => Self::Press,
      Direction::Release => Self::Release,
      Direction::Click => Self::Click,
    }
  }
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
  Horizontal,
  Vertical,
}

impl From<Axis> for enigo::Axis {
  fn from(axis: Axis) -> Self {
    match axis {
      Axis::Horizontal => Self::Horizontal,
      Axis::Vertical => Self::Vertical,
    }
  }
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coordinate {
  Abs,
  Rel,
}

impl From<Coordinate> for enigo::Coordinate {
  fn from(coordinate: Coordinate) -> Self {
    match coordinate {
      Coordinate::Abs => Self::Abs,
      Coordinate::Rel => Self::Rel,
    }
  }
}

#[napi(object)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[napi(object)]
pub struct Size {
  pub width: i32,
  pub height: i32,
}

/// A single persistent connection to the platform's input backend (X11 on
/// Linux, Quartz Event Services on macOS, the Win32 API on Windows). Create
/// one instance and reuse it — each call previously reconnected from
/// scratch, which is both slow and, on X11, opens a new display connection
/// per call.
#[napi]
pub struct Enigo {
  inner: EnigoImpl,
  #[cfg(target_os = "macos")]
  positioned_input: macos_input::PositionedInput,
}

#[napi]
impl Enigo {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    let inner = EnigoImpl::new(&Settings::default()).map_err(to_napi_err)?;
    #[cfg(target_os = "macos")]
    let positioned_input = macos_input::PositionedInput::new(inner.get_marker_value())?;
    Ok(Self {
      inner,
      #[cfg(target_os = "macos")]
      positioned_input,
    })
  }

  /// Move the mouse cursor. `coordinate` selects whether `x`/`y` are
  /// absolute screen coordinates or relative to the current position.
  #[napi]
  pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> Result<()> {
    self
      .inner
      .move_mouse(x, y, coordinate.into())
      .map_err(to_napi_err)
  }

  #[napi]
  pub fn button(&mut self, button: MouseButton, direction: Direction) -> Result<()> {
    let button = button.to_enigo()?;
    self
      .inner
      .button(button, direction.into())
      .map_err(to_napi_err)
  }

  #[napi]
  pub fn scroll(&mut self, length: i32, axis: Axis) -> Result<()> {
    self.inner.scroll(length, axis.into()).map_err(to_napi_err)
  }

  /// The current mouse cursor position, in pixels.
  #[napi]
  pub fn location(&self) -> Result<Point> {
    let (x, y) = self.inner.location().map_err(to_napi_err)?;
    Ok(Point { x, y })
  }

  /// The (width, height) of the main display, in pixels.
  #[napi]
  pub fn main_display(&self) -> Result<Size> {
    let (width, height) = self.inner.main_display().map_err(to_napi_err)?;
    Ok(Size { width, height })
  }

  /// Send a named key event. Not every `Key` variant is available on every
  /// platform (they mirror `enigo`'s own per-platform key set) — check with
  /// `Enigo.isKeySupported` if you're not sure, or catch the error.
  #[napi]
  pub fn key(&mut self, key: Key, direction: Direction) -> Result<()> {
    let key = key.to_enigo()?;
    self.inner.key(key, direction.into()).map_err(to_napi_err)
  }

  /// Send a raw hardware keycode, bypassing keysym/layout mapping.
  #[napi]
  pub fn raw_key(&mut self, keycode: u16, direction: Direction) -> Result<()> {
    self
      .inner
      .raw(keycode, direction.into())
      .map_err(to_napi_err)
  }

  /// Type arbitrary Unicode text. Prefer this over `key`/`unicodeKey` for
  /// entering text — it uses a fast whole-string path where available.
  #[napi]
  pub fn text(&mut self, text: String) -> Result<()> {
    self.inner.text(&text).map_err(to_napi_err)
  }

  /// Send a single Unicode character as a key event (`enigo::Key::Unicode`).
  /// For typing text, use `text` instead.
  #[napi]
  pub fn unicode_key(&mut self, character: String, direction: Direction) -> Result<()> {
    let ch = character
      .chars()
      .next()
      .ok_or_else(|| Error::new(Status::InvalidArg, "character must not be empty"))?;
    self
      .inner
      .key(enigo::Key::Unicode(ch), direction.into())
      .map_err(to_napi_err)
  }

  /// Send an arbitrary key by its platform-native code (`enigo::Key::Other`):
  /// a keysym on Linux, a virtual-key code on Windows, a `KeyCode` on macOS.
  #[napi]
  pub fn other_key(&mut self, code: u32, direction: Direction) -> Result<()> {
    self
      .inner
      .key(enigo::Key::Other(code), direction.into())
      .map_err(to_napi_err)
  }

  /// Whether `key` is implemented on the current platform.
  #[napi]
  pub fn is_key_supported(key: Key) -> bool {
    key.is_supported()
  }

  /// Whether `button` is implemented on the current platform.
  #[napi]
  pub fn is_button_supported(button: MouseButton) -> bool {
    button.is_supported()
  }
}

#[cfg(target_os = "macos")]
#[napi]
impl Enigo {
  /// macOS only: post a mouse button event at explicit Quartz global coordinates.
  /// Unlike moveMouse followed by button, this does not read a pending cursor move.
  /// clickCount defaults to 1; pass 2 for the second click of a double click.
  #[napi]
  pub fn button_at(
    &mut self,
    x: i32,
    y: i32,
    button: MouseButton,
    direction: Direction,
    click_count: Option<u32>,
  ) -> Result<()> {
    self
      .positioned_input
      .button_at(x, y, button, direction, click_count.unwrap_or(1))
  }

  /// macOS only: scroll at explicit Quartz global coordinates, using the same
  /// line units and direction as scroll(). Does not depend on cursor location.
  #[napi]
  pub fn scroll_at(&mut self, x: i32, y: i32, length: i32, axis: Axis) -> Result<()> {
    self.positioned_input.scroll_at(x, y, length, axis)
  }
}

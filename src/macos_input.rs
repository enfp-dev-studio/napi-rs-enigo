use core_graphics::{
  event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, EventField, ScrollEventUnit},
  event_source::{CGEventSource, CGEventSourceStateID},
  geometry::CGPoint,
};
use foreign_types::ForeignType;
use napi::{Error, Result, Status};

use crate::{Axis, Direction, MouseButton};

// core-graphics 0.23.2 exposes event locations for mouse creation and reading,
// but omits this setter, required to target a scroll event before posting it.
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  fn CGEventSetLocation(event: core_graphics::sys::CGEventRef, location: CGPoint);
}

pub(crate) struct PositionedInput {
  source: CGEventSource,
  marker: i64,
}

impl PositionedInput {
  pub(crate) fn new(marker: i64) -> Result<Self> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
      .map_err(|_| Error::new(Status::GenericFailure, "failed creating event source"))?;
    Ok(Self { source, marker })
  }

  pub(crate) fn button_at(
    &self,
    x: i32,
    y: i32,
    button: MouseButton,
    direction: Direction,
    click_count: u32,
  ) -> Result<()> {
    for event in self.button_events(x, y, button, direction, click_count)? {
      event.post(CGEventTapLocation::HID);
    }
    Ok(())
  }

  pub(crate) fn scroll_at(&self, x: i32, y: i32, length: i32, axis: Axis) -> Result<()> {
    self
      .scroll_event(x, y, length, axis)?
      .post(CGEventTapLocation::HID);
    Ok(())
  }

  fn button_events(
    &self,
    x: i32,
    y: i32,
    button: MouseButton,
    direction: Direction,
    click_count: u32,
  ) -> Result<Vec<CGEvent>> {
    if click_count == 0 {
      return Err(Error::new(
        Status::InvalidArg,
        "clickCount must be at least 1",
      ));
    }
    // Preserve the existing platform-specific unsupported-button error.
    button.to_enigo()?;
    let (button, down, up) = match button {
      MouseButton::Left => (
        CGMouseButton::Left,
        CGEventType::LeftMouseDown,
        CGEventType::LeftMouseUp,
      ),
      MouseButton::Right => (
        CGMouseButton::Right,
        CGEventType::RightMouseDown,
        CGEventType::RightMouseUp,
      ),
      MouseButton::Middle => (
        CGMouseButton::Center,
        CGEventType::OtherMouseDown,
        CGEventType::OtherMouseUp,
      ),
      MouseButton::ScrollUp
      | MouseButton::ScrollDown
      | MouseButton::ScrollLeft
      | MouseButton::ScrollRight => {
        if direction == Direction::Release {
          return Ok(Vec::new());
        }
        let (length, axis) = match button {
          MouseButton::ScrollUp => (-1, Axis::Vertical),
          MouseButton::ScrollDown => (1, Axis::Vertical),
          MouseButton::ScrollLeft => (-1, Axis::Horizontal),
          MouseButton::ScrollRight => (1, Axis::Horizontal),
          _ => unreachable!(),
        };
        return Ok(vec![self.scroll_event(x, y, length, axis)?]);
      }
      MouseButton::Back | MouseButton::Forward => unreachable!(),
    };
    let types = match direction {
      Direction::Press => vec![down],
      Direction::Release => vec![up],
      Direction::Click => vec![down, up],
    };
    types
      .into_iter()
      .map(|event_type| {
        let event = CGEvent::new_mouse_event(
          self.source.clone(),
          event_type,
          CGPoint::new(f64::from(x), f64::from(y)),
          button,
        )
        .map_err(|_| {
          Error::new(
            Status::GenericFailure,
            "failed creating positioned mouse event",
          )
        })?;
        event.set_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE, i64::from(click_count));
        event.set_integer_value_field(EventField::EVENT_SOURCE_USER_DATA, self.marker);
        Ok(event)
      })
      .collect()
  }

  fn scroll_event(&self, x: i32, y: i32, length: i32, axis: Axis) -> Result<CGEvent> {
    let delta = length
      .checked_neg()
      .ok_or_else(|| Error::new(Status::InvalidArg, "scroll length cannot be i32::MIN"))?;
    let (wheels, vertical, horizontal) = match axis {
      Axis::Vertical => (1, delta, 0),
      Axis::Horizontal => (2, 0, delta),
    };
    let event = CGEvent::new_scroll_event(
      self.source.clone(),
      ScrollEventUnit::LINE,
      wheels,
      vertical,
      horizontal,
      0,
    )
    .map_err(|_| {
      Error::new(
        Status::GenericFailure,
        "failed creating positioned scroll event",
      )
    })?;
    // Set the event's own coordinates instead of reading the asynchronously
    // updated cursor or moving it and hoping WindowServer processed the move.
    unsafe {
      CGEventSetLocation(event.as_ptr(), CGPoint::new(f64::from(x), f64::from(y)));
    }
    event.set_integer_value_field(EventField::EVENT_SOURCE_USER_DATA, self.marker);
    Ok(event)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn point(event: &CGEvent) -> (f64, f64) {
    let point = event.location();
    (point.x, point.y)
  }

  fn input() -> PositionedInput {
    PositionedInput::new(1729).unwrap()
  }

  #[test]
  fn click_events_preserve_coordinates_button_count_and_marker() {
    for (button, down, up, number) in [
      (
        MouseButton::Left,
        CGEventType::LeftMouseDown,
        CGEventType::LeftMouseUp,
        0,
      ),
      (
        MouseButton::Right,
        CGEventType::RightMouseDown,
        CGEventType::RightMouseUp,
        1,
      ),
      (
        MouseButton::Middle,
        CGEventType::OtherMouseDown,
        CGEventType::OtherMouseUp,
        2,
      ),
    ] {
      let events = input()
        .button_events(-1040, 289, button, Direction::Click, 2)
        .unwrap();
      assert_eq!(events.len(), 2);
      for (event, kind) in events.iter().zip([down, up]) {
        assert_eq!(event.get_type() as u32, kind as u32);
        assert_eq!(point(event), (-1040.0, 289.0));
        assert_eq!(
          event.get_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER),
          number
        );
        assert_eq!(
          event.get_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE),
          2
        );
        assert_eq!(
          event.get_integer_value_field(EventField::EVENT_SOURCE_USER_DATA),
          1729
        );
      }
    }
  }

  #[test]
  fn press_and_release_each_create_only_the_requested_event() {
    for (direction, kind) in [
      (Direction::Press, CGEventType::LeftMouseDown),
      (Direction::Release, CGEventType::LeftMouseUp),
    ] {
      let events = input()
        .button_events(40, -20, MouseButton::Left, direction, 1)
        .unwrap();
      assert_eq!(events.len(), 1);
      assert_eq!(events[0].get_type() as u32, kind as u32);
      assert_eq!(point(&events[0]), (40.0, -20.0));
    }
  }

  #[test]
  fn scroll_events_preserve_coordinates_axis_sign_and_marker() {
    for axis in [Axis::Vertical, Axis::Horizontal] {
      for length in [-40, 0, 40] {
        let event = input().scroll_event(-700, 762, length, axis).unwrap();
        assert_eq!(event.get_type() as u32, CGEventType::ScrollWheel as u32);
        assert_eq!(point(&event), (-700.0, 762.0));
        let vertical = if axis == Axis::Vertical { -length } else { 0 };
        let horizontal = if axis == Axis::Horizontal { -length } else { 0 };
        assert_eq!(
          event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1),
          i64::from(vertical)
        );
        assert_eq!(
          event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_2),
          i64::from(horizontal)
        );
        assert_eq!(
          event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_IS_CONTINUOUS),
          0
        );
        assert_eq!(
          event.get_integer_value_field(EventField::EVENT_SOURCE_USER_DATA),
          1729
        );
      }
    }
  }

  #[test]
  fn scroll_button_release_is_noop_and_press_uses_requested_location() {
    for button in [
      MouseButton::ScrollUp,
      MouseButton::ScrollDown,
      MouseButton::ScrollLeft,
      MouseButton::ScrollRight,
    ] {
      assert!(input()
        .button_events(5, 6, button, Direction::Release, 1)
        .unwrap()
        .is_empty());
      let events = input()
        .button_events(5, 6, button, Direction::Click, 1)
        .unwrap();
      assert_eq!(events.len(), 1);
      assert_eq!(events[0].get_type() as u32, CGEventType::ScrollWheel as u32);
      assert_eq!(point(&events[0]), (5.0, 6.0));
    }
  }

  #[test]
  fn invalid_arguments_fail_before_any_posting() {
    assert!(input()
      .button_events(0, 0, MouseButton::Left, Direction::Click, 0)
      .is_err());
    assert!(input()
      .button_events(0, 0, MouseButton::Back, Direction::Click, 1)
      .is_err());
    assert!(input()
      .scroll_event(0, 0, i32::MIN, Axis::Vertical)
      .is_err());
  }
}

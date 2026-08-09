# napi-rs-enigo: Node.js Wrapper for Enigo

A Node.js wrapper for the Enigo Rust library, providing a native interface for simulating input events like keyboard and mouse actions.

## Features

- [x] Simulate mouse events (move, click, scroll, location, display size).
- [x] Simulate keyboard events (press, release, type, raw keycodes).

## Requirements

- Node.js 10 or later
- Rust toolchain

## Installation

Use npm to install the library:

```shell
yarn add @enfpdev/napi-rs-enigo
```

## Usage

`Enigo` holds one persistent connection to the platform's input backend —
create an instance once and reuse it, rather than constructing one per call.

```typescript
import { Enigo, Key, MouseButton, Direction, Coordinate, Axis } from '@enfpdev/napi-rs-enigo'

const enigo = new Enigo()

enigo.moveMouse(100, 100, Coordinate.Rel)
enigo.moveMouse(100, 100, Coordinate.Abs)
enigo.button(MouseButton.Right, Direction.Click)
enigo.button(MouseButton.Left, Direction.Press)
enigo.button(MouseButton.Middle, Direction.Release)
enigo.scroll(100, Axis.Vertical)

enigo.location() // -> { x, y }
enigo.mainDisplay() // -> { width, height }

enigo.text('hello world')
enigo.key(Key.Return, Direction.Click)
enigo.rawKey(36, Direction.Click) // hardware keycode, bypasses keysym/layout mapping
enigo.unicodeKey('é', Direction.Click) // single Unicode char, for text() use `text` instead
enigo.otherKey(0x1234, Direction.Click) // platform-native key code (keysym / VK / KeyCode)

// Key and MouseButton cover every variant enigo knows about across all
// platforms, so a given variant may not exist on the platform you're
// running on. Check first, or catch the error it throws:
if (Enigo.isKeySupported(Key.MissionControl)) {
  enigo.key(Key.MissionControl, Direction.Click)
}
if (Enigo.isButtonSupported(MouseButton.Back)) {
  enigo.button(MouseButton.Back, Direction.Click)
}
```

### Upgrading from 1.1.x

1.2.0 replaces the six free functions (`moveMouseRel`, `moveMouseAbs`,
`mouseClick`, `mouseDown`, `mouseUp`, `mouseScroll`) with the `Enigo` class
above — each of those functions used to open a fresh connection to the
platform's input backend on every call. There's no compatibility shim; update
call sites to construct one `Enigo` instance and call its methods instead.

| 1.1.x                             | 1.2.0                                                                |
| --------------------------------- | -------------------------------------------------------------------- |
| `moveMouseRel(x, y)`              | `enigo.moveMouse(x, y, Coordinate.Rel)`                              |
| `moveMouseAbs(x, y)`              | `enigo.moveMouse(x, y, Coordinate.Abs)`                              |
| `mouseClick(button)`              | `enigo.button(MouseButton.X, Direction.Click)`                       |
| `mouseDown(button)`               | `enigo.button(MouseButton.X, Direction.Press)`                       |
| `mouseUp(button)`                 | `enigo.button(MouseButton.X, Direction.Release)`                     |
| `mouseScroll(length, isVertical)` | `enigo.scroll(length, isVertical ? Axis.Vertical : Axis.Horizontal)` |

## Contribute

Coffee fuels coding ☕️

<p align="center">
<a href="https://www.buymeacoffee.com/enfpdev" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>
</p>

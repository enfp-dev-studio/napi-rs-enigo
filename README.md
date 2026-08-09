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

// Key covers every key enigo knows about across all platforms, so a given
// variant may not exist on the platform you're running on. Check first,
// or catch the error it throws:
if (Enigo.isKeySupported(Key.MissionControl)) {
  enigo.key(Key.MissionControl, Direction.Click)
}
```

## Contribute

Coffee fuels coding ☕️

<p align="center">
<a href="https://www.buymeacoffee.com/enfpdev" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>
</p>

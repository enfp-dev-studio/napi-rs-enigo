# napi-rs-enigo: Node.js Wrapper for Enigo

A Node.js wrapper for the Enigo Rust library, providing a native interface for simulating input events like keyboard and mouse actions.

## Features

- [x] Simulate mouse events (move, click, scroll).
- [ ] Simulate keyboard events (press, release, type).

## Requirements

- Node.js 10 or later
- Rust toolchain

## Installation

Use npm to install the library:

```shell
yarn add @enfpdev/napi-rs-enigo
```

## Usage

```typescript
import {
  moveMouseRel,
  moveMouseAbs,
  mouseClick,
  mouseDown,
  mouseUp,
  mouseScroll
} from "napi-rs-enigo"

moveMouseRel(100, 100));
moveMouseAbs(100, 100));
mouseClick('rigtht'));
mouseDown('left'));
mouseUp('middle'));
mouseScroll(100, true));

```

## Contribute

Coffee fuels coding ☕️
<p align="center">
<a href="https://www.buymeacoffee.com/enfpdev" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>
</p>

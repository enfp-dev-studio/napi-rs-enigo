import test from 'ava'

import { Enigo, Key, MouseButton, Direction, Coordinate, Axis } from '../index'

test('mouse and keyboard events via a persistent Enigo instance', (t) => {
  const enigo = new Enigo()

  t.is(enigo.moveMouse(100, 100, Coordinate.Rel), undefined)
  t.is(enigo.moveMouse(100, 100, Coordinate.Abs), undefined)
  t.is(enigo.button(MouseButton.Right, Direction.Click), undefined)
  t.is(enigo.button(MouseButton.Left, Direction.Press), undefined)
  t.is(enigo.button(MouseButton.Middle, Direction.Release), undefined)
  t.is(enigo.scroll(100, Axis.Vertical), undefined)
  t.is(enigo.text('hi'), undefined)
  t.is(enigo.key(Key.Return, Direction.Click), undefined)

  const location = enigo.location()
  t.is(typeof location.x, 'number')
  t.is(typeof location.y, 'number')

  const display = enigo.mainDisplay()
  t.true(display.width > 0)
  t.true(display.height > 0)
})

test('Enigo.isKeySupported reflects the current platform', (t) => {
  t.true(Enigo.isKeySupported(Key.Return))
})

test('calling an unsupported key throws instead of failing silently', (t) => {
  const enigo = new Enigo()
  const unsupported = ([Key.Num0, Key.MissionControl] as Key[]).find((key) => !Enigo.isKeySupported(key))

  t.truthy(unsupported)
  t.throws(() => enigo.key(unsupported as Key, Direction.Click))
})

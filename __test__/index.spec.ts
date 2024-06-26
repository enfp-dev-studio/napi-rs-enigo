import test from 'ava'

import { moveMouseRel, moveMouseAbs, mouseClick, mouseDown, mouseUp, mouseScroll } from '../index'

test('sync function from native code', (t) => {
  const x = 100
  const y = 100
  const button = 'right'
  const length = 100
  const isVertical = true

  // console.log(moveMouseRel(x, y))
  // console.log(moveMouseAbs(x, y))
  // console.log(mouseClick(button))
  // console.log(mouseDown(button))
  // console.log(mouseUp(button))
  // console.log(mouseScroll(length, isVertical))

  t.is(moveMouseRel(x, y), undefined)
  t.is(moveMouseAbs(x, y), undefined)
  t.is(mouseClick(button), undefined)
  t.is(mouseDown(button), undefined)
  t.is(mouseUp(button), undefined)
  t.is(mouseScroll(length, isVertical), undefined)
})

import { moveMouseRel, moveMouseAbs, mouseClick, mouseDown, mouseUp, mouseScroll } from './index.js'

const x = 100
const y = 100
const button = 'right'
const length = 100
const isVertical = true

console.log(moveMouseRel(x, y))
console.log(moveMouseAbs(x, y))
console.log(mouseClick(button))
console.log(mouseDown(button))
console.log(mouseUp(button))
console.log(mouseScroll(length, isVertical))

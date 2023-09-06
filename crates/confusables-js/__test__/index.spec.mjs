import test from 'ava'

import {
  containsConfusable,
  isConfusableWith,
  confusableContains,
  replaceConfusable,
  detectReplaceConfusable,
} from '../index.js'
 
let a = "𝓗℮𝐥1೦"
let b = "Hello"

test('containsConfusable', (t) => {
  t.true(containsConfusable(a))
  t.true(!containsConfusable(b))
})

test('isConfusableWith', (t) => {
  t.true(isConfusableWith(a, b))
  t.true(!isConfusableWith(b, a))
})

test('confusableContains', (t) => {
  t.true(confusableContains(a, b))
})

test('replaceConfusable', (t) => {
  t.is(replaceConfusable(a), b)
  t.is(detectReplaceConfusable(a), b)
})

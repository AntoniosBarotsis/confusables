import {
  containsConfusable,
  isConfusableWith,
  confusableContains,
  replaceConfusable,
  detectReplaceConfusable,
} from './index.js'
 
import { strict as assert } from 'assert';

let a = "𝓗℮𝐥1೦"
let b = "Hello"

assert(containsConfusable(a))
assert(!containsConfusable(b))

assert(isConfusableWith(a, b))

assert(confusableContains(a, b))

assert(replaceConfusable(a) === b)
assert(detectReplaceConfusable(a) === b)

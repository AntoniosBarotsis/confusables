const confusables = require('.');
const assert = require('node:assert').strict;

let a = "𝓗℮𝐥1೦"
let b = "Hello"

assert(confusables.contains_confusable(a))
assert(!confusables.contains_confusable(b))

assert(confusables.is_confusable_with(a, b))

assert(confusables.confusable_contains(a, b))

assert(confusables.replace_confusable(a) === b)
assert(confusables.detect_replace_confusable(a) === b)

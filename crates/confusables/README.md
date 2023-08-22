# Confusables

A Rust crate that provides utilities around Unicode confusables/homoglyphs. Specifically about:

- detecting confusable characters in strings
- replacing confusable characters (according to the 
  [data file](http://www.unicode.org/Public/security/revision-03/confusables.txt) specified in the
  [Unicode® Technical Standard](https://www.unicode.org/reports/tr39/))
- confusable-resilient operations (equality, contains) on strings

## Get Started

All the new methods are implemented as part of a trait which you will need to import:

```rust
use confusables::Confusable;
```

All methods come with code examples so be sure to check those!

<!-- TODO: Add docs.rs link -->

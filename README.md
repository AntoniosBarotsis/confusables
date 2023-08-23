# Confusables

This repo contains:

- [`./crates/confusables`](./crates/confusables): A rust library with Unicode confusable utils
- [`./crates/confusables-js`](./crates/confusables-js): Node bindings for the `confusables` library
- [`./crates/is-rtl`](./crates/is-rtl): For whatever reason, I made a function that detects
  right-to-left characters

If you are interested on the Rust library specifically, checkout
[its README](./crates/confusables/README.md).

## Why?

[Text is kinda interesting](https://youtu.be/_mZBa3sqTrI) and I also wanted to experiment with
compile-time stuff like parsing and rust-node bindings.

## Should You Use This?

Well, I certainly don't mind!

Using it in Rust especially is trivial as it's just another crate. When it comes to Node, I was
too lazy to figure out how to properly publish the binary on NPM. That said you can still easily
build it yourself as long as you have the [Rust installed](https://www.rust-lang.org/tools/install).
Somewhat more detailed instructions can be found in [the README](./crates/confusables-js/README.md).

Would I recommend this for production in a Node project? Probably not, just because of the annoying
set-up alone. Perhaps it would be worth checking out
[`decancer`](https://github.com/null8626/decancer) (although to me, it is a lot less clear what
characters it covers compared to my version but oh well).

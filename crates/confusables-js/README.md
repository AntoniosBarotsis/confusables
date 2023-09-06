# confusables-js

Node bindings for the [`confusables`](../confusables/README.md) library.

> A Rust crate that provides utilities around Unicode confusables/homoglyphs. Specifically about:
> 
> - detecting confusable characters in strings
> - replacing confusable characters (according to the 
>   [data file](http://www.unicode.org/Public/security/revision-03/confusables.txt) specified in the
>   [Unicode® Technical Standard](https://www.unicode.org/reports/tr39/))
> - confusable-resilient operations (equality, contains) on strings

For usage examples, see [here](./__test__/index.spec.mjs).

Install with

```sh
npm i @antonisbarotsis/confusables-js
```

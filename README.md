# Simple Tree

Simple tree provides a simple interface to build printable trees.

Inspired by the `tree`.

## Implementation ideas:

- A tree where every node needs to be created manually and children added directly on nodes
  - Perhaps this should be the default used in the `Node` examples...
- Filesystem tree
  - Should be trivial to implement the default `tree` program
- Trie of words with counts, where each leaf is a full word and its ancestors are partial prefixes
  - Able to import a collection of words and get a count of occurrences of each word
- A Python/JS-style dict: `{"I'm the root": [{"foo": ["bar", "baz"]}, "fizz", "buzz"]}`
- A Scheme-style list: `("I'm the root" ("foo" "bar" "baz") "fizz" "buzz")`
  - Or maybe Scheme would look more like this: `("I'm the root" (("foo" ("bar" "baz")) "fizz" "buzz"))` (I prefer the former)

## License

This work is licensed under the Apache License 2.0.

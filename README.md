# cssOptiAdvisor

## Intro

**cssOptiAdvisor** is a tool for providing optimization suggestions for CSS.

## Features

- [x] keyframes : Suggest the value `0%` instead of `from`. (we save 1 character)
- [x] keyframes : Suggest the value `to` instead of `100%`. (we save 2 character)
- [x] keyframes : Suggest eliminating duplicate keys.

## Launch

Example :

```rust
cargo run -- --path tests/datasets/keyframes/duplicates.css
```

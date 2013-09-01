Port of [simplify-js](https://github.com/mourner/simplify-js) to [rust](https://github.com/mozilla/rust), compile with:

```bash
rustc --lib simplify.rs
rustc main.rs -L .
```

run with `./main points.json lessPoints.json 0.8`




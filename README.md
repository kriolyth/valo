<div align="center">

  <h1><code>valo</code></h1>

  <strong>An étude in WASM 🕸 flavour of Rust 🦀</strong>

</div>

## About

This is an experiment in making a WebGL graphics application with WASM backend, compiled from Rust code.

## Compile

Prerequisites: [Rust](https://www.rust-lang.org/learn/get-started), [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), [node.js](https://nodejs.org/en/) and [npm](https://www.npmjs.com).

1. Ensure you have wasm target installed:
```
rustup target add wasm32-unknown-unknown
```

2. Build WASM code
```
wasm-pack build
```

3. Download / update npm dependencies
```
cd www
npm ci
```

4. Start in dev mode
```
npm run start
```

5. Build complete page
```
npm run build
```

## Usage

Add two spoonfuls of colour. Stir, enjoy.

## License

Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE)).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.

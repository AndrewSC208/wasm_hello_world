# wasm_hello_world

A small litte tutorial to start learning rust and WebAssembly

## Setup
1. install wasm-pack
```sh
cargo install wasm-pack
```

2. Since we will be publishing a new npm module, we will need to signin to npm
```sh
npm adduser
```

## Tutorial
1. Create new lib 
```sh
cargo new --lib hello-wasm
```

2. Add code
```rs
# src/lib.rs
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A sample project with wasm-pack"
license = "MIT/Apache-2.0"
repository = "https://github.com/yourgithubusername/hello-wasm"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

3. Build Package
```sh
wasm-pack build --scope mynpmusername
```

4. Publish Package
```sh
cd pkg
npm publish --access=public
```

## Test
I am not going to review how to build the web server, or anything involved on that side of things. 
So you can run ```npm install``` then ```npm run serve``` to run the demo and navigate to ```localhost.8080``` to test out the alert function.

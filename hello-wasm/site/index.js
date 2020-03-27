const js = import("./node_modules/@andrewsc208/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});

First day solutions use Rust as a source language, which so far was quite easy to use.

You need to have `wasm-pack` installed, as well as the standart Rust toolchain.

I followed [these instructions](https://rustwasm.github.io/book/game-of-life/setup.html).

When Rust is compiled to Wasm, it generates a JS file `{name}_bg.js` in addition to the Wasm binary. Functions from that file need to be manually translated to Lia scripts. Try building Rust and comparing JS to Lia scripts in the generators: you will see that they are very similar (except for `passArray8ToWasm0` missing in the generator, which I inlined for brevity).
Day 6 uses Go compiled to Wasm with [TinyGo](tinygo.org). Regular Go compiler doesn't work well with urwasm because the latter only provides Kleisli arrows of Lia state monad `(list coin-wasm)=>(list coin-wasm)`, i.e. urwasm can only import functions that do not affect the state of your urbit (but can affect Wasm VM state). And binaries compiled with Go attempt to provide function references by calling stateful imported functions when `main()` is executed.

With TinyGo export statements are provided in a comment above the function.

To compile:

```
tinygo build -o 6-2.wasm 6-2.go
```
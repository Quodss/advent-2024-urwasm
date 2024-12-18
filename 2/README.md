This day's solution uses Zig. It is fairly easy to interact with Zig code, but the Wasm module doesn't provide an allocator. So we have to write the allocator ourselves, although in this case it's enough to provide a pointer to a byte buffer.

To build Wasm binary, do e.g.

```
zig build-exe 2-2.zig -target wasm32-freestanding -fno-entry --export=solve --export=get_arena
```

This example also features import handling in urwasm: I needed to debug something so I added a `print` import. Imports are provided in form of a map from pairs of cords to gates from `(list coin-wasm)` to Lia scripts that return `(list coin-wasm)`
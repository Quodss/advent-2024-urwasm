This day features C compiled with Emscripten.

To compile:

```
emcc main.c -o output.js -s EXPORTED_FUNCTIONS='["_solve", "_malloc"]'
```

For some reason Emscripten didn't want to generate functions that return u64, so I had to return it by reference.
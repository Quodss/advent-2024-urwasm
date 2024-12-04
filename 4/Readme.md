Today's solutution is written in AssemblyScript, which is a TypeScript dialect with stricter memory access rules and slightly different standart library.

To install AssemblyScript:

```
npm install assemblyscript
```

To compile the solutions:

```
asc aoc-4.ts --outFile aoc-4.wasm --optimize --bindings raw
```

This will also generate a JS file with bindings that will have to be translated to Lia scripts to interface the code. You can compare code from aoc-4.js with the binding I have in the generators for reference.
UPDATE: The presented solutions use older version of urwasm that had different types, so the Hoon part won't compile now. Breaking change: https://github.com/urbit/urbit/pull/7128/commits/d60b9bef289fb72aeda7cf96d4a8bb5f25094b36

Urwasm is now a part of `%base` desk distribution and the jets are included in the Vere binary release, so copying `/sur/wasm`, `/lib/wasm` and `/mar/wasm/hoon` from `%base` to your desk (or adding generators to `%base` desk instead) is enough.

---

These solutions to AoC puzzles in Hoon use [`urwasm`](https://github.com/Quodss/urwasm/tree/main).

(`$` denotes Unix terminal, `>` denotes Dojo)

To get the `wasm` deck with Hoon side:

```
$ git clone git@github.com:Quodss/urwasm.git
```

To build Vere fork with the jet and boot a fakezod:

```
$ git clone git@github.com:Quodss/vere.git
$ cd vere
$ git checkout lia-monad
$ zig build -Doptimize=ReleaseFast
$ cd ..
$ ./vere/zig-out/x86_64-linux-musl/urbit --loom 33 -F zod
```

Refer to Vere [build instructions](https://github.com/urbit/vere/blob/develop/INSTALL.md) if necessary.

Make a new desk `%wasm` and copy `desk` contents to it:

```
> |new-desk %wasm
> |mount %wasm
::  exit dojo
$ cp -r urwasm/desk/* zod/wasm/.
```

Return to dojo, commit the files, and run the tests to check that urwasm is installed correctly:

```
$ ./vere/zig-out/x86_64-linux-musl/urbit --loom 33 zod
::  back in dojo
> |commit %wasm
> -test /=wasm=/tests ~
```

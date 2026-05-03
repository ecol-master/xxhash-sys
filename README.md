# xxhash-sys

Raw Rust FFI bindings to the [xxhash](https://github.com/cyan4973/xxhash) C library.
It does not provide a safe Rust wrapper API; callers are responsible
for upholding the safety requirements documented by the upstream xxHash C API.

## Build Requirements

- `libxxhash` development headers, version 0.8.x
- `pkg-config` configured to find `libxxhash`
- `libclang`, required by `bindgen`

On macOS with Homebrew:

```sh
brew install xxhash pkg-config llvm
```

On Debian or Ubuntu:

```sh
sudo apt install libxxhash-dev pkg-config clang
```

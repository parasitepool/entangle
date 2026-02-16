<h1 align=center><code>entangle</code></h1>

<div align=center>
  <a href=https://crates.io/crates/entangle>
    <img src=https://img.shields.io/crates/v/entangle.svg alt="crates.io version">
  </a>
  <a href=https://github.com/parasitepool/entangle/actions/workflows/ci.yaml>
    <img src=https://github.com/parasitepool/entangle/actions/workflows/ci.yaml/badge.svg alt="build status">
  </a>
  <a href=https://github.com/parasitepool/entangle/releases>
    <img src=https://img.shields.io/github/downloads/parasitepool/entangle/total.svg alt=downloads>
  </a>
</div>
<br>

`entangle` builds Bitcoin swap transactions. Given two UTXOs owned by different parties, it produces a PSBT that safely exchanges them — party A signs, then party B signs, and the swap is complete.

## Usage

As a **library**:

```rust
use entangle::swap::{build_swap_psbt, SwapRequest};

let psbt = build_swap_psbt(&request)?;
```

As an **API server**:

```sh
entangle api --bind 0.0.0.0:3000
```

As a **frontend server** (with optional embedded API):

```sh
entangle server --bind 0.0.0.0:8080 --with-api
```

## Development

Requires [Hermit](https://cashapp.github.io/hermit/) for toolchain management.

```sh
just init   # set up environment
just dev    # run the app
just ci     # clippy + format check + tests
```
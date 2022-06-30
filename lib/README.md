# Bundled msdfgen library

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Crates.io Package](https://img.shields.io/crates/v/msdfgen-lib.svg?style=popout)](https://crates.io/crates/msdfgen-lib)
[![Docs.rs API Docs](https://docs.rs/msdfgen-lib/badge.svg)](https://docs.rs/msdfgen-lib)
[![Travis-CI Status](https://travis-ci.com/katyo/msdfgen-rs.svg?branch=master)](https://travis-ci.com/katyo/msdfgen-rs)

This crate provides bundled [msdfgen](https://github.com/Chlumsky/msdfgen) library for using with [__msdfgen__](https://crates.io/crates/msdfgen) crate.

## Usage

You can simply add this as dependency to your manifest:

```toml
[dependencies]
msdfgen = "^0.1"

# Use bundled library to avoid unresolved links
msdfgen-lib = "^0.1"
```

Next you should say compiler that you want to use that crate:

```rust
// Either in traditional manner
extern crate msdfgen_lib;

// Or in Rust2018 manner
use msdfgen_lib as _;
```

## Features

You can apply some customizations to library using those features:

- __shared__ Force bundle shared (or dynamic) library instead of static (default)
- __libcxx__ Link with _libc++_ instead of _libstdc++_ (default)
- __stdcxx-static__ Link with static C++ stdlib instead of shared (default)

# Rust_Yew_FrontEnd

- yew[![crates.io](https://img.shields.io/crates/v/yew.svg)](https://crates.io/crates/yew)![Crates.io](https://img.shields.io/crates/l/yew)![wasmtimeDownloads](https://img.shields.io/crates/d/yew.svg)<a href="https://github.com/yewstack/yew"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>![druidstar](https://img.shields.io/github/stars/yewstack/yew.svg)

  - <p dir="auto"><a href="https://yew.rs/" rel="nofollow">Website</a> | <a href="https://yew.rs/docs/getting-started/introduction" rel="nofollow">Guides</a>

  - test 할 수 있는 playground 먼저 해 보고 코드 만들자

    - https://play.yew.rs/

    https://github.com/yewstack/yew

<br>

<hr>

# trunk

```
trunk serve --open
```


- Build, bundle & ship your Rust WASM application to the web

https://github.com/thedodd/trunk


# Setting up

- Prerequisites

This tutorial assumes you're already familiar with Rust. If you're new to Rust, the free Rust Book offers a great starting point for beginners and continues to be an excellent resource even for experienced Rust developers.

Ensure the latest version of Rust is installed by running rustup update or by installing rust if you haven't already done so.

After installing Rust, you can use Cargo to install trunk by running:

```
cargo install trunk
```

We will also need to add the WASM build target by running:

```
rustup target add wasm32-unknown-unknown
```

https://yew.rs/docs/tutorial

<hr>

## trunk install

```
# Install via homebrew on Mac, Linux or Windows (WSL).
brew install trunk

# Install a release binary (great for CI).
# You will need to specify a value for ${VERSION}.
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


# Install via cargo.
cargo install --locked trunk
# Until wasm-bindgen has pre-built binaries for Apple M1, M1 users will
# need to install wasm-bindgen manually.
cargo install --locked wasm-bindgen-cli




```

https://trunkrs.dev/#install

<hr>

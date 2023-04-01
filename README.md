# Rust_Yew_FrontEnd<a href="https://yew.rs/"><img align="left" alt="rustyew" width="26px" src="https://user-images.githubusercontent.com/67513038/227089073-47d3e238-107f-4545-bf41-9d6c24951f35.png" /></a><a href="https://www.rust-lang.org/"><img align="left" alt="rust1" width="26px" src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" /></a>

- yew[![crates.io](https://img.shields.io/crates/v/yew.svg)](https://crates.io/crates/yew)![Crates.io](https://img.shields.io/crates/l/yew)![wasmtimeDownloads](https://img.shields.io/crates/d/yew.svg)<a href="https://github.com/yewstack/yew"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>![druidstar](https://img.shields.io/github/stars/yewstack/yew.svg)

  - <p dir="auto"><a href="https://yew.rs/" rel="nofollow">Website</a> | <a href="https://yew.rs/docs/getting-started/introduction" rel="nofollow">Guides</a>

  - test 할 수 있는 playground 먼저 해 보고 코드 만들자

    - https://play.yew.rs/

    https://github.com/yewstack/yew

## Yew Tutorial<a href="https://yew.rs/"><img align="left" alt="rustyew" width="26px" src="https://user-images.githubusercontent.com/67513038/227089073-47d3e238-107f-4545-bf41-9d6c24951f35.png" /></a>

https://yew.rs/docs/tutorial

https://yew.rs/docs/getting-started/introduction

- The Yew Tutorial | chris biscardi

- 1. https://youtu.be/S-O9QkrlfYw

- Series

  - https://youtube.com/playlist?list=PLWtPciJ1UMuBpRg1KbXqxE5WOdY9ze37S

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

# Tokio

- tokio.rs[![crates.io](https://img.shields.io/crates/v/tokio.svg)](https://crates.io/crates/tokio)![Crates.io](https://img.shields.io/crates/l/tokio)![wasmtimeDownloads](https://img.shields.io/crates/d/tokio.svg)<a href="https://github.com/tokio-rs/tokio"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>![druidstar](https://img.shields.io/github/stars/tokio-rs/tokio.svg)

  - A runtime for writing reliable asynchronous applications with Rust. Provides I/O, networking, scheduling, timers, ...

  - <p dir="auto"><a href="https://tokio.rs" rel="nofollow">Website</a> | <a href="https://tokio.rs/tokio/tutorial" rel="nofollow">Guides</a> | <a href="https://docs.rs/tokio/latest/tokio" rel="nofollow">API Docs</a> | <a href="https://discord.gg/tokio" rel="nofollow">Chat</a></p>

    A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. It is:

  - Fast: Tokio's zero-cost abstractions give you bare-metal performance.

  - Reliable: Tokio leverages Rust's ownership, type system, and concurrency model to reduce bugs and ensure thread safety.

  - Scalable: Tokio has a minimal footprint, and handles backpressure and cancellation naturally.

    - https://github.com/tokio-rs/tokio

# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

# Install

https://tauri.app/

```
cargo install create-tauri-app
cargo create-tauri-app

PS D:\my_tutorial> cargo create-tauri-app
✔ Project name · clickme
✔ Choose which language to use for your frontend · Rust - (cargo)
✔ Choose your UI template · Yew - (https://yew.rs/)

Template created!

Your system is missing dependencies (or they do not exist in $PATH):
╭───────────┬───────────────────────────────╮
│ Tauri CLI │ Run `cargo install tauri-cli` │
╰───────────┴───────────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/v1/guides/getting-started/prer
equisites, then run:

  cd clickme
  cargo tauri dev

PS D:\my_tutorial> cargo install tauri-cli

```

<hr>
<hr>

# Yew Turoial(Rust)

https://yew.rs/docs/tutorial

https://yew.rs/docs/getting-started/introduction

# The Yew Tutorial | chris biscardi

https://youtu.be/S-O9QkrlfYw

- Series

https://youtube.com/playlist?list=PLWtPciJ1UMuBpRg1KbXqxE5WOdY9ze37S

# yew(Rust) snippet Setting(VSCode)

https://yew.rs/docs/getting-started/editor-setup

- Snippets:Configure User
- New Snippet files for

Navigate to File > Preferences > User Snippets.
Select Rust as the language.
Add the following snippet in the snippet JSON file:

- prefix `yewfc` , prefix`yewsc`

```
{
    "New Yew function component": {
        "prefix": "yewfc",
        "body": [
            "#[derive(PartialEq, Properties)]",
            "pub struct ${1:ComponentName}Props {}",
            "",
            "#[function_component]",
            "pub fn $1(props: &${1}Props) -> Html {",
            "    let ${1}Props {} = props;",
            "    html! {",
            "        <${2:div}>$0</${2}>",
            "    }",
            "}"
        ],
        "description": "Create a minimal Yew function component"
    },
    "New Yew struct component": {
        "prefix": "yewsc",
        "body": [
            "pub struct ${1:ComponentName};",
            "",
            "pub enum ${1}Msg {",
            "}",
            "",
            "impl Component for ${1} {",
            "    type Message = ${1}Msg;",
            "    type Properties = ();",
            "",
            "    fn create(ctx: &Context<Self>) -> Self {",
            "        Self",
            "    }",
            "",
            "    fn view(&self, ctx: &Context<Self>) -> Html {",
            "        html! {",
            "            $0",
            "        }",
            "    }",
            "}"
        ],
        "description": "Create a new Yew component with a message enum"
    }
}
```

# VS Code

## Rust-Yew extension

https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew

This is a work in progress, and community maintained project! Please see details and direct related bug reports / issues / questions over to the extension's repository

Rust-Yew extension is avaliable on VSC Marketplace, providing syntax highlight, renames, hover, and more.

Emmet support should work out of the box, if not please fall back to edditing the settings.json file:

```
"emmet.includeLanguages": {
    "rust": "html",
}
```

# Thanks to

https://github.com/ShironCat/ShironCat.github.io

- Series

  - https://youtube.com/playlist?list=PLWtPciJ1UMuBpRg1KbXqxE5WOdY9ze37S

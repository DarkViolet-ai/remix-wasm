# Integrating wasm built from rust into a remix app

Below is a blog article from https://darkviolet.ai that describes the process for integrating a wasm module built from rust into a remix app. This repository contains the code for the example. To run this example, clone the repository and run the following commands:

```bash
npm install
npm run dev
```

Note that your rust toolchain must be installed and up to date.

Navigate to http://localhost:3000/game-of-life to see the game of life example.

Navigate to http://localhost:3000/breakout to see the breakout game example.

## Intro

This is a guide on how to integrate a wasm module built from rust into a remix app. All of the code for this example can be found at https://github.com/DarkViolet-ai/remix-wasm

## Setting up the project

Make sure your rust installation is up to date with rustup.

```bash
rustup update
```

### Create the project

Start by creating the new remix project

```bash
npx create-remix@latest  --app-name=remix-wasm
```

Intall the vite-plugin-wasm package

```bash
npm install vite-plugin-wasm
```

You are now ready to start building your wasm module.

### Setting up the wasm module

Create a new rust project

```bash
cargo new rust-wasm-lib --lib
```

Edit your cargo toml file to include whatever you need for your wasm module. For the game of life example, the cargo.toml will look like this:

```toml
[package]
name = "rust-wasm-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { version = "0.2.93", features = ["serde-serialize"] }
wasm-bindgen-futures = "0.4.33"
web-sys = { version = "0.3.70", features = [
    "Window", "Document", "HtmlCanvasElement", "CanvasRenderingContext2d",
    "AudioContext", "AnalyserNode", "MediaDevices", "Navigator",
    "MediaStreamConstraints", "MediaStream", "MediaStreamAudioSourceNode"
] }
js-sys = "0.3.70"
plotters = "0.3.5"
plotters-canvas = "0.3.0"
wee_alloc = "0.4.5"
rand = "0.8.5"
getrandom = { version = "0.2", features = ["js"] }

[profile.release]
panic = "abort"
```

For this example, I started with an implementation of the game of life. This code was generated in collaboration with Claude Sonnet 3.5 using the cursor IDE. I won't paste all of that code in here, but you can review it in the github repo.( https://github.com/DarkViolet-ai/remix-wasm/blob/main/rust-wasm-lib/src/lib.rs). This is a simple implementation whose main purpose is to give us some eye candy to work with in the remix app.

Let's set up the scripts in the package.json for the remix app.

```json
"scripts": {
    "build:wasm": "cd rust-wasm-lib && wasm-pack build --target web",
    "build": "npm run build:wasm && remix vite:build",
    "dev": "npm run build:wasm && remix vite:dev",
}

```

Test out the build of the wasm module by running

```bash
npm run build:wasm
```

## Integrating the wasm module into the remix app

All that is left now is to make the component that will host the wasm module.

Note that the build process will create a pkg folder with the wasm module and the js wrapper code. We only need to import the wrapper code into our project.

This is implemented in the game-of-life route: (https://github.com/DarkViolet-ai/remix-wasm/blob/main/app/routes/game-of-life.tsx)

```tsx
import initWasm, { Universe } from "../../rust-wasm-lib/pkg/rust_wasm_lib";
```

From there, we just call the functions from the wasm module. In this case, we pass a canvas element id to the wasm module and it will draw the game of life to the canvas. All of the graphics processing and game logic is handled in the wasm module.

In the repo, we also have an example of breakout game, also written entirely by AI (sonnet 3.5 and cursor IDE).

This project was one of my first attempts to use Cursor Composer to write the entire codebase according to my normal language specifications. I think having the ability to insert wasm modules into remix is a powerful tool that can enable a lot of new functionality in web apps. I used AI to teach me how to get this, done and it worked better than I expected. Now I have a new skill under my belt, and I hope you do too.

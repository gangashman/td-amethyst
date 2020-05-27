# td-amethyst

Primitive 2D Tower Defense game. It used maps from Tiled map editor.

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```

Screenshots from development
=======
<div align="center"><img src="https://github.com/gangashman/td-amethyst/blob/master/screenshots/Screenshot_20200527_123750.png"/></div>

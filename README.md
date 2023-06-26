## About

This is a [Zellij][zellij] plugin, for now it's just a first attempt a writing a plugin, and all it does is echo back whatever keys get sent to the plugin while it is focused.

However, once zellij exposes the ability to listen for global key events, it could evolve into something much more useful.

The main use case currently is to debug why certain keybindings might not be working, since it will print out the keys that zellij received. For example `ctrl+i` which gets interpreted as `tab`.

[zellij]: https://github.com/zellij-org/zellij

## Development

_Note_: you will need to have `wasm32-wasi` added to rust as a target to build the plugin. This can be done with `rustup target add wasm32-wasi`.

## Inside Zellij

![img-2023-06-14-143355](https://github.com/zellij-org/rust-plugin-example/assets/795598/d9e563dc-5d71-4e10-af5b-190365bdca3b)

You can load the `./plugin-dev-workspace.kdl` file as a Zellij layout to get a terminal development environment:

Either when starting Zellij:

```
zellij --layout ./plugin-dev-workspace.kdl
```

_Note that in this case there's a small bug where the plugin is opened twice, it can be remedied by closing the oldest instance or loading with the new-tab action as secified below - this will be addressed in the near future_

Or from a running Zellij session:

```bash
zellij action new-tab --layout ./plugin-dev-workspace.kdl
```

## Otherwise

1. Build the project: `cargo build`
2. Load it inside a running Zellij session: `zellij action start-or-reload-plugin file:target/wasm32-wasi/debug/zellij-echo.wasm`
3. Repeat on changes (perhaps with a `watchexec` or similar command to run on fs changes).

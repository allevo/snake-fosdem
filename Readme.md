# Snake

This project is a workspace with the following members:
- `snake`: contains all the logic
- `handly-made-snake`: wasm-targeted snake
- `bevy-snake`: bevy implementation

## Workspace members

### `snake`

This project is purely implemented using Rust without any graphic stuff

### `handly made snake`

This project is a graphic implementation of snake binding `snake` workspace member.

To run:
```
cd handmade-snake
wasm-pack build
cd www
npm run start
```

Open http://localhost:8080

For production release
```
cd handly-made-snake
wasm-pack build --release
cd www
npm run build
```
The `dist` folder contains all the static assets for a website

### `bevy-snake`

To run natively:
```
cargo run bevy_snake
```

For production release
```
cargo run --release bevy_snake
```

To run in browser:
```
cd bevy-snake
trunk build
```
Serve `dist` folder.

For production release
```
cd bevy-snake
trunk build --release
```

## License

See [LICENSE](LICENSE)





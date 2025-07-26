# My personal website

I've developed this website for two main reasons
1. The previous one, while graphically better, was not intuitive to use for people not used to write in terminal (even STEM peoples);
2. I hate js, I cannot stand working on it and it was not fun to implement new stuff.

This new website should be easier to use even if it's still not accessible for many users (e.g. mobile)

## How to build
Install [trunk] to build and serve the web application.

```sh
cargo install --locked trunk
```

Add compilation target `wasm32-unknown-unknown`:

```sh
rustup target add wasm32-unknown-unknown
```

Then serve it on your browser:

```sh
trunk serve
```

Now go to [http://localhost:8080](http://localhost:8080) to explore the website!


## Deploy

To build the WASM bundle, you can run the following command:

```sh
trunk build --release
```

Then you can serve the server from the `dist` directory.

[trunk]: https://trunkrs.dev
[Ratatui]: https://ratatui.rs

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=3c8cba&logoColor=white)](./LICENSE-MIT)

Licensed under  [The MIT License](./LICENSE-MIT).

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2025, [Daniele Giachetto](mailto:work@danielegiachetto.com)

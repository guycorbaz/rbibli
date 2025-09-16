# Development environment

To develop rbibli, certain tools need to be set up.

## Rust
We assume that rust is already installed.

## Trunk
Trunk is used in the development of the web interface. It serves as both a web server.
It also monitor files and recompile the various modules in the event of file changes.

To install trunk, use the following command:

```
cargo install --locked trunk
```

Trun requires a configuration file called *Trunk.toml*.
This file is included in the subdirectory *frontend*.

Trunk can also start tailwindcss in watch mode.

You can find documentation in the [trunk documentation](https://trunkrs.dev/guide/installation/)

## Leptos
Leptos is the fullsttack framework used to develop the web interface.
It needs some crates to be installed.

See more on the [leptos documentation](https://book.leptos.dev/)

## Tailwind
Tailwind is used to style the web interface.
The integration with Leptos and Trunk is a bit tricky.

We are using tailwind v4.

Tailwind requires a binary called *tailwindcss* to be installed. The binary can be downloaded
from the [tailwind website](https://tailwindcss.com/docs/installation)
and installed in a folder that is on your PATH.

You can find more on the
[tailwind documentation](https://tailwindcss.com/docs/guides/leptos)

To start tailwind in watch mode, use the following command:

'''
npx tailwindcss  -i ./input.css -o ./public/tailwind.css --watch
'''
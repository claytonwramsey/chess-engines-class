# Tomato: a pedagogical chess engine

Tomato is a chess engine developed by Clayton Ramsey as a resource for COLL 110, a class on chess
engines taught at Rice University.
It's a fork of Fiddler, Clayton's personal engine project.
Students of COLL 110 will be expected to fork Tomato and implement their own features as part of the
class.
It's written in Rust, with an emphasis on ergonomic usage and good performance.

## Features

- Full UCI support

- Multi-threaded search

- Phased move generation

- Principal variation search (with quiescence)

- Piece-square table evaluation

- Integrated gradient descent tuner

## Usage

Tomato uses nightly, unstable Rust.
As a result, you must use the nightly compiler to compile this code.
The most simple way of doing this is by running `rustup default nightly` before proceeding.

To create the main UCI executable, navigate to the root of this repository and run
`cargo build --release --bin tomato`.
This will then create the executable `target/release/tomato` (or `target/release/tomato.exe` for
Windows users).

You can also create a tuner executable.
To do so, run `cargo build --release --bin tune`.

Tomato uses features from relatively new versions of Rust, so you may need to update your
installation of Rust to compile it.
To do so, you can simply invoke `rustup upgrade`.

### Building with a specific target in mind

If you want to have a build which is fully optimized for your machine, you can set your machine as
the target architecture.
To do this, learn your target triple by running `rustc -vV` and reading the `host` line.
For an example of how to do this, here's the output on my machine:

```sh
$ rustc -vV
rustc 1.63.0 (4b91a6ea7 2022-08-08)
binary: rustc
commit-hash: 4b91a6ea7258a947e59c6522cd5898e7c0a6a88f
commit-date: 2022-08-08
host: x86_64-pc-windows-gnu
release: 1.63.0
LLVM version: 14.0.5
```

Once you have obtained the target triple (in my case, `x86_64-pc-windows-gnu`), you can then build
with a single target architecture.

```sh
cargo build --release --bin engine --target=<your target triple here>
```

This will then create a a new directory in the `target` folder named after your target triple
containing the target-optimized binary.
In my case, the path to the binary is `./target/x86_64-pc-windows-gnu/release/engine.exe`.

## UCI options supported

- `Hash`: Set the transposition table size, in megabytes.

## License

This code is licensed under the GNU GPLv3. For mor information, refer to [LICENSE.md](LICENSE.md).

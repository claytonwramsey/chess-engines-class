# Resources for learning the Rust programming language

A lot of students are having trouble picking up Rust, so I want to make sure that you all know where
you can find resources for working in Rust.

I've compiled a set of resources which I personally like here, and have tried to categorize them
pretty thoroughly.

## Tutorials and introductions

These resources are meant to be from-scratch tutorials on using the Rust programming language.
They're all typically intended to be read in order.

- [The Rust Book](https://doc.rust-lang.org/stable/book/).
  This book is an exhaustive introduction to the full Rust language.
  It gives (in my opinion) the best overview of working with the build system.

- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html).
  An introduction to Rust which is light on text and heavy on examples.
  It was my first introduction to the language.

- [Learn Rust with Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).
  A comprehensive and somewhat silly introduction to writing code in Rust by implementing a linked
  list, which is a common data structure in functional programming.
  This is my favorite introductory tutorial for Rust.

- [Rustlings](https://github.com/rust-lang/rustlings/).
  An interactive course as an intro to Rust.

## References and manuals

These resources are meant to be referenced, and generally not read cover-to-cover.

- [The Rust Standard Library](https://doc.rust-lang.org/std/index.html).
  Documentation for the Rust standard library.

- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).
  Documentation for using Cargo, Rust's package and build manager.

- [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/intro.html).
  The Rustonomicon is a reference for writing unsafe code in Rust.
  We won't be writing any unsafe code, but it's still pretty neat (and maybe useful if you care
  about really, really performant code).

## Advanced readings

These readings are not necessary for the class but might be interesting to you.

- [Rust Atomics and Locks](https://marabos.nl/atomics/).
  An introduction to creating concurrency primitives from scratch, using only atomic variables.

- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html).
  A brief overview of common idioms in Rust.

- [Rust for Rustaceans](https://rust-for-rustaceans.com/).
  This is a book you have to pay actual money for, which I don't like.
  However, it's a pretty good intro on building webservers in Rust.

- [One Hundred Thousand Lines](https://matklad.github.io/2021/09/05/Rust100k.html).
  One user's experience on maintaining large Rust projects.

## Clayton's IDE notes

There are lots of ways to get an IDE running for a Rust build system.
Basically all of them are fine, and you can find a tutorial for setting them up on the Internet.
I'll walk through my personal setup for writing Rust here to make it convenient for writing.

I write my code in [VSCodium](https://github.com/VSCodium/vscodium), a de-Microsofted fork of
[VS Code](https://code.visualstudio.com/).
There's nothing wrong with using VS Code - I simply believe that inconveniencing Microsoft is the
prerogative of every computer scientist everywehere.

Inside of VSCodium, I use the [rust-analyzer](https://rust-analyzer.github.io/) extension for
syntax highlighting, IDE hints, and more.
The default settings for rust-analyzer are pretty good, but I would also recommend turning on
"format on save" and changing the checker from "cargo check" to "cargo clippy."

When running code, I typically just use the command line (often in the integrated terminal).
When using VSCode(ium), it's important to be conscious of the current working directory in the
terminal session.
The commands I write most often are here:

```sh
cargo check # Run a very high-performance type-checking of the code. This will run very fast.
cargo clippy # Run a linter, which analyzes your code for common mistakes and style issues.
cargo test # Run all tests.
cargo build --release # Create a release build of your code in the directory target/release.
cargo run --release # Run your code in a fully-optimized binary.
```

I've also tried a lot of other IDEs and text editors for Rust, and here are my composed thoughts:

- **Sublime Text** is very nice, but the extensions are sometimes buggy and it doesn't have
  everything that I personally like.
  Kudos for how fast it is, though.

- **Atom** is an OK text editor, but it's slow and it's been killed by GitHub (i cri evri tim).
  Probably not worth your time.

- **vim** is a modal command-line editor used only by maniacs and 45-year-olds.
  If you are reading this you either should not be using vim or you are already reading this
  document in vim.

  If you think you're clever and use a different modal editor (nvim, hx, whatever) - you're not.
  This applies to you too.

- **Lapce** is a pretty nice development environment for Rust and not much else.
  I wish I could recommend it, but it's still in early alpha and is still extremely rough around the
  edges.

- **IntelliJ** and **CLion** are OK, but I haven't used them for Rust.
  The only experience I have with them that I can share is that they take forever to boot.

- **GNOME Builder** is surprisingly good.
  It's a little buggy but it holds a special place in my heart.

- **Kate** is also quite good.
  If you're a KDE nerd, it's a great fit.

- If you use **Notepad++** you really need to get back to filling out your spreadsheets and going to
  your meetings on the quarterly earnings reports.

## Tricks for using Git

I do all my Git work from the command line.
However, people new to Git may have some trouble with this.
I suggest using a GUI for working with Git - supposedly GitHub has a decent frontend for doing this.

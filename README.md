# Cedalion [![Build Status](https://travis-ci.org/nokaa/cedalion.svg?branch=master)](https://travis-ci.org/nokaa/cedalion)
Cedalion is a pastebin clone written in Rust using [rotor-http](https://github.com/tailhook/rotor-http), [chomp](https://github.com/m4rw3r/chomp), and [diesel](https://github.com/sgrif/diesel).

The webpages use [Erato](https://github.com/nokaa/erato), a minimalistic stylesheet.

<img src="cedalion.png"/>

### Install
NOTE: Building on ARM requires a patched version of mio v0.5.0, which you can get [on my git server](https://git.nokaa.moe/nokaa/mio_arm/src/v0.5.1).
NOTE: Cedalion currently will not build on ARM due to an issue with [nix](https://github.com/nix-rust/nix). The current version of nix compiles properly, but Mio v0.5.0 requires v0.4.2, which will not build. As there are breaking changes between nix v0.4.2 and v0.5.0, it is not possible to easily get around this issue. Until Mio updates to the newest version of nix, and rotor then updates to the newest version of Mio, using rotor on ARM does not seem possible.


Cedalion requires PostgreSQL with a database named `cedalion`. You can create this by running `createdb cedalion`.

Cedalion requires Rust Nightly. If you don't already have nightly, I recommend using [multirust](https://github.com/brson/multirust) to get it.

Next you need to install Diesel's cli tool: `cargo install diesel_cli`.

Run `diesel migration run` to create the table. Now we can `cargo build` to build the project, and `cargo run` to run the server.

Note: If you plan on using this, you should run `cargo build --release` and `cargo run --release`.

### TODO
- Add html viewer
  - This will require templates of some form. [Maud](https://github.com/lfairy/maud) looked interesting.
- Add ability to get raw file; this should trigger browser download
  - Should be accessed via html viewer

# Cedalion [![Build Status](https://travis-ci.org/nokaa/cedalion.svg?branch=master)](https://travis-ci.org/nokaa/cedalion)

Cedalion is a pastebin clone written in Rust using [rotor-http](https://github.com/tailhook/rotor-http). This branch uses redis as a database. 

The webpages use [Erato](https://github.com/nokaa/erato), a minimalistic stylesheet.

<img src="cedalion.png"/>

### Install

Cedalion requires redis to be installed on your system.

Cedalion requires the Rust toolchain. If you don't already have it, I recommend using [multirust](https://github.com/brson/multirust) to get it.

```
git clone https://github.com/nokaa/cedalion
cd cedalion
cargo build --release
./target/release/cedalion
```

### Building on ARM

mio v0.5.0, a dependency of rotor, does not currently build on ARM. I have
patched it so that it will build properly without breaking anything.
In order to use the patched version of mio:

```
git clone https://git.nokaa.moe/nokaa/mio_arm /my/path/mio
cd /my/path/mio
git checkout v0.5.1
cd /path/to/geomys
mkdir .cargo
echo 'paths = ["/my/path/mio",]' > .cargo/config
cargo build --release
sudo ./target/release/cedalion
```

### TODO

- Add html viewer
  - This will require templates of some form. [Maud](https://github.com/lfairy/maud) looked interesting.
- Add ability to get raw file; this should trigger browser download
  - Should be accessed via html viewer

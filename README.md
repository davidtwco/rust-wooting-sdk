# rust-wooting-sdk
This crate provides Rust bindings to the Analog and RGB SDKs provided by Wooting for the Wooting
One and Wooting Two.

See the [documentation](https://docs.rs/wooting-sdk) for more information on usage and
prerequisites.

## Known issues
This repository currently uses a fork of the Wooting SDKs that allow them to be built on Linux and
that add partial support for the Wooting Two (`direct_set_key` will not work for the numpad keys).

As soon as an updated version of the SDK is made available, this crate switch to use it.

## Prerequisites
In order to generate bindings for the SDKs using [bindgen][bindgen], `wooting-analog-sdk-sys` and
`wooting-rgb-sdk-sys` require `libclang`. If the submodules for the SDKs are not cloned, then the
build scripts will attempt to clone them, in this case, `git` is required. When building the SDKs,
`libusb` or `libudev` will be required for the SDKs' `hidapi` dependency on Linux, these are
expected to exist.

### Windows
Download and install the official pre-built binary for `libclang` from the
[LLVM download page][llvm].

### Mac
If using Homebrew, run:

```text
brew install llvm
```

If using MacPorts, run:

```text
port install clang-3.9
```

### Linux
If running Ubuntu 16.10 or greater, run:

```text
apt install llvm-3.9-dev libclang-3.9-dev clang-3.9 libusb-1.0.0-dev libusb-1.0.0
```

If running a version earlier than Ubuntu 16.10, then you may need to use the
[LLVM apt repositories][llvm_apt].

If using any other distribution, then you'll just need to figure it out yourself.

[llvm]: http://releases.llvm.org/download.html
[llvm_apt]: http://apt.llvm.org/
[bindgen]: https://github.com/rust-lang/rust-bindgen

## License
All of the crates contained in this repo are licensed under both MIT and Apache 2.0. Both the
[Wooting Analog SDK][analog] and [Wooting RGB SDK][rgb] are licensed under MPL.

[analog]: https://github.com/WootingKb/wooting-analog-sdk
[rgb]: https://github.com/WootingKb/wooting-rgb-sdk

## Code of Conduct
All interactions on this repository (whether on issues, PRs, or elsewhere) are governed by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

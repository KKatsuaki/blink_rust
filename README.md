# blink_rust
Blink LED code for rpi4 with Rust

## Environment
Host: macOS BigSur 11.2.3

Target: aarch64 (rapi4/raspi3)

## pre-build
If you have never tried to code embedded program in Rust, you should run below code to install cargo-binutils such as `rust-objcopy`, `rust-ld` and `rust-objdump`.

```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

**aarch64-unkwon-none** rust toolchain needs to build. 
run `rustup target add aarch64-unknown-none` to install the toolchain.

Moreover, to build, you have to **enable nightly**.

## Build
+ Run `make build` to build `kernel8.img`. After you build it, copy the img file to the SD_CARD.
+ Run `make build bsp=raspi3` to build `kernel8.img` for raspi3. (but I've not trid to run actually)
+ Run `make doc` to make a documet for this project
+ Run `make dump` for objdump

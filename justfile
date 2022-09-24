
info:
  echo "This file serves only to archive the commands and should not be used."
  echo "Use the normal cargo commands: cargo run|test|build"

run:
  cargo run

qemu: bootimage
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-cbos/debug/bootimage-cbos.bin

bootimage:
  cargo bootimage

build:
  cargo build --target x86_64-cbos.json

build_for_host:
  cargo rustc -- -C link-arg=-nostartfiles

build_for_thumbv7em:
  cargo build --target thumbv7em-none-eabihf

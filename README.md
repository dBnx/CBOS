# Own OS for x86-64

![GitLab](https://img.shields.io/gitlab/license/dBnx/os)
![GitLab](https://img.shields.io/badge/platform-x86--64-lightgrey)

<p align="center">
  <img src="https://gitlab.com/dBnx/os/uploads/81e45fd26fecedb63e80b4c63a52debc/os.png" alt="Output after boot using QEMU."/>
</p>
This project is a result from the awesome articles of [Philipp Oppermann](https://github.com/sponsors/phil-opp), and can be found at https://os.phil-opp.com/.
All the credits go to him. I just added a few things, a nicer interface and some other stuff.

## The current system uses

- Linked List allocator
- Single thread
- Kernel access to physical ram through a direct mapping in virtual space
- (Cooperative) Multitasking support through a async executor supporting async Rust
- A minimal shell to interact with it
- A glorious status bar, that shows the name of the OS and roughly the time since boot

## Future goals

- [ ] Custom slab and backup allocator
- [ ] Multithreading
- [ ] Userspace
- [ ] ACPI
- [ ] Filesystem support

## Tests and running

`QEMU` is required. Integration and unit tests can be run using the `cargo` infrastructure:

```sh
cargo test
```

The OS can then be experimented with, by running:

```sh
cargo run
```

It starts a local VM with minimal setup and shows something similar to the image at the top.

## Building

```sh
cargo build
```

Creates a bootable image.

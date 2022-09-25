# Own OS

<p align="center">
  <img src="https://gitlab.com/dBnx/os/uploads/5f5169a35acc744b71cfa82d9ca657f4/os.png" alt="Output after boot using QEMU."/>
</p>
This project is a result from the awesome articles of [Philipp Oppermann](https://github.com/sponsors/phil-opp), and can be found at https://os.phil-opp.com/.
All the credits go to him. I just added a few things, a nicer interface and some other stuff.

## The current system uses

- Linked List allocator
- Single thread
- (Cooperative) Multitasking support through a async executor supporting async Rust
- Kernel access to physical ram through a direct mapping in virtual space

## Future goals

- Custom slab and backup allocator
- Multithreading
- Userspace
- ACPI

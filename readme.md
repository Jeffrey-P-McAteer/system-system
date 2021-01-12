
# `system-system`

`system-system` is a bridge between the world of developers
and the world of end-users.
Between a CLI system and a GUI system.
Between an open debuggable inspectable system and a closed polished sellable system.


# How does it work?

1. Write a small PoC to perform some task using a lot of libraries that come from _somewhere_, in:
    * python
    * java
    * C/C++
2. Write 10 lines of Rust describing the libraries and files your PoC uses
3. `cargo build --release` and ship a single `.exe` file (`PE32+`, `ELF`, and `MACH-O` binaries output)

# How does it work (v2)?

1. You have something in production
2. It needs a new feature
3. Build a small PoC delivering that feature using a lot of libraries that come from _somewhere_, in:
    * python
    * java
    * C/C++
4. Write 10 **more** lines of Rust describing the libraries and files your PoC uses, and how it talks to the first PoC
5. `cargo build --release` and ship a single `.exe` file (`PE32+`, `ELF`, and `MACH-O` binaries output)





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


# No seriously, how does it work?

`system-system` is a Rust procedural macro. This means when you call `system!()`
it generates code based on the arguments passed and the files in your repository.

These files are archived into a tarball, which gets linked into the compiled binary as a static library.
When the compiled binary is run on a user's machine, the tarball is extracted to `$HOME/._system_APPNAME/`.

After extraction, your code will be run. Java classes will be run using an embedded JVM which is downloaded during compile time,
Python is run the same way. Things like JVM arguments and envrionment variables are specified using the `system!()` macro arguments.

`system-system` also provides helper logic for IPC mechanisms like `sqlite3` database files, so you can write a schema and ensure all
programs will see that database when they start.



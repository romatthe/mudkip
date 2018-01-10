:warning: Official Crappy Code:tm: inside! :warning: 

# Mudkip
:construction: **Under construction!** :construction:

A bare-bones NES emulator, writtin in Rust!

As this is the first non-trivial thing I'm trying to implement in Rust, don't expect any of the code here to be idiomatic. At the time of writing, most of it is actually rather of rather poor quality, so you're advised not to use this as a reference.

Currently, as it stands, my goal for this project is to reach a stage where Super Mario Bros. is playable. I do not aim for any of this to be accurate emulation, or fun to play.

### Currently implemented
- [x] ROM disassembling
- [ ] Super Mario Bros. emulation
- [ ] SDL2 rendering
- [ ] Popular mapper support
- [ ] Support for NES2.0 format
- [ ] Debugging facilities

Get Mudkip
-----------

If you're a Rust Programmer, you can install `mudkip` using Cargo:

```
$ cargo install --git https://github.com/romatthe/mudkip
```

Usage
-----

```
$ mudkip disassemble --help
USAGE:
    mudkip disassemble --file </path/to/file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file </path/to/file>    Path to the ROM you want to disassemble
```

Example
-------
Disassemble a target iNES ROM:

```
$ mudkip disassemble --file ~/roms/smb.nes
```

Please note: only ROMs with the iNES format are currently supported. I plan to add NES2.0 support later down the lane. Maybe.
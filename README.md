# penlist

<div align="center">

**Todo List console based application written in Rust.**

[![Crates.io](https://img.shields.io/crates/v/penlist)](https://crates.io/crates/penlist)
</div>
<p align="center">
    <img src="https://imgur.com/nhsVWdG.png"/>
</p>

### 📋 Requirements

`Rust`, `Cargo` installed in your machine and a `Nerd Font` to icons appears properly in your machine.

- `Rust`
- `Cargo`
- `Nerd Font`

**[Dependencies](#-project-dependencies)** are automatically installed.

### 🔧 Install

To install it just execute this:

```
$ cargo install penlist
```

## ⚙️ Running

To run the application just type this in your terminal:

```
$ penlist
```

```
────────────────────────────────────────────────────
                     🐧 PenList
────────────────────────────────────────────────────

           ☐ 001: Make dinner
           ☑ 002: Add /routes/findUser.ts
           ☐ 003: Fix issue n° 284

     ──────────────────────────────────────────
                        1/3
>
```

In the `>` prompt, you can access all commands and aliases with `help` command, an there are the commands:

- `help`: prints help message.
- `add <title>`: adds an item to the list.
- `remove <id>`: remove an item from the list.
- `toggle <id>`: toggle an item to checked and unchecked such as ☐ and ☑
- `quit`: quit from application.
- `save <filename>`: save your todo list into a file.
- `load <filename>`: load your todo list from a file.

- Aliases: `help`: (`h`); `add`: (`a`); `remove`: (`rm`); `toggle`: (`done`, `dn`); `quit`: (`q`);

## 🛠️ Project Dependencies

* [colored](https://crates.io/crates/colored) - Colorize the terminal, prettier
* [serde_json](https://github.com/serde-rs/https://crates.io/crates/serde_json) - To load and save JSON
* [serde](https://crates.io/crates/serde) - Serializing and deserializing
* [regex](https://github.com/rust-lang/regex) - ANSI escape codes
* [termsize](https://crates.io/crates/term_size) - Get terminal size

## ✔️ To Do

- [x] Aliases
- [x] Option to save todo list into a file.
- [x] Progess bar.
- [x] Content prettier and in the middle.

## ✒️ Author

* **Original Author** - [mi66mc](https://github.com/mi66mc)

## 📄 License

[MIT LICENSE](https://github.com/mi66mc/penlist/LICENSE)
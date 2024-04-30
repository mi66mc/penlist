# penlist

<div align="center">

**Todo List console based application written in Rust.**

[![Crates.io](https://img.shields.io/crates/v/penlist)](https://crates.io/crates/penlist)
</div>
<p align="center">
    <img src="https://imgur.com/tLV8dKj.png"/>
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
🐧 PenList
────────────────────────────
☐ 001: Type "help" for help
>
```

In the `>` prompt, you can access all commands and aliases with `help` command, an there are the commands:

- `help`: prints help message.
- `add <title>`: adds an item to the list.
- `remove <id>`: remove an item from the list.
- `toggle <id>`: toggle an item to checked and unchecked such as ☐ and ☑
- `quit`: quit from application.

- Aliases: `help`: (`h`); `add`: (`a`); `remove`: (`rm`); `toggle`: (`done`, `dn`); `quit`: (`q`);

## 🛠️ Project Dependencies

* [Colored](https://crates.io/crates/colored) - Rust crate

## To Do

- [x] Aliases
- [ ] Option to save todo list into a file.

## ✒️ Author

* **Original Author** - [mi66mc](https://github.com/mi66mc)

## 📄 License

[MIT LICENSE](https://github.com/mi66mc/penlist/LICENSE)
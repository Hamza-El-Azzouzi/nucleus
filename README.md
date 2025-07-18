# nucleus – A Minimalist Unix-Like Shell in Rust 🦀

**nucleus** is a standalone, minimalist Unix shell written entirely in Rust. Designed for embedded or low-level environments, it implements essential shell functionality without relying on external binaries or other shell programs like `bash`, `sh`, or `BusyBox`.

Inspired by projects like BusyBox and tailored for constrained systems, **nucleus** is built from the ground up using system calls and Rust's safe abstractions, offering a lightweight but capable shell experience.

---

## 🚀 Features

- Custom shell prompt: `$ `  
- Built-in commands implemented *from scratch*  
- Handles basic file system navigation and manipulation  
- No dependency on external shell utilities or binaries  
- Graceful exit on `Ctrl+D` (EOF)  
- Aligned with Unix conventions  
- Written in safe and idiomatic Rust  

---

## 🧩 Supported Commands

| Command    | Description                                  | Notes                        |
|------------|----------------------------------------------|------------------------------|
| `echo`     | Prints messages to standard output           | Supports escape sequences    |
| `cd`       | Changes the current working directory        |                              |
| `ls`       | Lists directory contents                     | Supports `-l`, `-a`, `-F`    |
| `pwd`      | Prints the current working directory         |                              |
| `cat`      | Displays file contents                       | Preserves Unicode & newlines |
| `cp`       | Copies files                                 |                              |
| `rm`       | Removes files or directories                 | Supports `-r` (recursive)    |
| `mv`       | Moves or renames files                       |                              |
| `mkdir`    | Creates new directories                      |                              |
| `exit`     | Exits the shell                              |                              |

If an unrecognized command is entered, the shell will print:
```
Command '<name>' not found
```

---

## 🛠 Building the Project

### Prerequisites

- [Rust (stable)](https://rust-lang.org/tools/install)
- A Unix-like operating system (Linux recommended)

### Build

```bash
git clone https://github.com/Hamza-El-Azzouzi/nucleus.git
cd nucleus
cargo build --release
```

### Run

```bash
cargo run
```

Or directly after building:

```bash
./target/release/nucleus
```

---

## 📦 Project Structure

```
0-shell/
├── src/
│   ├── main.rs              # Entry point: shell loop, prompt display
│   ├── lib.rs               # Common declarations (optional)
│   ├── parser.rs            # Parses user input into command + args
│   ├── executor.rs          # Handles execution routing based on parsed input
│   ├── builtins/
│   │   ├── mod.rs           # Declares subcommands module
│   │   ├── echo.rs          # echo implementation
│   │   ├── cd.rs            # cd implementation
│   │   ├── pwd.rs           # pwd implementation
│   │   ├── ls.rs            # ls implementation (+ -a -l -F)
│   │   ├── cat.rs           # cat implementation
│   │   ├── cp.rs            # cp implementation
│   │   ├── mv.rs            # mv implementation
│   │   ├── rm.rs            # rm implementation (+ -r)
│   │   ├── mkdir.rs         # mkdir implementation
│   │   └── exit.rs          # exit implementation
│   ├── utils.rs             # Helper functions (e.g., flag parsing, formatting)
│   └── errors.rs            # Custom error types and handling utilities
├── tests/                   # Integration tests
│   ├── shell_loop.rs        # Test shell interaction
│   └── commands.rs          # Test built-in commands
├── Cargo.toml               # Project metadata and dependencies
├── Cargo.lock               # Dependency lock file
└── README.md                # Project overview and usage
```

---

## 📚 Learning Objectives

This project aims to teach system-level concepts using Rust:

- Process creation and system calls
- Low-level file and directory handling
- Manual command parsing and argument handling
- Graceful input/output interaction in CLI
- Writing clean, safe, and modular Rust code

---

## 💡 Inspiration

- [BusyBox](https://busybox.net/)
- The Linux Programming Interface
- Rust's zero-cost abstractions and focus on safety

---

## ⚠️ Limitations

- Only basic shell syntax supported (no piping `|`, redirection `>`, or globbing `*`)
- No environment variable support (e.g., `$HOME`)
- Only works in Unix-like systems (Linux, macOS)

---

## 📄 License

MIT License © 2025 Hamza El Azzouzi

---

## 🧠 Future Work

- Add support for command chaining and pipelines
- Implement environment variable expansion
- Add unit tests and integration tests for all commands
- Create cross-compilation profile for embedded targets

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📫 Contact

For questions or suggestions, feel free to open an issue on GitHub.

---

*Built with ❤️ and Rust 🦀*
# 0-shell
minimalist Unix-like shell implemented in Rust, designed to run core Unix commands using system calls—without relying on external binaries or built-in shells like bash or sh


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
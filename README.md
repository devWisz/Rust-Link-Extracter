# Rust-Link-Extracter

Rust-Link-Extracter is a simple and lightweight command-line tool built with Rust that fetches a webpage and extracts all hyperlinks from it.
---

## Features

- Extracts all links from a webpage
- Fast and lightweight
- Async HTTP requests
- Clean terminal output
- Beginner-friendly Rust project


## Prerequisites

Before running this project, make sure you have Rust and Cargo installed on your system.

Check installation:

```bash
rustc --version
cargo --version
```

If Rust is not installed, download it from:

https://www.rust-lang.org/tools/install

---

## Clone the Repository

```bash
git clone https://github.com/devWisZ/Rust-Link-Extracter.git
```

Move into the project directory:

```bash
cd Rust-Link-Extracter
```

---

## Add Dependencies

Add the following dependencies inside your `Cargo.toml` file:

```toml
[dependencies]
anyhow = "1"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
select = "0.6"
```

---

## Run the Project

Run the project in development mode:

```bash
cargo run
```

Build an optimized release version:

```bash
cargo build --release
```

The executable binary will be generated inside:

```bash
target/release/
```

---

## Example Output

```bash
HTML length: 18564

/
https://play.rust-lang.org/
https://blog.rust-lang.org/
https://doc.rust-lang.org/
```



## Open Source

This project is open-source and free to use for educational and personal purposes.


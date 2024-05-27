<div align="center">
  <img src="https://github.com/AvaterClasher/extron/assets/116944847/de36059a-7eca-4287-a05a-ce759e0bd1ca" alt="Extron Logo"/>

[Website][Extron] | [Getting started] | [Documentation]
</div>

This is the main source code repository for [Extron]. It contains the compiler,
standard library, and documentation.

[Extron]: https://extron.vercel.app
[Getting Started]: https://extron.vercel.app/docs#getting-started
[Documentation]: https://extron.vercel.app/docs

## Why Extron ?

- Mainly because I wanted to learn Rust. And secondly because I wanted to make a simple and easy to use programming language. Thats it. 😁

## Quick Start

### Installing on Linux or WSL (Windows Subsystem for Linux)

```bash
curl -sfL https://raw.githubusercontent.com/AvaterClasher/extron/main/install.sh | sh
```

### Installing on Windows

```powershell
iwr -useb https://raw.githubusercontent.com/AvaterClasher/extron/main/install.ps1 | iex
```

## Installing from Source

### Build steps

1. Clone the [source] with `git`:

   ```sh
   git clone https://github.com/AvaterClasher/extron.git
   cd extron
   ```

[source]: https://github.com/AvaterClasher/extron

2. Run the following command to build the project:

   ```bash
   # For running the REPL (Read-Eval-Print-Loop)
   cargo run
   ```

   ```bash
   # For running a Extron (.ext) file
    cargo run run <path-to-file>
   ```

## Getting Help

See https://github.com/AvaterClasher/extron/issues for reporting bugs and feature requests.

## License

Extron is primarily distributed under the terms of the MIT license.

See [LICENSE-MIT](LICENSE.md) for details.
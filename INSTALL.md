# Installation Guide

## Installing in Linux

Just run the [install script](install.sh) in your terminal:

```sh
sh install.sh
```

## Installing on Windows

On Windows, to install extron run the following in a terminal:
Download the latest version of Extron from [here](https://github.com/AvaterClasher/extron/releases/download/latest/extron.exe).

Then edit your system's `PATH` variable and add: `{your-path-here}\extron.exe`.
See [this guide on editing the system `PATH`](https://www.java.com/en/download/help/path.html)
from the Java documentation.

## Building Locally

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
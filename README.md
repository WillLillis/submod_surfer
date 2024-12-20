<div align="center">
  <a href="https://github.com/WillLillis/"><img src="assets/submod_surfer.png"        width="500px" alt="Submod Surfer logo"/></a>
</div>

#

Surf through the submodules of your project with ease!

# Installation

1. Build the `submod_surfer` crate (`cargo build --release`).
2. `source env.sh`

# Usage

The `submod_surfer` binary isn't intended to be invoked directly. Rather, it only
handles the user's fuzzy finding over the submodules of their git project. Once a
choice has been made, the program outputs the path to said submodule and exits.
This path is then used by the shell function defined in `env.sh` script to change
into the desired directory. In order to use this tool, please *source* the `env.sh`
script.

```
$ submod_surfer --help
Usage: submod_surfer [OPTIONS]

Options:
  -m, --module-path <MODULE_PATH>  Path to the .gitmodules file of interest
  -f, --fmt <FMT>                  Default: "%n", Name: %n or %N, Path: %p, Url: %u, Branch: %b
  -h, --help                       Print help

$ source /path/to/env.sh
```

That's it! It's common to utilize the `module-path` argument alongside a shell alias
so you can surf even while outside of your project's root directory. For example,

```shell
alias surf="submod_surfer --module-path /absolute/path/to/project --fmt %p"
```

## TODO
- The tool does not work inside git bash's terminal emulator. This needs to be investigated further
- More cleanup with shell integration (Shells besides zsh and Bash)
- Add more tests
- Address user feedback


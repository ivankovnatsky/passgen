# passgen

A command-line tool for generating secure passwords in an Apple-like format.

## Development

### Prerequisites

- [direnv](https://direnv.net/)
- [Nix](https://nixos.org/download.html) with flakes enabled

### Setup

Allow direnv to automatically load the development environment:

```console
direnv allow
```

### Build

```console
make build
```

### Usage

Generate a new password:

```console
make run
```

## Format

Passwords are generated in the format: `xxXxxx-xxxxXx-XxxXxx`.

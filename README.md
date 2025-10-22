# Simple server password CLI

`pget` is a tiny Rust command-line tool to get and manage servers passwords.

* Query a server password: `pget server_name # it will copy to your clipboard`
* Add or update a server password: `pget -a server_name some_password`

---

## Installation

- Download from [releases](https://github.com/rado31/pget/releases)

Or build it manually

```bash
# Build and install with cargo
cargo install --path .
# or build locally
cargo build --release
# binary will be in target/release/pget
```

## Usage

```text
Usage:
  pget <NAME>                   # print password for NAME
  pget -a <NAME> <PASSWORD>     # add or update NAME with PASSWORD
  pget --help                   # show help
  pget --version                # show version
```

Examples:

```bash
# copy the password to your clipboard for server_1
pget server_1

# add or update server_2's password
pget -a server_2 s3cr3tP@ssw0rd

# show help
pget --help
```

---

## Storage

> When you will run for first time, it will create a file: `~/.ssh/passwords.toml`.
> So you don't have to add a new server's password by cli. You can also add it manually in the file.

Example file layout (TOML):

```toml
[servers]
server_1 = "s3cr3tP@ssw0rd",
db-primary = "dbP@ss!"
```

---

## Contributing

1. Fork the repo
2. Create a topic branch: `git switch -c feat/your-feature`
3. Make your changes
4. Open a PR and describe the change

Keep commits clean for logic where appropriate.

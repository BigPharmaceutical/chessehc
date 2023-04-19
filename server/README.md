# Chessehc Server

## Installing

First, install the Cargo toolchain: https://rustup.rs/

To install the server, run
```sh
cargo install --git https://github.com/BigPharmaceutical/chessehc --features server
```

<details>

<summary>Troubleshooting</summary>

```
error: linker `cc` not found
```

You need to install a package like `gcc` (most) / [`build-base`](https://pkgs.alpinelinux.org/packages?name=build-base) (Alpine) / [`build-essential`](https://packages.ubuntu.com/search?keywords=build-essential&searchon=names) (Ubuntu) / [`base-devel`](https://archlinux.org/groups/x86_64/base-devel/) (Arch)

```
error: failed to run custom build command for `openssl-sys v0.9.85`
```

Look at the sections starting with `--- stderr`, they will help fix this issue.

Most likely, you will need to install [`pkgconf`](https://pkgs.alpinelinux.org/packages?name=pkgconf) (Alpine) / [`pkg-config`](https://packages.ubuntu.com/search?keywords=pkg-config&searchon=names) (Ubuntu, Arch) and [`openssl-dev`](https://pkgs.alpinelinux.org/packages?name=openssl-dev) (Alpine) / [`libssl-dev`](https://packages.ubuntu.com/search?keywords=libssl-dev&searchon=names) (Ubuntu) / [`openssl`](https://archlinux.org/packages/core/x86_64/openssl/) (Arch).

</details>

Install PostgreSQL: https://www.postgresql.org/download/

## Setup

Copy the [`.env.template`](./.env.template) file and rename to `.env`. Replace the database username and password, and the address if needed.

Create a `chessehc` database in PostGres.  
Run the SQL commands in [setup.sql](../database/setup.sql) in the database.

## Running the Server

If you have a `.env` file, make sure you are in the same directory as it.

If you installed it with `cargo install`, run `chessehc_server`, if not, the binary will be in the `server/target/release` directory.

## Running Cargo Commands

### Commands

Run clippy on all the code:
```bash
cargo clippy_all
```

Run all tests:
```bash
cargo test_all
```

Update `sqlx-data.json`:
```bash
cargo db_data_update
```

Run the server:
```bash
cargo run --features server
```

Build the server in release mode:
```bash
cargo build --features server --release
```

### Without a Database Connection

If you do not have a database set up or the database is not accessible, set the `SQLX_OFFLINE` environment variable to `true`.

<details>

<summary>Setting the environment variable</summary>

#### Unix-like (Linux / MacOS / etc.)

```sh
SQLX_OFFLINE=true <command>
```  
or  
```sh
export SQLX_OFFLINE=true
<command>
```

#### Windows (PowerShell)

```ps
$env:SQLX_OFFLINE = "true"
```

</details>


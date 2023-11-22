# auth-api
Hicampi Auth is designed to work with Postgres. There are two parts to every Auth system:

## Development

### Local Development

#### 1. Coding Structure

```sh
auth-api/
├── config        # Server Configuration Files
├── crates        # Auth API crates
├── docs          # Document Files
├── migrations    # Diesel.rs Migration Files
├── scripts       # Scripts
├── tests         # Testing Cases
└── src           # Source Codes
```

#### 2. Prepare Development Environment
We use following technologies for Hicampi Auth.

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs/) 
- [Diesel](http://diesel.rs/)
- [PostgreSQL](https://www.postgresql.org)

**On MacOS:**

If you use macOS to develop Hicampi Auth,

**1. Install Rust Language**: A language empowering everyone to build reliable and efficient software. [Installation Guide Here.](https://doc.rust-lang.org/book/ch01-01-installation.html)

**2. Install Homebrew**: The Missing Package Manager for macOS. [Installation Guide Here.](https://brew.sh)

**3. Install PostgreSQL**: PostgreSQL is a powerful, open source object-relational database system with over 30 years of active development that has earned it a strong reputation for reliability, feature robustness, and performance. [Installation Guide Here.](https://www.postgresql.org/download/)

You can also install PostgreSQL using Homebrew:

```
  brew install postgresql
  brew services start postgresql
  /usr/local/opt/postgres/bin/createuser -s postgres
```

**4. Set Up User and Database for Hicampi Auth**:
You must prepare database for Hicampi Auth to use following commands.

```
  # Either execute db-init.sh, or manually initialize the postgres database:
  sudo -u postgres psql -c "create user auth with password 'password' superuser;"
  sudo -u postgres psql -c "create database auth with owner auth;"
  export AUTH_DATABASE_URL=postgres://auth:password@localhost:5432/auth
```

#### 3. Get Source Code

```
git clone git@github.com:hicampi/auth-api.git
```
If you get `Permission denied` error using `ssh` refer [here](https://doc.gitlab.com/) or use `https` link as a fallback.

```
git clone https://github.com/hicampi/auth-api.git
```

#### 4. Compilation & Run

```
    cd auth-api
    cargo run
```

If Hicampi Auth started successfully, you can check `http://localhost:8536/auth/v1/health` endpoint to check availability of Hicampi Auth API Server.

You also using `cargo check` command to check compilation error of Hicampi Auth. It's more quicker than `cargo run` command.

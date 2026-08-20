# Rust Web Application

A web application built with Rust that provides user asset management with REST API and web interface.

The application was created using feature approach, where model, controllers, services and repositories are grouped by domain or as its name says by feature (ex. User, Asset, etc..). This project has a learning process only.

## Features

- **REST API** to query user, assets, and authenticate users.
- **Web Interface** for login and asset visualization
- **HTTP Server** running on port 3000
- **User Management** and associated assets

## Requirements

- Rust 1.96.0  or higher
- Cargo (included with Rust)
- make (Windows)

## Installation

Clone the repository:

```bash
git clone https://github.com/unosolo/wallet-live.git
cd wallet-live
```

## Usage

### Initial requirements
- A `Postgres` database must be provided before running the app.
- Apply the migration on the database using sqlx. Install sqlx first. Then run `cargo sqlx migrate run` once.
- The commands in the `makefile` facilitate the creation and running of Postgres container.
- Containers are created using `wslc` containers, so the commands must be run in Windows terminal.
  - Make commands useful:
    - `make setup` (run once)
    - `make run` (run once)
    - `make restart` (to stop and start the containers)

Start the application with:

```bash
cargo run
```

The application will be available at `http://localhost:3000`

## Available Endpoints

### REST API

- **GET** `/api/assets` - Retrieves  a specific user info.
- **POST** `/api/assets` - Add a user
- **PATCH** `/api/assets` - Updates a specific user info.

- **GET** `/api/user/{user_id}/owned_asset` - Retrieves the assets owned by a specific user
- **POST** `/api/user/{user_id}/owned_asset` - Add the assets bought by a specific user
- **PATCH** `/api/user/{user_id}/owned_asset/{asset_id}` - Update the assets owned by a specific user

### Web

Available web pages:

- **GET** `/login` - Authentication page. if user does not exist, it register them.
- **GET** `/` - Home page - Works like the Login page.
- **GET** `/assets` - Asset visualization page

## Project Structure

```
.
├── src/
│   ├── main.rs
│   ├── feature/
│   ├── global/
│   ├── page/
│   └── routes/
├── Cargo.toml
└── README.md
```

## Main Dependencies

Dependencies are listed in `Cargo.toml`. To view all dependencies used:

```bash
cargo tree
```

## Development

To build in debug mode:

```bash
cargo build
```

To build in release mode (optimized):

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

## License
Apache License

## Author

Julio Herrera - Wallet Live (Rust)

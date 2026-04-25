# Yuhuu Function

A Rust workspace project with Actix Web framework, organized into multiple crates for better modularity and code
organization.

## Project Structure

```
yuhuu-function/
├── Cargo.toml              # Workspace configuration
├── main-app/               # Main application entry point
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # Actix web server
└── crates/                 # Shared crates
    ├── models/             # Data models and types
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── logic/              # Business logic
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── functions/          # Utility functions
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── http-response/      # HTTP response utilities
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── database/           # Database configuration
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── config-env/         # Environment configuration
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── graphql/            # GraphQL schema and handlers
        ├── Cargo.toml
        └── src/lib.rs
```

## Crates Overview

- **main-app**: The main application that runs the Actix web server
- **models**: Data structures, entities, DTOs, and type definitions
- **logic**: Business logic, services, and core application functionality
- **functions**: Utility functions and helper methods
- **http-response**: HTTP response formatting and standardization
- **database**: Database connection and configuration
- **config-env**: Environment variable configuration service
- **graphql**: GraphQL schema, queries, mutations, and handlers

## Getting Started

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo
- PostgreSQL (or your preferred database)

### Configuration

1. Copy the example environment file:

```bash
cp .env.example .env
```

2. Update the `.env` file with your configuration:

```env
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgres://user:password@localhost/yuhuu_db
SQLX_LOG=false
```

### Building the Project

```bash
# Build all crates in the workspace
cargo build

# Build in release mode
cargo build --release
```

### Running the Application

```bash
# Run the main application
cd main-app
cargo run

# Or from the workspace root
cargo run -p main-app
```

The server will start on `http://127.0.0.1:8080`

### Available Endpoints

- `GET /graphql` - GraphQL Playground (development)
- `POST /graphql` - GraphQL endpoint
- `POST /strapi-proxy` - Direct Strapi proxy endpoint

## Development

### Adding Dependencies

Dependencies are managed at the workspace level in the root `Cargo.toml`. To add a new dependency:

1. Add it to `[workspace.dependencies]` in the root `Cargo.toml`
2. Reference it in individual crate `Cargo.toml` files using `{ workspace = true }`

### Testing

```bash
# Run all tests in the workspace
cargo test

# Run tests for a specific crate
cargo test -p models
```

### Code Structure

Each crate has its own `lib.rs` file that serves as the entry point. You can add modules and functionality as needed:

- **models**: Add your data structures, enums, and type definitions
- **logic**: Implement business logic, services, and core functionality
- **functions**: Add utility functions and helpers
- **http-response**: Create standardized HTTP response types and utilities
- **database**: Configure database connections and migrations
- **config-env**: Manage environment variables and configuration
- **graphql**: Define GraphQL schema, queries, and mutations

## GraphQL API

The project includes a GraphQL API with:

- **Health Check**: Query to verify the server is running
- **Strapi Proxy**: Execute queries directly against a Strapi backend
- **Playground**: Interactive GraphQL IDE for development

Example query:

```graphql
query {
  health
}
```

## Docker Support

Build and run with Docker:

```bash
# Build the Docker image
docker build -t yuhuu-function .

# Run the container
docker run -p 8080:8080 --env-file .env yuhuu-function
```

## Technologies Used

- **Actix Web**: Fast and powerful web framework
- **Tokio**: Async runtime
- **Serde**: Serialization/deserialization
- **UUID**: Unique identifier generation
- **Chrono**: Date and time handling
- **Tracing**: Structured logging
- **Anyhow/Thiserror**: Error handling
- **SeaORM**: Database ORM
- **async-graphql**: GraphQL server implementation

## License

This project is licensed under the MIT License.

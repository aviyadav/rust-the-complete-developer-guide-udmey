# rust-web-service

A production-style REST API built with [Actix-Web 4](https://actix.rs/), SQLite via [SQLx](https://github.com/launchbadge/sqlx), and Tokio async runtime. Implements full CRUD for a **Tasks** resource, custom request-timing middleware, structured logging, and a clean layered architecture.

---

## Table of Contents

- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [API Reference](#api-reference)
- [Prerequisites](#prerequisites)
- [Configuration](#configuration)
- [Build](#build)
- [Run](#run)
- [Testing](#testing)
- [Dependencies](#dependencies)

---

## Project Structure

```
rust-web-service/
├── Cargo.toml
├── .env                       # Environment variables (see Configuration)
└── src/
    ├── main.rs                # Entry point: server setup, DB pool init, workers
    ├── app.rs                 # Route registration (configure_routes)
    ├── db/
    │   └── mod.rs             # Connection pool factory + schema migration
    ├── models/
    │   ├── mod.rs
    │   └── task.rs            # Task struct, CreateTaskRequest, UpdateTaskRequest
    ├── services/
    │   ├── mod.rs
    │   └── task_service.rs    # CRUD business logic (find_all, find_by_id, create, update, delete)
    ├── handlers/
    │   ├── mod.rs
    │   ├── health.rs          # GET /health
    │   └── tasks.rs           # GET/POST/PUT/DELETE /api/tasks
    └── middleware/
        ├── mod.rs
        └── request_timer.rs   # Logs method + path + status + elapsed ms per request
```

---

## Architecture

The service follows a strict three-layer design:

```
HTTP Request
     │
     ▼
┌─────────────┐
│  Middleware  │  RequestTimer (elapsed logging) + Actix Logger (access log)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Handlers  │  Deserialise request, validate input, return HTTP responses
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Services  │  Business logic: build queries, apply defaults, map results
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Database   │  SQLx connection pool → SQLite (swappable for Postgres)
└─────────────┘
```

**Key design decisions:**

- `web::Data<DbPool>` is registered once and cloned into every worker thread — zero lock contention on the pool handle.
- Worker count is configurable via `WORKERS` env var; defaults to 4.
- Schema is created at startup with `CREATE TABLE IF NOT EXISTS` — no external migration tool needed for development.
- All database errors surface as typed `sqlx::Error`, logged at `error` level, and returned as `500 Internal Server Error` JSON.

---

## API Reference

### Health

| Method | Path      | Response                         |
|--------|-----------|----------------------------------|
| `GET`  | `/health` | `200 OK` — `{ status, service, version }` |

### Tasks

| Method   | Path              | Body                        | Success         |
|----------|-------------------|-----------------------------|-----------------|
| `GET`    | `/api/tasks`      | —                           | `200` array     |
| `POST`   | `/api/tasks`      | `CreateTaskRequest`         | `201` task      |
| `GET`    | `/api/tasks/{id}` | —                           | `200` task      |
| `PUT`    | `/api/tasks/{id}` | `UpdateTaskRequest`         | `200` task      |
| `DELETE` | `/api/tasks/{id}` | —                           | `204 No Content`|

#### Task object

```json
{
  "id":          "550e8400-e29b-41d4-a716-446655440000",
  "title":       "Write README",
  "description": "Document the project",
  "completed":   false,
  "created_at":  "2026-04-06T12:00:00+00:00",
  "updated_at":  "2026-04-06T12:00:00+00:00"
}
```

#### CreateTaskRequest

```json
{ "title": "string (required)", "description": "string (optional)" }
```

#### UpdateTaskRequest (all fields optional)

```json
{ "title": "string", "description": "string", "completed": true }
```

#### Error responses

```json
{ "error": "Task not found" }         // 404
{ "error": "title must not be empty" } // 400
{ "error": "Failed to fetch tasks" }  // 500
```

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust + Cargo | stable (≥ 1.75) | `curl https://sh.rustup.rs -sSf \| sh` |
| SQLite dev libs | any | `sudo apt install libsqlite3-dev` (Debian/Ubuntu) |

> SQLite itself is bundled by `libsqlite3-sys` via the `sqlx` crate — no separate SQLite binary is required at runtime.

---

## Configuration

Copy `.env` and adjust as needed:

```ini
# SQLite database path (created automatically on first run)
DATABASE_URL=sqlite:tasks.db

# Server bind address
HOST=127.0.0.1
PORT=9090

# Number of actix-web worker threads
WORKERS=4

# Log level: error | warn | info | debug | trace
RUST_LOG=info
```

Environment variables override `.env` values if set in the shell.

---

## Build

### Development build

```bash
cargo build
```

### Release (optimised) build

```bash
cargo build --release
# binary → target/release/rust-web-service
```

### Check for errors without producing a binary

```bash
cargo check
```

---

## Run

```bash
# Development (uses .env automatically)
cargo run

# Or run the release binary directly
./target/release/rust-web-service
```

On startup you should see:

```
[INFO] Starting rust-web-service on http://127.0.0.1:9090
[INFO] Workers: 4
```

### Quick smoke test with curl

```bash
# Health check
curl http://127.0.0.1:9090/health

# Create a task
curl -s -X POST http://127.0.0.1:9090/api/tasks \
  -H 'Content-Type: application/json' \
  -d '{"title":"Buy milk","description":"Semi-skimmed"}' | jq

# List all tasks
curl http://127.0.0.1:9090/api/tasks | jq

# Get one task (replace <id>)
curl http://127.0.0.1:9090/api/tasks/<id> | jq

# Mark completed
curl -s -X PUT http://127.0.0.1:9090/api/tasks/<id> \
  -H 'Content-Type: application/json' \
  -d '{"completed":true}' | jq

# Delete
curl -X DELETE http://127.0.0.1:9090/api/tasks/<id> -v
```

---

## Testing

### Unit / integration tests

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test by name
cargo test <test_name>
```

### Linting and formatting

```bash
# Check formatting
cargo fmt --check

# Auto-format
cargo fmt

# Run Clippy linter
cargo clippy -- -D warnings
```

### Check for unused dependencies

```bash
cargo +nightly udeps   # requires: cargo install cargo-udeps
```

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `actix-web 4` | Async HTTP framework (actor model, multi-worker) |
| `sqlx 0.7` | Async SQL with compile-time checked queries; SQLite driver |
| `tokio 1` | Async runtime (`#[actix_web::main]` wraps it) |
| `serde / serde_json` | JSON serialisation / deserialisation |
| `uuid 1` | Generate v4 UUIDs for task IDs |
| `chrono 0.4` | RFC 3339 timestamps for `created_at` / `updated_at` |
| `dotenvy 0.15` | Load environment variables from `.env` |
| `env_logger / log` | Structured levelled logging |
| `thiserror 1` | Ergonomic custom error types |
| `futures-util 0.3` | `LocalBoxFuture` used in custom middleware |

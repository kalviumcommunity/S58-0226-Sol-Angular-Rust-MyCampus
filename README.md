# Concept 2: Building Type-Safe, Fast APIs Using Rust

In this concept, a Rust backend API was implemented using Axum. The goal was to understand how routes, handlers, typed structs, and async workflows work together to process HTTP requests and return JSON responses.

## Implemented Endpoints

### GET /api/health

Returns a simple JSON response to verify the server is running.

### POST /api/products

Accepts JSON input with:

- name (String)
- quantity (i32)

The input is deserialized into a typed struct (`CreateProductRequest`), ensuring type safety. The handler processes the request and returns a structured JSON response (`CreateProductResponse`).

## Request–Response Flow

1. Angular component triggers an action (e.g., form submission).
2. Angular service sends HTTP POST using HttpClient.
3. Rust route matches the endpoint.
4. Rust handler validates JSON into a typed struct.
5. (Future step) SQLx will insert data into PostgreSQL.
6. Rust returns JSON response.
7. Angular updates the UI based on response.

## Why Type Safety Matters

Rust enforces strict typing using structs and compile-time checks. This prevents invalid JSON structures from entering business logic and reduces runtime errors. Combined with TypeScript in Angular, this creates a predictable and reliable full-stack system.

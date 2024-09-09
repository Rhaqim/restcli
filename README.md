# Rest CLI endpoint Generator

This is a simple CLI tool to generate REST API endpoints for a given resource. It is written in Rust and uses the [clap](https://docs.rs/clap/4.5.17/clap/) library for parsing command line arguments.

## Features

- Generate REST API endpoints for Python, Rust, Golang and JavaScript files.
- Generate endpoints for a given resource.
- Generate endpoints for a given resource with a given base path.

## Installation

To install the CLI tool, you can use the following command:

```bash
cargo install --path .
```

## Usage

To generate REST API endpoints for a given resource, you can use the following command:

```bash
rest-cli-generator --resource <resource_name>
```

To generate REST API endpoints for a given resource with a given base path, you can use the following command:

```bash
rest-cli-generator --resource <resource_name> --base-path <base_path>
```

## Example

To generate REST API endpoints for a resource named `user`, you can use the following command:

```bash
rest-cli-generator --resource user
```

This will generate the following output:

```bash
GET /users
GET /users/:id
POST /users
PUT /users/:id
DELETE /users/:id
```

To generate REST API endpoints for a resource named `user` with a base path of `/api`, you can use the following command:

```bash
rest-cli-generator --resource user --base-path /api
```

This will generate the following output:

```bash
GET /api/users
GET /api/users/:id
POST /api/users
PUT /api/users/:id
DELETE /api/users/:id
```

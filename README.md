# Rest CLI endpoint Generator

This is a simple CLI tool to generate REST API endpoints from defined routes in Python, Rust, Golang and JavaScript files.

## Features

- Generate REST API endpoints for Postman collection, VSCode REST Client, and curl (HTTPie).
- Generate REST API endpoints for Python, Rust, Golang, and JavaScript.

## Installation

To install the CLI tool, you can use the following command:

```bash
cargo install --path .
```

## Flags

- `--help` - Display help information.
- `-p`, `--postman` - Generate Postman collection.
- `r`, `--rest-client` - Generate VSCode REST Client.
- `-c`, `--curl` - Generate curl commands.
- `url` - The base URL for the REST API. Default is `http://localhost`.
- `port` - The port number for the REST API. Default is `8080`.
- `a`, `--append` - Append the generated REST API endpoints to an existing file.
- `-o`, `--output` - The output file for the generated REST API endpoints. Default is `requests`.

## Supported Languages and Frameworks

- Python (Flask)
- Rust (Actix Web)
- Golang (Gin)
- JavaScript (Express)

## Example

Here is an example of a Python file with defined routes:

```python
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/api/v1/hello', methods=['GET'])
def hello():
    return jsonify({'message': 'Hello, World!'})

@app.route('/api/v1/greet', methods=['POST'])
def greet():
    data = request.get_json()
    name = data.get('name')
    return jsonify({'message': f'Hello, {name}!'})

if __name__ == '__main__':
    app.run(port=8080)
```

You can generate REST API endpoints for this Python file using the following command:

- Single File

```bash
rest-cli -p -r -c --url http://localhost --port 8080 app.py
```

- Multiple Files

```bash
rest-cli -p -r -c --url http://localhost --port 8080 app1.py app2.py app3.py
```

This will generate the following files:

- `requests.postman_collection.json` - Postman collection.
- `requests.http` - VSCode REST Client.
- `requests.sh` - curl commands.

You can import the Postman collection into Postman, open the VSCode REST Client file in VSCode, and run the curl commands in your terminal.

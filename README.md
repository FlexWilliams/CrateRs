# CrateRs

[An attempt at] A [remoteStorage](https://remotestorage.io/) web server written in rust.

(TODO: tag frameworks used).

## Project Management

[Project Board](./project_mgmt/BOARD.md)

## Development

### Building

```
cargo clean && cargo build
```

### Running

```
cargo run
```

The server will be listening on: `http://127.0.0.1:8000`

Make a request via curl to test:
```
curl -XGET 'http://127.0.0.1:8000'
```

HTTP Repsonse Body: `Hello, from CrateRs!`

<br>

### API Requests:

#### HEAD a folder/document
```
curl -I 'http://127.0.0.1:8000/storage/flexwilliams/path/to/my/folder'
```

#### GET a folder/document
```
curl -XGET 'http://127.0.0.1:8000/storage/alexwilliams/path/to/my/folder'
```

#### PUT a document
```
curl -XPUT 'http://127.0.0.1:8000/storage/alexwilliams/path/to/my/folder'
```

#### DELETE a document
```
curl -XDELETE 'http://127.0.0.1:8000/storage/alexwilliams/path/to/my/folder'
```

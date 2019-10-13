# rust-protobuf-pyo3-example

Example using protobuf for I/F of pyo3

## Directory structures

```bash
├── main.py  # python test script
├── proto  # protobuf definitions
│   └── function_a
│       └── v1
│           └── function.proto
├── proto_example  # python package
│   ├── __init__.py
│   └── generated  # python protobuf client
│       ├── __init__.py
│       └── function_a
│           └── v1
│               └── function_pb2.py
└── src  # rust crate
    ├── generated  # rust protobuf client
    │   ├── function_a
    │   │   ├── mod.rs
    │   │   └── v1
    │   │       ├── function.rs
    │   │       └── mod.rs
    │   └── mod.rs
    └── lib.rs
```

## Build

```bash
$ python setup.py develop
```

## Testing

```bash
$ python main.py
[src/lib.rs:11] &req = descriptions: "hello from python"

rust response: descriptions: "hello from rust"

```

# config_lite

A lightweight and customizable configuration management library for binary crates. Inspired by [node config](https://www.npmjs.com/package/config)

Still a work in progress

## Example

```rust
let config = config::Config::new();

let value = config::get<String>("value");
let user = config::get<User>("value.user");
```

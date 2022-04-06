# config_lite

A lightweight and customizable configuration management library for binary crates. Inspired by [node config](https://www.npmjs.com/package/config)

### Caveats
The values read from the configuration files must be owned, meaning they cannot be deserialized into reference types

Still a work in progress

## Example

```rust
use config::Config;

let config = Config::new()?;

let value = config.get<String>("value")?;
let user = config.get<User>("value.user")?;
```

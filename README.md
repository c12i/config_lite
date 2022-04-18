# config_lite

A lightweight and customizable configuration management library for binary crates. Inspired by [node config](https://www.npmjs.com/package/config)

### Caveats
The values read from the configuration files must be owned, meaning they cannot be deserialized with [serde](https://github.com/serde-rs/serde) into reference types

Still a work in progress

## Example

Given the following json config file:

```json
{
  "foo": "bar",
  "test": {
   "user": {
      "id": 1,
      "name": "Foo Baz",
      "screen_name": "foo_baz",
      "isActive": true
    }
  },
  "database": {
    "password": "{{DATABASE_PASSWORD}}" // config will read this value from the env var $DATABASE_PASSWORD
  }
}
```

The configurations can be read like so:

```rust
use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
  id: u32,
  name: String,
  screen_name: String,
  #[serde(rename(deserialize = "isActive"))]
  is_active: bool,
}

let config = Config::new()?;

let value = config.get::<String>("foo")?;
let user = config.get::<User>("test.user")?;
let database_password = config.get::<String>("database.password")?;
```

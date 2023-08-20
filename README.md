# config_lite

This lightweight and customizable Rust library provides a simple configuration file parser that can read 
JSON and YAML files as well as referencing environment variables defined in the config file.

It defines a Config struct that represents a configuration file and provides methods for initializing and retrieving values from it.

Inspired by [node config](https://www.npmjs.com/package/config)

## Installation

You can add this library to your project by adding the following to your Cargo.toml file:

```toml
[dependencies]
config_lite = "2.1.0-beta"
```

## Usage

To use the Config struct, you can import it and call its methods as follows, 
given the configuration json file below in `config/default.json` at the root dir of your
project:

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

```rs
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

let config = Config::init()?;

let value = config.get::<String>("foo")?;
let user = config.get::<User>("test.user")?;
let database_password = config.get::<String>("database.password")?;
```

The `init()` method initializes the Config object by searching for the configuration file in 
the file system and reading its contents. 

The `get()` method retrieves a value from the configuration file given a string path, delimited
with a period `.`.

## Customization
Some attributes of the library can be customized by the user:

1. The config environment which will determine which config file to read from can be set via the 
`CONFIG_LITE_ENV` env var. The default environment is `default`. Which expects at the very least
a `default.json` or `default.yaml` config file to exist in the config directory
2. The config directory name can be customized by changing the `CONFIG_LITE_DIRECTORY_NAME`
env var. The default directory name is `config` and is expected to be defined at the root
dir of your crate.

## Caveats
The values from the config type must be mapped to owned types and not shared types.

## License
This library is released under the MIT License. See the LICENSE file for more information.

## Contributing
I am still open to build upon this library, currently what is out is a proof of concept.
If you would like to contribute to this project, 
feel free to open a pull request or issue on the [GitHub repository](https://github.com/collinsmuriuki/config_lite).
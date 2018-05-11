# web_logger

Implements a logger that can be used in web browsers.

## Usage

### In libraries

Not required. Libraries should use the [`log`](https://doc.rust-lang.org/log) crate instead.

### In executables

It must be added along with `log` to the project dependencies:

```toml
[dependencies]
log = "0.4"
web_logger = "0.1"
```

`web_logger` must be initialized as early as possible in the project.
After it's initialized, you can use the `log` macros to do actual logging.

```rust
#[macro_use]
extern crate log;
extern crate web_logger;

fn main() {
    web_logger::init();

    info!("starting up");

    // ...
}
```

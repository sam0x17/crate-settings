# crate-settings

This crate allows you to specify crate-specific metadata settings in the
`package.metadata.settings` table in `Cargo.toml`. These settings can then be read at
compile-time using the `settings!` macro. `crate-settings` is designed to automatically find
the real workspace root of the current project and uses this directory to key off a search for
the relevant `Cargo.toml`.

## Example

```toml
# Cargo.toml
[package]
name = "my-crate"
# ..

[package.metadata.settings.some-crate]
some_setting = "value in settings"
some_int = 567
an_array = [1, 2, 3]
```

```rust
use crate_settings::*;

const SOME_SETTING: &'static str = settings!("some-crate", "some_setting", "my default value");
const SOME_INT: isize = settings!("some-crate", "some_int", 145);

fn main() {
    let items = settings!("some-crate", "an_array", [4, 5, 6]);
    for item in items {
        println!("{}", item);
    }
}
```

The first value passed to `settings!` is the _name_ of the crate these settings pertain to.
This should always just be the name of the current crate, and should match whatever you have in
the `[package]` section of your `Cargo.toml`. This field is required to avoid name collisions
between keys belonging to different crates, and is also used to locate the `Cargo.toml` for the
crate in certain situations.

The second value passed to `settings!` should be the _key_ you want to read from settings. This
should be a valid `TOML` field name.

The last value is optional, and lets you specify a _default_ value that will be used if the
specified key cannot be found. If you do not specify a _default_, then a compiler error will be
issued if the key cannot be found. When a _default_ is specified, however, any failure will
cause the default value to be used, meaning these failures will be silent.

## Directory Traversal

In situations where multiple levels of `Cargo.toml` are present (i.e. crates nested inside of
each other), the _most specific_ version of a key will be used, so for example if a main crate
and a sub-crate both specify a value for the same crate name and key pair, the sub-crate's
value will be used. This allows the main crate to set a value that sub-crates can either use or
override.

## Notes

This is not yet production-ready. I need to publish to crates.io to properly test how this
behaves in multi-workspace environments. API is subject to change.

# Features

Features provide a mechanism to express conditional compilation, which allows for enabling or disabling specific parts of the code during the build process.

## `[features]` section

A package defines a set of features in the `[features]` section of the `Scarb.toml` file. Each feature specifies an array of other features that it enables.

For example, a package that supports many hash functions may define the following features:

```toml
[features]
poseidon = []
pedersen = []
keccak = []
```

With these features defined, cfg expressions can be used to conditionally include code to support the requested feature at compile time. For example:

```rust
// Conditionally include a package
#[cfg(feature: 'poseidon')]
mod poseidon;

// Conditionally define a function
#[cfg(feature: 'pedersen')]
fn hash_pedersen(value: felt252) -> felt252 {
  // ...
}
```

To enable certain features, pass the `--features` flag with a list of features to be enabled. Multiple features should be separated by commas. For instance, to build this package with only the `poseidon` and `pedersen` features enabled, run:


```
scarb build --features poseidon,pedersen
```

It is also possible to enable all features by passing the `--all-features` flag.

## `default` feature

By default, all features are disabled unless explicitly enabled with the `--features` flag. This behavior can be changed by specifying a default feature, e.g.:

```toml
[features]
default = ["poseidon", "pedersen"]
poseidon = []
pedersen = []
keccak = []
```

When building, the compiler will enable the default feature, which, in turn, enables all listed features.

The default feature can be disabled by passing the `--no-default-features` flag.

So, for the example above, running `scarb build` would enable `poseidon` and `pedersen` features; `scarb build --features keccak` - all `poseidon`, `pedersen` and `keccak` features; and `scarb build --no-default-features --features keccak` - only `keccak` feature.

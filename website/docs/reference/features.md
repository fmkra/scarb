# Features

Features provide a machanism to express conditional compilation,
which allows enabling or disabling certain parts of code during build.

## `[features]` section

A package defines a set of features in the `[features]` section of `Scarb.toml` file.
Each feature specifies an array of other features that it enables.

For example, a package which supports many hash functions may define the following features.
```toml
[features]
poseidon = []
pedersen = []
keccak = []
```

With those features defined, cfg expressions can be used to conditionally include code to support the requested feature at compile time.
For example
```
// Conditionally include package
#[cfg(feature: 'poseidon')]
mod poseidon;

// Conditionally define a function
#[cfg(feature: 'pedersen')]
fn hash_pedersen(value: felt252) -> felt252 {
  // ...
}
```

To enable certain features, pass `--features` flag with list of features to be enabled.
Multiple features should be separated with commas.
For instance, to build this package with only `poseidon` and `pedersen` enabled, run
```
scarb build --features poseidon,pedersen
```

## TODO: default features

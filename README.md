# security-translocate-sys
Direct Rust FFI bindings to Apple’s `libsecurity_translocate`, part of the
Security framework, which facilitates the creation and destruction of app
translocation points. These bindings are based on the SecTranslocate header,
available [here](https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h).

## Install

Run the following Cargo command in your project directory:

```sh
cargo add security-translocate-sys
```

Or add the following line to your Cargo.toml:

```toml
security-translocate-sys = "0.1.0"
```

## Documentation

https://docs.rs/security-translocate-sys

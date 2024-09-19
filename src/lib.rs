//! Direct Rust FFI bindings to Apple’s `libsecurity_translocate`, part of the
//! Security framework, which facilitates the creation and destruction of app
//! translocation points. These bindings are based on the SecTranslocate header,
//! available [here](https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h).

mod annotations;
pub use annotations::*;

mod bindings;
pub use bindings::*;

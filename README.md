# pebble-sys

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/pebble-sys)
[![Crates.io](https://img.shields.io/crates/v/pebble-sys)](https://crates.io/crates/pebble-sys)
[![Docs.rs](https://docs.rs/pebble-sys/badge.svg)](https://docs.rs/crates/pebble-sys)

![Rust nightly-2020-10-31](https://img.shields.io/static/v1?logo=Rust&label=&message=nightly-2020-10-31&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/pebble-sys.svg?branch=develop)](https://travis-ci.com/Tamschi/pebble-sys/branches)
![Crates.io - License](https://img.shields.io/crates/l/pebble-sys/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/pebble-sys)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/pebble-sys)](https://github.com/Tamschi/pebble-sys/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/pebble-sys)](https://github.com/Tamschi/pebble-sys/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/pebble-sys.svg)](https://web.crev.dev/rust-reviews/crate/pebble-sys/)

Low-level FFI bindings for Pebble (watch) SDK 4.3.

I recommend using the high-level wrapper in [`pebble-skip`] instead, since it provides almost the same functionality with full memory safety and with very little overhead.

[`pebble-skip`]: https://github.com/Tamschi/pebble-skip

This crate is still heavily work in progress, so expect frequent breaking changes and missing functionality before 0.1. If you'd like me to prioritise a specific API, please [file a feature request on GitHub].

[file a feature request on GitHub]: https://github.com/Tamschi/pebble-sys/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add pebble-sys
```

## Example

<!-- markdownlint-disable no-inline-html -->
<img alt="Aplite emulator screenshot: 'miles to see you' and 10000 in a number picker window" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAJAAAACoAQMAAAA1ofU7AAAABlBMVEX///8AAABVwtN+AAABo0lEQVRIx+3TsWrrMBQG4F8IrpZQrRku1itcU7iTsV8lU+96oYsH07p46OJ3yKsoFJolkDVTUckDVN00GKtKqpTUh1IoLQ2hHj+sc36dg/D6K7z3P3TUNNLAn3cJQ+Ln7U2ju//imtc78ov53HROLq91oOmGxovF2SRtR6vFJJCM9A9pezLeUC+eabmu01aopUHR8Uj3Om35dEOOxYMwactWI6CwiDQJ5fFc3tOOkUKuLObaUUj/e5v+sNbxHTRKb8ZDOl0T+ns6pF9nHyR++0FizdoN6Wq2GlINvE8ShE4OYR2fQpeE+oJQpwg5+pcltR4M6TifEWoaQjNByEhCVhFyRaAEF0AH5sAtiu4yUM98zT1ED9mh6L0H77nXwtfSQ/Uh/ZaED2208nXhI93JxiphcqkrVW8JiYRLmM24KYWOlMBlzJbc2D0qM9iSGct3lKOq4Bx7tNxEypCVKC0eDftyoiHejqpQ5rAVM26PbM5MxbUTL5O4soqbXOhK7oiMEAkZNJL9dQQaLO1gXsfxkwUhxwh1nFAvaHlJaXoAd/yho6En7hDX5UR0V1MAAAAASUVORK5CYII=">
<!-- markdownlint-enable no-inline-html -->

```rust
#![no_std]

use pebble_sys::{
  foundation::app::app_event_loop,
  standard_c::memory::{c_str, void},
  user_interface::{
    window::number_window::{
      number_window_create, number_window_get_window_mut, number_window_set_value,
      NumberWindowCallbacks,
    },
    window_stack::window_stack_push,
  },
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
  static mut CONTEXT: () = ();

  unsafe {
    let label = &*("miles to see you\0" as *const _ as *const c_str);
    let number_window = number_window_create(
      label,
      NumberWindowCallbacks {
        incremented: None,
        decremented: None,
        selected: None,
      },
      &mut *(&mut CONTEXT as *mut _ as *mut void),
    )
    .unwrap();
    number_window_set_value(number_window, 10_000);
    let window = number_window_get_window_mut(number_window);
    window_stack_push(window, true);
    app_event_loop();
    0
  }
}

```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`pebble-sys` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

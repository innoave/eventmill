# eventmill &emsp;

[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Rustc Support 1.39+]][Rust 1.39]
[![Build Status]][actions]

[Latest Release]: https://img.shields.io/crates/v/eventmill.svg
[crates.io]: https://crates.io/crates/eventmill
[Documentation]: https://docs.rs/eventmill/badge.svg 
[docs.rs]: https://docs.rs/eventmill
[License]: https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg
[MIT]: https://opensource.org/licenses/MIT
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[Build Status]: https://img.shields.io/github/workflow/status/innoave/eventmill/CI/master
[actions]: https://github.com/innoave/eventmill/actions?query=branch%3Amaster
[Rustc Support 1.39+]: https://img.shields.io/badge/rustc-1.39+-lightgray.svg
[Rust 1.39]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html

**Event sourcing and CQRS for Rust applications**

> very much work in progress!

```toml
[dependencies]
eventmill = "0.1"
```

to use the `derive` macros from the `eventmill-derive` crate activate the `derive` feature:

```toml
[dependencies]
eventmill = { version = "0.1", features = ["derive"] }    
```

## TODO

* [X] define basic abstractions and API
* [X] provide a first example on how it looks like to use the API
* [ ] make more examples to polish the API
* [ ] write rust-doc for the API
* [ ] support async/await or switch to async as the only option
* [ ] consider providing default implementations for eventstores and other building blocks
* [ ] ...

# `wasm-bindgen` Change Log

--------------------------------------------------------------------------------

## Unreleased

Released YYYY-MM-DD.

### Added

* TODO (or remove section if none)

### Changed

* TODO (or remove section if none)

### Deprecated

* TODO (or remove section if none)

### Removed

* TODO (or remove section if none)

### Fixed

* TODO (or remove section if none)

### Security

* TODO (or remove section if none)

--------------------------------------------------------------------------------

## 0.2.21

Released 2018-09-07

### Added

* Added many more bindings for `WebAssembly` in the `js-sys` crate.

### Fixed

* The "names" section of the wasm binary is now correctly preserved by
  wasm-bindgen.

--------------------------------------------------------------------------------

## 0.2.20

Released 2018-09-06

### Added

* All of `wasm-bindgen` is configured to compile on stable Rust as of the
  upcoming 1.30.0 release, scheduled for October 25, 2018.
* The underlying `JsValue` of a `Closure<T>` type can now be extracted at any
  time.
* Initial and experimental support was added for modules that have shared memory
  (use atomic instructions).

### Removed

* The `--wasm2asm` flag of `wasm2es6js` was removed because the `wasm2asm` tool
  has been removed from upstream Binaryen. This is replaced with the new
  `wasm2js` tool from Binaryen.

### Fixed

* The "schema" version for wasm-bindgen now changes on all publishes, meaning we
  can't forget to update it. This means that the crate version and CLI version
  must exactly match.
* The `wasm-bindgen` crate now has a `links` key which forbids multiple versions
  of `wasm-bindgen` from being linked into a dependency graph, fixing obscure
  linking errors with a more first-class error message.
* Binary releases for Windows has been fixed.

--------------------------------------------------------------------------------

## 0.2.19 (and 0.2.18)

Released 2018-08-27.

### Added

* Added bindings to `js-sys` for some `WebAssembly` types.
* Added bindings to `js-sys` for some `Intl` types.
* Added bindings to `js-sys` for some `String` methods.
* Added an example of using the WebAudio APIs.
* Added an example of using the `fetch` API.
* Added more `extends` annotations for types in `js-sys`.
* Experimental support for `WeakRef` was added to automatically deallocate Rust
  objects when gc'd.
* Added support for executing `wasm-bindgen` over modules that import their
  memory.
* Added a global `memory()` function in the `wasm-bindgen` crate for accessing
  the JS object that represent wasm's own memory.

### Removed

* Removed `AsMut` implementations for imported objects.

### Fixed

* Fixed the `constructor` and `catch` attributes combined on imported types.
* Fixed importing the same-named static in two modules.

--------------------------------------------------------------------------------

## 0.2.17

Released 2018-08-16.

### Added

* Greatly expanded documentation in the wasm-bindgen guide.
* Added bindings to `js-sys` for `Intl.DateTimeFormat`
* Added a number of `extends` attributes for types in `js-sys`

### Fixed

* Fixed compile on latest nightly with latest `proc-macro2`
* Fixed compilation in some scenarios on Windows with paths in `module` paths

--------------------------------------------------------------------------------

## 0.2.16

Released 2018-08-13.

### Added

* Added the `wasm_bindgen::JsCast` trait, as described in [RFC #2][rfc-2].
* Added [the `#[wasm_bindgen(extends = ...)]` attribute][extends-attr] to
  describe inheritance relationships, as described in [RFC #2][rfc-2].
* Added support for receiving `Option<&T>` parameters from JavaScript in
  exported Rust functions and methods.
* Added support for receiving `Option<u32>` and other option-wrapped scalars.
* Added reference documentation to the guide for every `#[wasm_bindgen]`
  attribute and how it affects the generated bindings.
* Published the `wasm-bindgen-futures` crate for converting between JS
  `Promise`s and Rust `Future`s.

### Changed

* Overhauled the guide's documentation on passing JS closures to Rust, and Rust
  closures to JS.
* Overhauled the guide's documentation on using serde to serialize complex data
  to `JsValue` and deserialize `JsValue`s back into complex data.
* Static methods are now always bound to their JS class, as is required for
  `Promise`'s static methods.

### Removed

* Removed internal usage of `syn`'s `visit-mut` cargo feature, which should
  result in faster build times.

### Fixed

* Various usage errors for the `#[wasm_bindgen]` proc-macro are now properly
  reported with source span information, rather than `panic!()`s inside the
  proc-macro.
* Fixed a bug where taking a struct by reference and returning a slice resulted
  in lexical variable redeclaration errors in the generated JS glue. [#662][]
* The `#[wasm_bindgen(js_class = "....")]` attribute for binding methods to
  renamed imported JS classes now properly works with constructors.

[rfc-2]: https://rustwasm.github.io/rfcs/002-wasm-bindgen-inheritance-casting.html
[extends-attr]: https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html
[#662]: https://github.com/rustwasm/wasm-bindgen/pull/662

--------------------------------------------------------------------------------

## 0.2.15

Released 2018-07-26.

### Fixed

* Fixed `wasm-bindgen` CLI version mismatch checks that got broken in the last
  point release.

--------------------------------------------------------------------------------

## 0.2.14

Released 2018-07-25.

### Fixed

* Fixed compilation errors on targets that use
  Mach-O. [#545](https://github.com/rustwasm/wasm-bindgen/issues/545)

--------------------------------------------------------------------------------

## 0.2.13

Released 2018-07-22.

### Added

* Support the `#[wasm_bindgen(js_name = foo)]` attribute on exported functions
  and methods to allow renaming an export to JS. This allows JS to call it by
  one name and Rust to call it by another, for example using `camelCase` in JS
  and `snake_case` in Rust

### Fixed

* Compilation with the latest nightly compiler has been fixed (nightlies on and
  after 2018-07-21)

--------------------------------------------------------------------------------

## 0.2.12

Released 2018-07-19.

This release is mostly internal refactorings and minor improvements to the
existing crates and functionality, but the bigs news is an upcoming `js-sys` and
`web-sys` set of crates. The `js-sys` crate will expose [all global JS
bindings][js-all] and the `web-sys` crate will be generated from WebIDL to
expose all APIs browsers have. More info on this soon!

[js-all]: https://github.com/rustwasm/wasm-bindgen/issues/275

### Added

* Support for `Option<T>` was added where `T` can be a number of slices or
  imported types.
* Comments in Rust are now preserved in generated JS bindings, as well as
  comments being generated to indicate the types of arguments/return values.
* The online documentation has been reorganized [into a book][book].
* The generated JS is now formatted better by default for readability.
* A `--keep-debug` flag has been added to the CLI to retain debug sections by
  default. This happens by default when `--debug` is passed.

[book]: https://rustwasm.github.io/wasm-bindgen/

### Fixed

* Compilation with the latest nightly compiler has been fixed (nightlies on and
  after 2018-07-19)
* Declarations of an imported function in multiple crates have been fixed to not
  conflict.
* Compilation with `#![deny(missing_docs)]` has been fixed.

--------------------------------------------------------------------------------

## 0.2.11

Released 2018-05-24.

--------------------------------------------------------------------------------

## 0.2.10

Released 2018-05-17.

--------------------------------------------------------------------------------

## 0.2.9

Released 2018-05-11.

[Rustache](https://rustache.github.io) [![Build Status](https://travis-ci.org/rustache/rustache.svg?branch=master)](https://travis-ci.org/rustache/rustache)
====

Rustache is a Rust implementation of the Mustache spec.

## Documentation

The different Mustache tags are documented at [mustache(5)](http://mustache.github.com/mustache.5.html).

## Install

Install it through Cargo:

```toml
[dependencies.rustache]
git = "https://github.com/rustache/rustache"
```

Then link it within your crate like so:

```rust
extern crate rustache;
```

## API Methods

The main forward interface that users will interact with when using Rustache is the `rustache::render` method like so:

```rust
rustache::render("path/to/template.html", &data)
```

Users also have the option to utilize more focused API methods for 
interfacing with Rustache. The following methods handle template 
files from a provided file path:

```rust
// Renders a template file from a HashBuilder to a specified writer
rustache::render_file_from_hb("path/to/template.html", &data, &writer)

// Renders a template file from a JSON enum to a specified writer
rustache::render_file_from_json_enum("path/to/template.html", &data, &writer)

// Renders a template file from a JSON string to a specified writer
rustache::render_file_from_json_string("path/to/template.html", &str, &writer)

// Renders a template file from a JSON file to a specified writer
rustache::render_file_from_json_file("path/to/template.html", "data/data.json", &writer)
```

The following methods handle templates in the form of text strings:

```rust
// Render template text from a specified HashBuilder to a specified writer
rustache::render_text_from_hb("{{ name }}", &data, &writer)

// Render template text from a JSON enum to a specified writer
rustache::render_text_from_json_enum("{{ name }}", &json, &writer)

// Render template text from a JSON string to a specified writer
rustache::render_text_from_json_string("{{ name }}", &str, &writer)

// Render template text from a JSON file to a specified writer
rustache::render_text_from_json_file("{{ name }}", "data/data.json", &writer)
```
====

## Examples

====

## Testing

Simply clone and run:

```bash
cargo test
```
====

## Contribute

====

## License

Copyright (c) 2014 Team Rustache

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


Inspired by https://github.com/erickt/rust-mustache:

Copyright (c) 2012 Erick Tryzelaar

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

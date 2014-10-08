extern crate rustache;
extern crate memstream;

use std::io::MemWriter;
// use memstream::MemStream;
use rustache::HashBuilder;
// use rustache::Render;

// - name: Interpolation
//     desc: A lambda's return value should be interpolated.
//     data:
//       lambda: !code
//         ruby:    'proc { "world" }'
//         perl:    'sub { "world" }'
//         js:      'function() { return "world" }'
//         php:     'return "world";'
//         python:  'lambda: "world"'
//         clojure: '(fn [] "world")'
//     template: "Hello, {{lambda}}!"
//     expected: "Hello, world!"
#[test]
fn test_spec_lambdas_interpolation() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_lambda("lambda", |_| {
                     "world".to_string()               
                 });

    rustache::render_text_from_hb("Hello, {{lambda}}!", &data, &mut w);

    assert_eq!("Hello, world!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Interpolation - Expansion
//     desc: A lambda's return value should be parsed.
//     data:
//       planet: "world"
//       lambda: !code
//         ruby:    'proc { "{{planet}}" }'
//         perl:    'sub { "{{planet}}" }'
//         js:      'function() { return "{{planet}}" }'
//         php:     'return "{{planet}}";'
//         python:  'lambda: "{{planet}}"'
//         clojure: '(fn [] "{{planet}}")'
//     template: "Hello, {{lambda}}!"
//     expected: "Hello, world!"
#[test]
fn test_spec_lambdas_interpolation_expansion() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                    .insert_string("planet", "world")
                    .insert_lambda("lambda", |_| {
                     "{{planet}}".to_string()               
                 });

    rustache::render_text_from_hb("Hello, {{lambda}}!", &data, &mut w);

    assert_eq!("Hello, world!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Interpolation - Alternate Delimiters
//     desc: A lambda's return value should parse with the default delimiters.
//     data:
//       planet: "world"
//       lambda: !code
//         ruby:    'proc { "|planet| => {{planet}}" }'
//         perl:    'sub { "|planet| => {{planet}}" }'
//         js:      'function() { return "|planet| => {{planet}}" }'
//         php:     'return "|planet| => {{planet}}";'
//         python:  'lambda: "|planet| => {{planet}}"'
//         clojure: '(fn [] "|planet| => {{planet}}")'
//     template: "{{= | | =}}\nHello, (|&lambda|)!"
//     expected: "Hello, (|planet| => world)!"
// #[test]
// fn test_spec_lambdas_interpolation_alternate_delimeters() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("planet", "world")
//                 .insert_lambda("lambda", |_| {
//                     "|planet| => {{planet}}".to_string()               
//                 });

//     rustache::render_text_from_hb("{{= | | =}}\nHello, (|&lambda|)!", &data, &mut w);

//     assert_eq!("Hello, (|planet| => world)!".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Interpolation - Multiple Calls
//     desc: Interpolated lambdas should not be cached.
//     data:
//       lambda: !code
//         ruby:    'proc { $calls ||= 0; $calls += 1 }'
//         perl:    'sub { no strict; $calls += 1 }'
//         js:      'function() { return (g=(function(){return this})()).calls=(g.calls||0)+1 }'
//         php:     'global $calls; return ++$calls;'
//         python:  'lambda: globals().update(calls=globals().get("calls",0)+1) or calls'
//         clojure: '(def g (atom 0)) (fn [] (swap! g inc))'
//     template: '{{lambda}} == {{{lambda}}} == {{lambda}}'
//     expected: '1 == 2 == 3'
#[test]
fn test_spec_lambdas_interpolation_multiple_calls() {
    let mut w = MemWriter::new();
    let mut calls = 0u;
    let data = HashBuilder::new()
                .insert_lambda("lambda", |_| {
                    calls += 1;
                    calls.to_string()
                });

    rustache::render_text_from_hb("{{lambda}} == {{{lambda}}} == {{lambda}}", &data, &mut w);

    assert_eq!("1 == 2 == 3".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Escaping
//     desc: Lambda results should be appropriately escaped.
//     data:
//       lambda: !code
//         ruby:    'proc { ">" }'
//         perl:    'sub { ">" }'
//         js:      'function() { return ">" }'
//         php:     'return ">";'
//         python:  'lambda: ">"'
//         clojure: '(fn [] ">")'
//     template: "<{{lambda}}{{{lambda}}}"
//     expected: "<&gt;>"
#[test]
fn test_spec_lambdas_escaping() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_lambda("lambda", |_| {
                    ">".to_string()               
                });


    rustache::render_text_from_hb("<{{lambda}}{{{lambda}}}", &data, &mut w);

    assert_eq!("<&gt;>".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Section
//     desc: Lambdas used for sections should receive the raw section string.
//     data:
//       x: 'Error!'
//       lambda: !code
//         ruby:    'proc { |text| text == "{{x}}" ? "yes" : "no" }'
//         perl:    'sub { $_[0] eq "{{x}}" ? "yes" : "no" }'
//         js:      'function(txt) { return (txt == "{{x}}" ? "yes" : "no") }'
//         php:     'return ($text == "{{x}}") ? "yes" : "no";'
//         python:  'lambda text: text == "{{x}}" and "yes" or "no"'
//         clojure: '(fn [text] (if (= text "{{x}}") "yes" "no"))'
//     template: "<{{#lambda}}{{x}}{{/lambda}}>"
//     expected: "<yes>"
#[test]
fn test_spec_lambdas_section() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_string("x", "Error!")
                .insert_lambda("lambda", |txt| {
                    if txt.as_slice() == "{{x}}" {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    }
                });

    rustache::render_text_from_hb("<{{#lambda}}{{x}}{{/lambda}}>", &data, &mut w);

    assert_eq!("<yes>".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Section - Expansion
//     desc: Lambdas used for sections should have their results parsed.
//     data:
//       planet: "Earth"
//       lambda: !code
//         ruby:    'proc { |text| "#{text}{{planet}}#{text}" }'
//         perl:    'sub { $_[0] . "{{planet}}" . $_[0] }'
//         js:      'function(txt) { return txt + "{{planet}}" + txt }'
//         php:     'return $text . "{{planet}}" . $text;'
//         python:  'lambda text: "%s{{planet}}%s" % (text, text)'
//         clojure: '(fn [text] (str text "{{planet}}" text))'
//     template: "<{{#lambda}}-{{/lambda}}>"
//     expected: "<-Earth->"
#[test]
fn test_spec_lambdas_section_expansion() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_string("planet", "Earth")
                .insert_lambda("lambda", |txt| {
                    let mut result = txt.clone();
                    result.push_str("{{planet}}");
                    result.push_str(txt.as_slice());
                    result
                 });

    rustache::render_text_from_hb("<{{#lambda}}-{{/lambda}}>", &data, &mut w);

    assert_eq!("<-Earth->".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Section - Alternate Delimiters
//     desc: Lambdas used for sections should parse with the current delimiters.
//     data:
//       planet: "Earth"
//       lambda: !code
//         ruby:    'proc { |text| "#{text}{{planet}} => |planet|#{text}" }'
//         perl:    'sub { $_[0] . "{{planet}} => |planet|" . $_[0] }'
//         js:      'function(txt) { return txt + "{{planet}} => |planet|" + txt }'
//         php:     'return $text . "{{planet}} => |planet|" . $text;'
//         python:  'lambda text: "%s{{planet}} => |planet|%s" % (text, text)'
//         clojure: '(fn [text] (str text "{{planet}} => |planet|" text))'
//     template: "{{= | | =}}<|#lambda|-|/lambda|>"
//     expected: "<-{{planet}} => Earth->"
// #[test]
// fn test_spec_lambdas_section_alternate_delimeters() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("planet", "Earth")
//                 .insert_lambda("lambda", |txt| {
//                     let mut result = txt.to_string();
//                     result.push_str("{{planet}} => |planet|");
//                     result.push_str(txt.as_slice());
//                     result
//                 });

//     rustache::render_text_from_hb("{{= | | =}}<|#lambda|-|/lambda|>", &data, &mut w);

//     assert_eq!("<-{{planet}} => Earth->".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Section - Multiple Calls
//     desc: Lambdas used for sections should not be cached.
//     data:
//       lambda: !code
//         ruby:    'proc { |text| "__#{text}__" }'
//         perl:    'sub { "__" . $_[0] . "__" }'
//         js:      'function(txt) { return "__" + txt + "__" }'
//         php:     'return "__" . $text . "__";'
//         python:  'lambda text: "__%s__" % (text)'
//         clojure: '(fn [text] (str "__" text "__"))'
//     template: '{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}'
//     expected: '__FILE__ != __LINE__'
#[test]
fn test_spec_lambdas_section_multiple_calls() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_lambda("lambda", |txt| {
                    let mut result = "__".to_string();
                    result.push_str(txt.as_slice());
                    result.push_str("__");
                    result
                });

    rustache::render_text_from_hb("{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}", &data, &mut w);

    assert_eq!("__FILE__ != __LINE__".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Inverted Section
//     desc: Lambdas used for inverted sections should be considered truthy.
//     data:
//       static: 'static'
//       lambda: !code
//         ruby:    'proc { |text| false }'
//         perl:    'sub { 0 }'
//         js:      'function(txt) { return false }'
//         php:     'return false;'
//         python:  'lambda text: 0'
//         clojure: '(fn [text] false)'
//     template: "<{{^lambda}}{{static}}{{/lambda}}>"
//     expected: "<>"
#[test]
fn test_spec_lambdas_inverted_section() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_string("static", "static")
                .insert_lambda("lambda", |_| {
                    "false".to_string()
                });

    rustache::render_text_from_hb("<{{^lambda}}{{static}}{{/lambda}}>", &data, &mut w);

    assert_eq!("<>".to_string(), String::from_utf8(w.unwrap()).unwrap());
}


// #[test]
// fn test_spec_lambdas_interpolation_using_render_text() {
//     let mut s = MemStream::new();
//     let data = HashBuilder::new()
//                 .insert_lambda("lambda", |_| {
//                      "world".to_string()               
//                  });
//     let s = rustache::render_text("Hello, {{lambda}}!", data);

//     assert_eq!("Hello, world!".to_string(), String::from_utf8(s.unwrap()).unwrap());
// }

// #[test]
// fn test_spec_lambdas_inverted_section_using_render_text() {
//     let data = HashBuilder::new()
//                 .insert_string("static", "static")
//                 .insert_lambda("lambda", |_| {
//                     "false".to_string()
//                 });

//     let s = rustache::render_text("<{{^lambda}}{{static}}{{/lambda}}>", data);

//     assert_eq!("<>".to_string(), String::from_utf8(s.unwrap()).unwrap());
// }

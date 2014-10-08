extern crate rustache;

use std::io::MemWriter;

use rustache::HashBuilder;

// - name: No Interpolation
//   desc: Mustache-free templates should render as-is.
//   data: { }
//   template: |
//     Hello from {Mustache}!
//   expected: |
//     Hello from {Mustache}!
#[test]
fn test_spec_interpolation_none() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    rustache::render_text_from_hb("Hello from {Mustache}!", &data, &mut w);

    assert_eq!("Hello from {Mustache}!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Basic Interpolation
//   desc: Unadorned tags should interpolate content into the template.
//   data: { subject: "world" }
//   template: |
//     Hello, {{subject}}!
//   expected: |
//     Hello, world!
#[test]
fn test_spec_interpolation_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("subject", "world");

    rustache::render_text_from_hb("Hello, {{subject}}!", &data, &mut w);

    assert_eq!("Hello, world!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: HTML Escaping
//   desc: Basic interpolation should be HTML escaped.
//   data: { forbidden: '& " < >' }
//   template: |
//     These characters should be HTML escaped: {{forbidden}}
//   expected: |
//     These characters should be HTML escaped: &amp; &quot; &lt; &gt;
#[test]
fn test_spec_interpolation_html_escaping() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("forbidden", "& \" < >");

    rustache::render_text_from_hb("These characters should be HTML escaped: {{forbidden}}", &data, &mut w);

    assert_eq!("These characters should be HTML escaped: &amp; &quot; &lt; &gt;".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache
//   desc: Triple mustaches should interpolate without HTML escaping.
//   data: { forbidden: '& " < >' }
//   template: |
//     These characters should not be HTML escaped: {{{forbidden}}}
//   expected: |
//     These characters should not be HTML escaped: & " < >
#[test]
fn test_spec_interpolation_no_html_escaping_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("forbidden", "& \" < >");

    rustache::render_text_from_hb("These characters should not be HTML escaped: {{{forbidden}}}", &data, &mut w);

    assert_eq!("These characters should not be HTML escaped: & \" < >".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand
//   desc: Ampersand should interpolate without HTML escaping.
//   data: { forbidden: '& " < >' }
//   template: |
//     These characters should not be HTML escaped: {{&forbidden}}
//   expected: |
//     These characters should not be HTML escaped: & " < >
#[test]
fn test_spec_interpolation_no_html_escaping_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("forbidden", "& \" < >");

    rustache::render_text_from_hb("These characters should not be HTML escaped: {{&forbidden}}", &data, &mut w);

    assert_eq!("These characters should not be HTML escaped: & \" < >".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Basic Integer Interpolation
//   desc: Integers should interpolate seamlessly.
//   data: { mph: 85 }
//   template: '"{{mph}} miles an hour!"'
//   expected: '"85 miles an hour!"'
#[test]
fn test_spec_interpolation_integer_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_int("mph", 85);

    rustache::render_text_from_hb("{{mph}} miles an hour!", &data, &mut w);

    assert_eq!("85 miles an hour!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache Integer Interpolation
//   desc: Integers should interpolate seamlessly.
//   data: { mph: 85 }
//   template: '"{{{mph}}} miles an hour!"'
//   expected: '"85 miles an hour!"'
#[test]
fn test_spec_interpolation_integer_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_int("mph", 85);

    rustache::render_text_from_hb("{{{mph}}} miles an hour!", &data, &mut w);

    assert_eq!("85 miles an hour!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand Integer Interpolation
//   desc: Integers should interpolate seamlessly.
//   data: { mph: 85 }
//   template: '"{{&mph}} miles an hour!"'
//   expected: '"85 miles an hour!"'
#[test]
fn test_spec_interpolation_integer_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_int("mph", 85);

    rustache::render_text_from_hb("{{mph}} miles an hour!", &data, &mut w);

    assert_eq!("85 miles an hour!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Basic Decimal Interpolation
//   desc: Decimals should interpolate seamlessly with proper significance.
//   data: { power: 1.210 }
//   template: '"{{power}} jiggawatts!"'
//   expected: '"1.21 jiggawatts!"'
#[test]
fn test_spec_interpolation_float_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_float("power", 1.210);

    rustache::render_text_from_hb("{{power}} jiggawatts!", &data, &mut w);

    assert_eq!("1.21 jiggawatts!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache Decimal Interpolation
//   desc: Decimals should interpolate seamlessly with proper significance.
//   data: { power: 1.210 }
//   template: '"{{{power}}} jiggawatts!"'
//   expected: '"1.21 jiggawatts!"'
#[test]
fn test_spec_interpolation_float_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_float("power", 1.210);

    rustache::render_text_from_hb("{{{power}}} jiggawatts!", &data, &mut w);

    assert_eq!("1.21 jiggawatts!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand Decimal Interpolation
//   desc: Decimals should interpolate seamlessly with proper significance.
//   data: { power: 1.210 }
//   template: '"{{&power}} jiggawatts!"'
//   expected: '"1.21 jiggawatts!"'
#[test]
fn test_spec_interpolation_float_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_float("power", 1.210);

    rustache::render_text_from_hb("{{&power}} jiggawatts!", &data, &mut w);

    assert_eq!("1.21 jiggawatts!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Basic Context Miss Interpolation
//   desc: Failed context lookups should default to empty strings.
//   data: { }
//   template: "I ({{cannot}}) be seen!"
//   expected: "I () be seen!"
#[test]
fn test_spec_interpolation_context_miss() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    rustache::render_text_from_hb("I ({{cannot}}) be seen!", &data, &mut w);

    assert_eq!("I () be seen!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache Context Miss Interpolation
//   desc: Failed context lookups should default to empty strings.
//   data: { }
//   template: "I ({{{cannot}}}) be seen!"
//   expected: "I () be seen!"
#[test]
fn test_spec_interpolation_context_miss_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    rustache::render_text_from_hb("I ({{{cannot}}}) be seen!", &data, &mut w);

    assert_eq!("I () be seen!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand Context Miss Interpolation
//   desc: Failed context lookups should default to empty strings.
//   data: { }
//   template: "I ({{&cannot}}) be seen!"
//   expected: "I () be seen!"
#[test]
fn test_spec_interpolation_context_miss_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    rustache::render_text_from_hb("I ({{cannot}}) be seen!", &data, &mut w);

    assert_eq!("I () be seen!".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Basic Interpolation
//   desc: Dotted names should be considered a form of shorthand for sections.
//   data: { person: { name: 'Joe' } }
//   template: '"{{person.name}}" == "{{#person}}{{name}}{{/person}}"'
//   expected: '"Joe" == "Joe"'
#[test]
fn test_spec_interpolation_dotted_names_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_hash("person", |h| { h.insert_string("name", "Joe") });

    rustache::render_text_from_hb("\"{{person.name}}\" == \"{{#person}}{{name}}{{/person}}\"", &data, &mut w);

    assert_eq!("\"Joe\" == \"Joe\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Triple Mustache Interpolation
//   desc: Dotted names should be considered a form of shorthand for sections.
//   data: { person: { name: 'Joe' } }
//   template: '"{{{person.name}}}" == "{{#person}}{{{name}}}{{/person}}"'
//   expected: '"Joe" == "Joe"'
#[test]
fn test_spec_interpolation_dotted_names_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("person", |h| {
                    h.insert_string("name", "Joe")
                });

    rustache::render_text_from_hb("\"{{{person.name}}}\" == \"{{#person}}{{{name}}}{{/person}}\"", &data, &mut w);

    assert_eq!("\"Joe\" == \"Joe\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Ampersand Interpolation
//   desc: Dotted names should be considered a form of shorthand for sections.
//   data: { person: { name: 'Joe' } }
//   template: '"{{&person.name}}" == "{{#person}}{{&name}}{{/person}}"'
//   expected: '"Joe" == "Joe"'
#[test]
fn test_spec_interpolation_dotted_names_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("person", |h| {
                    h.insert_string("name", "Joe")
                });

    rustache::render_text_from_hb("\"{{&person.name}}\" == \"{{#person}}{{&name}}{{/person}}\"", &data, &mut w);

    assert_eq!("\"Joe\" == \"Joe\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Arbitrary Depth
//   desc: Dotted names should be functional to any level of nesting.
//   data:
//     a: { b: { c: { d: { e: { name: 'Phil' } } } } }
//   template: '"{{a.b.c.d.e.name}}" == "Phil"'
//   expected: '"Phil" == "Phil"'
#[test]
fn test_spec_interpolation_dotted_names_arbitrary_depth() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("a", |h| { 
                    h.insert_hash("b", |h| {
                        h.insert_hash("c", |h| {
                            h.insert_hash("d", |h| {
                                h.insert_hash("e", |h| { 
                                    h.insert_string("name", "Phil")
                                })
                            })
                        })
                    })
                });

    rustache::render_text_from_hb("\"{{a.b.c.d.e.name}}\" == \"Phil\"", &data, &mut w);

    assert_eq!("\"Phil\" == \"Phil\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Broken Chains
//   desc: Any falsey value prior to the last part of the name should yield ''.
//   data:
//     a: { }
//   template: '"{{a.b.c}}" == ""'
//   expected: '"" == ""'
#[test]
fn test_spec_interpolation_dotted_broken_chains() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    rustache::render_text_from_hb("\"{{a.b.c}}\" == \"\"", &data, &mut w);

    assert_eq!("\"\" == \"\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Broken Chain Resolution
//   desc: Each part of a dotted name should resolve only against its parent.
//   data:
//     a: { b: { } }
//     c: { name: 'Jim' }
//   template: '"{{a.b.c.name}}" == ""'
//   expected: '"" == ""'
#[test]
fn test_spec_interpolation_dotted_broken_chain_resolution() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("a", |h| {
                    h.insert_hash("b", |h| {
                        h
                    })
                })
                .insert_hash("c", |h| {
                    h.insert_string("name", "Jim")
                });

    rustache::render_text_from_hb("\"{{a.b.c}}\" == \"\"", &data, &mut w);

    assert_eq!("\"\" == \"\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Initial Resolution
//   desc: The first part of a dotted name should resolve as any other name.
//   data:
//     a: { b: { c: { d: { e: { name: 'Phil' } } } } }
//     b: { c: { d: { e: { name: 'Wrong' } } } }
//   template: '"{{#a}}{{b.c.d.e.name}}{{/a}}" == "Phil"'
//   expected: '"Phil" == "Phil"'
#[test]
fn test_spec_interpolation_dotted_initial_resolution() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("a", |h| { 
                    h.insert_hash("b", |h| {
                        h.insert_hash("c", |h| {
                            h.insert_hash("d", |h| {
                                h.insert_hash("e", |h| { 
                                    h.insert_string("name", "Phil")
                                })
                            })
                        })
                    })
                })
                .insert_hash("b", |h| {
                    h.insert_hash("c", |h| {
                        h.insert_hash("d", |h| {
                            h.insert_hash("e", |h| { 
                                h.insert_string("name", "Wrong")
                            })
                        })
                    })
                });

    rustache::render_text_from_hb("\"{{#a}}{{b.c.d.e.name}}{{/a}}\" == \"Phil\"", &data, &mut w);

    assert_eq!("\"Phil\" == \"Phil\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Dotted Names - Context Precedence
//   desc: Dotted names should be resolved against former resolutions.
//   data:
//     a: { b: { } }
//     b: { c: 'ERROR' }
//   template: '{{#a}}{{b.c}}{{/a}}'
//   expected: ''
#[test]
fn test_spec_interpolation_dotted_context_precedence() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_hash("a", |h| {
                    h.insert_hash("b", |h| {
                        h
                    })
                })
                .insert_hash("b", |h| {
                    h.insert_hash("c", |h| {
                        h.insert_string("name", "ERROR")
                    })
                });

    rustache::render_text_from_hb("{{#a}}{{b.c}}{{/a}}", &data, &mut w);

    assert_eq!("".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Interpolation - Surrounding Whitespace
//   desc: Interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: '| {{string}} |'
//   expected: '| --- |'
#[test]
fn test_spec_interpolation_surrounding_whitespace_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("| {{string}} |", &data, &mut w);

    assert_eq!("| --- |".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache - Surrounding Whitespace
//   desc: Interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: '| {{{string}}} |'
//   expected: '| --- |'
#[test]
fn test_spec_interpolation_surrounding_whitespace_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("| {{{string}}} |", &data, &mut w);

    assert_eq!("| --- |".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand - Surrounding Whitespace
//   desc: Interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: '| {{&string}} |'
//   expected: '| --- |'
#[test]
fn test_spec_interpolation_surrounding_whitespace_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("| {{&string}} |", &data, &mut w);

    assert_eq!("| --- |".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Interpolation - Standalone
//   desc: Standalone interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: "  {{string}}\n"
//   expected: "  ---\n"
#[test]
fn test_spec_interpolation_standalone_basic() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("  {{string}}\n", &data, &mut w);

    assert_eq!("  ---\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache - Standalone
//   desc: Standalone interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: "  {{{string}}}\n"
//   expected: "  ---\n"
#[test]
fn test_spec_interpolation_standalone_triple_mustache() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("  {{{string}}}\n", &data, &mut w);

    assert_eq!("  ---\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand - Standalone
//   desc: Standalone interpolation should not alter surrounding whitespace.
//   data: { string: '---' }
//   template: "  {{&string}}\n"
//   expected: "  ---\n"
#[test]
fn test_spec_interpolation_standalone_ampersand() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("  {{&string}}\n", &data, &mut w);

    assert_eq!("  ---\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Interpolation With Padding
//   desc: Superfluous in-tag whitespace should be ignored.
//   data: { string: "---" }
//   template: '|{{ string }}|'
//   expected: '|---|'
#[test]
fn test_spec_interpolation_with_padding() {
  let mut w = MemWriter::new();
  let data = HashBuilder::new().insert_string("string", "---");

  rustache::render_text_from_hb("|{{ string }}|", &data, &mut w);

  assert_eq!("|---|".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Triple Mustache With Padding
//   desc: Superfluous in-tag whitespace should be ignored.
//   data: { string: "---" }
//   template: '|{{{ string }}}|'
//   expected: '|---|'
#[test]
fn test_spec_interpolation_triple_mustache_with_padding() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("|{{{ string }}|", &data, &mut w);

    assert_eq!("|---|".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

// - name: Ampersand With Padding
//   desc: Superfluous in-tag whitespace should be ignored.
//   data: { string: "---" }
//   template: '|{{& string }}|'
//   expected: '|---|'
#[test]
fn test_spec_interpolation_ampersand_with_padding() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("string", "---");

    rustache::render_text_from_hb("|{{& string }}|", &data, &mut w);

    assert_eq!("|---|".to_string(), String::from_utf8(w.unwrap()).unwrap());
}


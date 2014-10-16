extern crate rustache;

use rustache::HashBuilder;

// - name: Falsey
//   desc: Falsey sections should have their contents rendered.
//   data: { boolean: false }
//   template: '"{{^boolean}}This should be rendered.{{/boolean}}"'
//   expected: '"This should be rendered."'
#[test]
fn test_spec_inverted_falsy_bool() {
    let data = HashBuilder::new().insert_bool("boolean", false);

    let rv = rustache::render_text("{{^boolean}}This should be rendered.{{/boolean}}", data);

    assert_eq!("This should be rendered.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Truthy
//   desc: Truthy sections should have their contents omitted.
//   data: { boolean: true }
//   template: '"{{^boolean}}This should not be rendered.{{/boolean}}"'
//   expected: '""'
#[test]
fn test_spec_inverted_truthy_bool() {
    let data = HashBuilder::new().insert_bool("boolean", true);

    let rv = rustache::render_text("{{^boolean}}This should not be rendered.{{/boolean}}", data);

    assert_eq!("".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Context
//   desc: Objects and hashes should behave like truthy values.
//   data: { context: { name: 'Joe' } }
//   template: '"{{^context}}Hi {{name}}.{{/context}}"'
//   expected: '""'
#[test]
fn test_spec_inverted_truthy_hash() {
    let data = HashBuilder::new().insert_hash("context", |builder| { builder.insert_string("name", "joe") });

    let rv = rustache::render_text("{{^context}}Hi {{name}}.{{/context}}", data);

    assert_eq!("".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: List
//   desc: Lists should behave like truthy values.
//   data: { list: [ { n: 1 }, { n: 2 }, { n: 3 } ] }
//   template: '"{{^list}}{{n}}{{/list}}"'
//   expected: '""'
#[test]
fn test_spec_inverted_truthy_vec() {
    let data = HashBuilder::new().insert_vector("list", |vec| {
        vec.push_hash(|h| { 
          h.insert_string("n", "1")
           .insert_string("n", "2")
           .insert_string("n", "3")
         })
    });

    let rv = rustache::render_text("{{^list}}{{n}}{{/list}}", data);

    assert_eq!("".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Empty List
//   desc: Empty lists should behave like falsey values.
//   data: { list: [ ] }
//   template: '"{{^list}}Yay lists!{{/list}}"'
//   expected: '"Yay lists!"'
#[test]
fn test_spec_inverted_falsy_on_empty_vec() {
    let data = HashBuilder::new().insert_vector("list", |vec| {
        vec
    });

    let rv = rustache::render_text("{{^list}}Yay lists!{{/list}}", data);

    assert_eq!("Yay lists!".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Doubled
//   desc: Multiple inverted sections per template should be permitted.
//   data: { bool: false, two: 'second' }
//   template: |
//     {{^bool}}
//     * first
//     {{/bool}}
//     * {{two}}
//     {{^bool}}
//     * third
//     {{/bool}}
//   expected: |
//     * first
//     * second
//     * third
// #[test]
// fn test_spec_inverted_multiple() {
//     let data = HashBuilder::new().insert_bool("bool", false).insert_string("two", "second");

//     let rv = rustache::render_text("{{^bool}}\n* first\n{{/bool}}\n* {{two}}\n{{^bool}}\n* third\n{{/bool}}", data)

//     assert_eq!("* first\n* second\n* third".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Nested (Falsey)
//   desc: Nested falsey sections should have their contents rendered.
//   data: { bool: false }
//   template: "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |"
//   expected: "| A B C D E |"
#[test]
fn test_spec_inverted_nested_falsy() {

    let data = HashBuilder::new().insert_bool("bool", false);

    let rv = rustache::render_text("| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |", data);

    assert_eq!("| A B C D E |".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Nested (Truthy)
//   desc: Nested truthy sections should be omitted.
//   data: { bool: true }
//   template: "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |"
//   expected: "| A  E |"
#[test]
fn test_spec_inverted_nested_truthy() {
    let data = HashBuilder::new().insert_bool("bool", true);

    let rv = rustache::render_text("| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |", data);

    assert_eq!("| A  E |".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Context Misses
//   desc: Failed context lookups should be considered falsey.
//   data: { }
//   template: "[{{^missing}}Cannot find key 'missing'!{{/missing}}]"
//   expected: "[Cannot find key 'missing'!]"
#[test]
fn test_spec_inverted_missing_falsey() {
    let data = HashBuilder::new();

    let rv = rustache::render_text("[{{^missing}}Cannot find key 'missing'!{{/missing}}]", data);

    assert_eq!("[Cannot find key 'missing'!]".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Dotted Names - Truthy
//   desc: Dotted names should be valid for Inverted Section tags.
//   data: { a: { b: { c: true } } }
//   template: '"{{^a.b.c}}Not Here{{/a.b.c}}" == ""'
//   expected: '"" == ""'
// #[test]
// fn test_spec_truthy_dotted_names_valid_inverted_section_tags() {
//     let data = HashBuilder::new()
//         .insert_hash("a", |builder| {
//             builder
//                 .insert_hash("b", |builder| {
//                     builder
//                         .insert_bool("c", true)
//                 })
//         });

//     let rv = rustache::render_text("'{{^a.b.c}}Not Here{{/a.b.c}}' == ''", data)

//     assert_eq!("'' == ''".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Dotted Names - Falsey
//   desc: Dotted names should be valid for Inverted Section tags.
//   data: { a: { b: { c: false } } }
//   template: '"{{^a.b.c}}Not Here{{/a.b.c}}" == "Not Here"'
//   expected: '"Not Here" == "Not Here"'
#[test]
fn test_spec_falsey_dotted_names_valid_inverted_section_tags() {
    let data = HashBuilder::new()
        .insert_hash("a", |builder| {
            builder
                .insert_hash("b", |builder| {
                    builder
                        .insert_bool("c", false)
                })
        });

    let rv = rustache::render_text("'{{^a.b.c}}Not Here{{/a.b.c}}' == 'Not Here'", data);

    assert_eq!("'Not Here' == 'Not Here'".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Dotted Names - Broken Chains
//   desc: Dotted names that cannot be resolved should be considered falsey.
//   data: { a: { } }
//   template: '"{{^a.b.c}}Not Here{{/a.b.c}}" == "Not Here"'
//   expected: '"Not Here" == "Not Here"'

// - name: Surrounding Whitespace
//   desc: Inverted sections should not alter surrounding whitespace.
//   data: { boolean: false }
//   template: " | {{^boolean}}\t|\t{{/boolean}} | \n"
//   expected: " | \t|\t | \n"
#[test]
fn test_spec_inverted_surrounding_whitespace() {
    let data = HashBuilder::new().insert_bool("boolean", false);

    let rv = rustache::render_text(" | {{^boolean}}\t|\t{{/boolean}} | \n", data);

    assert_eq!(" | \t|\t | \n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Internal Whitespace
//   desc: Inverted should not alter internal whitespace.
//   data: { boolean: false }
//   template: " | {{^boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n"
//   expected: " |  \n  | \n"
#[test]
fn test_spec_inverted_internal_whitespace() {
    let data = HashBuilder::new().insert_bool("boolean", false);

    let rv = rustache::render_text(" | {{^boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n", data);

    assert_eq!(" |  \n  | \n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Indented Inline Sections
//   desc: Single-line sections should not alter surrounding whitespace.
//   data: { boolean: false }
//   template: " {{^boolean}}NO{{/boolean}}\n {{^boolean}}WAY{{/boolean}}\n"
//   expected: " NO\n WAY\n"
#[test]
fn test_spec_inverted_indented_inline_sections() {
    let data = HashBuilder::new().insert_bool("boolean", false);

    let rv = rustache::render_text(" {{^boolean}}NO{{/boolean}}\n {{^boolean}}WAY{{/boolean}}\n", data);

    assert_eq!(" NO\n WAY\n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Standalone Lines
//   desc: Standalone lines should be removed from the template.
//   data: { boolean: false }
//   template: |
//     | This Is
//     {{^boolean}}
//     |
//     {{/boolean}}
//     | A Line
//   expected: |
//     | This Is
//     |
//     | A Line
// #[test]
// fn test_spec_inverted_standalone_lines() {
//     let data = HashBuilder::new().insert_bool("boolean", false);

//     let rv = rustache::render_text("| This Is\n{{^boolean}}\n|\n{{/boolean}}\n| A Line", data)

//     assert_eq!("| This Is\n|\n| A Line".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Indented Lines
//   desc: Standalone indented lines should be removed from the template.
//   data: { boolean: false }
//   template: |
//     | This Is
//       {{^boolean}}
//     |
//       {{/boolean}}
//     | A Line
//   expected: |
//     | This Is
//     |
//     | A Line
// #[test]
// fn test_spec_inverted_standalone_indented_lines() {
//     let data = HashBuilder::new().insert_bool("boolean", false);

//     let rv = rustache::render_text("| This Is\n  {{^boolean}}\n|\n  {{/boolean}}\n| A Line", data)

//     assert_eq!("| This Is\n|\n| A Line".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Line Endings
//   desc: '"\r\n" should be considered a newline for standalone tags.'
//   data: { boolean: false }
//   template: "|\r\n{{^boolean}}\r\n{{/boolean}}\r\n|"
//   expected: "|\r\n|"
// #[test]
// fn test_spec_inverted_standalone_rn_is_linebreak() {
//     let data = HashBuilder::new().insert_bool("boolean", false);

//     let rv = rustache::render_text("|\r\n{{^boolean}}\r\n{{/boolean}}\r\n|", data)

//     assert_eq!("|\r\n|".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Without Previous Line
//   desc: Standalone tags should not require a newline to precede them.
//   data: { boolean: false }
//   template: "  {{^boolean}}\n^{{/boolean}}\n/"
//   expected: "^\n/"
// #[test]
// fn test_spec_inverted_standalone_without_previous_line() {
//     let data = HashBuilder::new().insert_bool("boolean", false);

//     let rv = rustache::render_text("  {{^boolean}}\n^{{/boolean}}\n/", data);

//     assert_eq!("^\n/".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Without Newline
//   desc: Standalone tags should not require a newline to follow them.
//   data: { boolean: false }
//   template: "^{{^boolean}}\n/\n  {{/boolean}}"
//   expected: "^\n/\n"
// #[test]
// fn test_spec_inverted_standalone_without_newline() {
//     let data = HashBuilder::new().insert_bool("boolean", false);

//     let rv = rustache::render_text("^{{^boolean}}\n/\n  {{/boolean}}", data);

//     assert_eq!("^\n/\n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Padding
//   desc: Superfluous in-tag whitespace should be ignored.
//   data: { boolean: false }
//   template: '|{{^ boolean }}={{/ boolean }}|'
//   expected: '|=|'
#[test]
fn test_spec_inverted_whitespace_insensitivity() {
    let data = HashBuilder::new().insert_bool("boolean", false);

    let rv = rustache::render_text("|{{^ boolean }}={{/ boolean }}|", data);

    assert_eq!("|=|".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}



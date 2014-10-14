extern crate rustache;

use rustache::HashBuilder;

// - name: Truthy
//   desc: Truthy sections should have their contents rendered.
//   data: { boolean: true }
//   template: '"{{#boolean}}This should be rendered.{{/boolean}}"'
//   expected: '"This should be rendered."'
#[test]
fn test_spec_sections_truthy_should_render_contents() {
    let data = HashBuilder::new()
        .insert_bool("boolean", true);

    let rv = rustache::render_text("{{#boolean}}This should be rendered.{{/boolean}}", data);

    assert_eq!("This should be rendered.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Falsy
//   desc: Falsy sections should have their contents omitted.
//   data: { boolean: false }
//   template: '"{{#boolean}}This should not be rendered.{{/boolean}}"'
//   expected: '""'
#[test]
fn test_spec_sections_falsy_should_not_render_contents() {
    let data = HashBuilder::new()
        .insert_bool("boolean", false);

    let rv = rustache::render_text("{{#boolean}}This should not be rendered.{{/boolean}}", data);

    assert_eq!("".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Context
//   desc: Objects and hashes should be pushed onto the context stack.
//   data: { context: { name: 'Joe' } }
//   template: '"{{#context}}Hi {{name}}.{{/context}}"'
//   expected: '"Hi Joe."'
#[test]
fn test_spec_sections_objects_and_hashes_should_be_pushed_onto_context_stack() {
    let data = HashBuilder::new()
        .insert_hash("context", |builder| {
            builder
                .insert_string("name", "Joe")
        });

    let rv = rustache::render_text("{{#context}}Hi {{name}}.{{/context}}", data);

    assert_eq!("Hi Joe.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Deeply Nested Contexts
//   desc: All elements on the context stack should be accessible.
//   data:
//     a: { one: 1 }
//     b: { two: 2 }
//     c: { three: 3 }
//     d: { four: 4 }
//     e: { five: 5 }
//   template: |
//     {{#a}}
//     {{one}}
//     {{#b}}
//     {{one}}{{two}}{{one}}
//     {{#c}}
//     {{one}}{{two}}{{three}}{{two}}{{one}}
//     {{#d}}
//     {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
//     {{#e}}
//     {{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}
//     {{/e}}
//     {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
//     {{/d}}
//     {{one}}{{two}}{{three}}{{two}}{{one}}
//     {{/c}}
//     {{one}}{{two}}{{one}}
//     {{/b}}
//     {{one}}
//     {{/a}}
//   expected: |
//     1
//     121
//     12321
//     1234321
//     123454321
//     1234321
//     12321
//     121
//     1
// #[test]
// fn test_spec_sections_all_elements_on_the_context_stack_should_be_accessible() {
//     let data = HashBuilder::new()
//         .insert_hash("a", |builder| {
//             builder
//                 .insert_int("one", 1)
//         })
//         .insert_hash("b", |builder| {
//             builder
//                 .insert_int("two", 2)
//         })
//         .insert_hash("c", |builder| {
//             builder
//                 .insert_int("three", 3)
//         })
//         .insert_hash("d", |builder| {
//             builder
//                 .insert_int("four", 4)
//         })
//         .insert_hash("e", |builder| {
//             builder
//                 .insert_int("five", 5)
//         });

//     let rv = rustache::render_text("{{#a}}
//                            {{one}}
//                            {{#b}}
//                            {{one}}{{two}}{{one}}
//                            {{#c}}
//                            {{one}}{{two}}{{three}}{{two}}{{one}}
//                            {{#d}}
//                            {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
//                            {{#e}}
//                            {{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}
//                            {{/e}}
//                            {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
//                            {{/d}}
//                            {{one}}{{two}}{{three}}{{two}}{{one}}
//                            {{/c}}
//                            {{one}}{{two}}{{one}}
//                            {{/b}}
//                            {{one}}
//                            {{/a}}",
//                            &data,
//                            &mut w
//                         );
//     assert_eq!("1
//                 121
//                 12321
//                 1234321
//                 123454321
//                 1234321
//                 12321
//                 121
//                 1".to_string(),
//                 String::from_utf8(rv.unwrap().unwrap()).unwrap()
//               );
// }

// - name: List
//   desc: Lists should be iterated; list items should visit the context stack.
//   data: { list: [ { item: 1 }, { item: 2 }, { item: 3 } ] }
//   template: '"{{#list}}{{item}}{{/list}}"'
//   expected: '"123"'
#[test]
fn test_spec_sections_list_items_are_iterated() {
    let data = HashBuilder::new()
        .insert_vector("list", |builder| {
            builder
                .push_hash(|builder| {
                    builder
                        .insert_string("item".to_string(), "1".to_string())
                })
                .push_hash(|builder| {
                    builder
                        .insert_string("item".to_string(), "2".to_string())
                })
                .push_hash(|builder| {
                    builder
                        .insert_string("item".to_string(), "3".to_string())
                })
        });

    let rv = rustache::render_text("{{#list}}{{item}}{{/list}}", data);

    assert_eq!("123".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Empty List
//     desc: Empty lists should behave like falsy values.
//     data: { list: [ ] }
//     template: '"{{#list}}Yay lists!{{/list}}"'
//     expected: '""'
#[test]
fn test_spec_sections_empty_lists_behave_like_falsy_values() {
    let data = HashBuilder::new()
        .insert_vector("list", |builder| {
            builder
        });

    let rv = rustache::render_text("{{#list}}Yay lists!{{/list}}", data);

    assert_eq!("".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Doubled
//     desc: Multiple sections per template should be permitted.
//     data: { bool: true, two: 'second' }
//     template: |
//       {{#bool}}
//       * first
//       {{/bool}}
//       * {{two}}
//       {{#bool}}
//       * third
//       {{/bool}}
//     expected: |
//       * first
//       * second
//       * third
// #[test]
// fn test_spec_sections_multiple_per_template_permitted() {
//     let data = HashBuilder::new()
//         .insert_bool("bool", true)
//         .insert_string("two", "second");

//     let rv = rustache::render_text("{{#bool}}
//                            * first
//                            {{/bool}}
//                            * {{two}}
//                            {{#bool}}
//                            * third
//                            {{/bool}}",
//                            &data,
//                            &mut w
//                          );

//     assert_eq!("* first
//                 * second
//                 * third".to_string(),
//                 String::from_utf8(rv.unwrap().unwrap()).unwrap()
//               );
// }

//   - name: Nested (Truthy)
//     desc: Nested truthy sections should have their contents rendered.
//     data: { bool: true }
//     template: "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |"
//     expected: "| A B C D E |"
#[test]
fn test_spec_sections_nested_truthy_contents_render() {
    let data = HashBuilder::new()
        .insert_bool("bool", true);

    let rv = rustache::render_text("| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |", data);

    assert_eq!("| A B C D E |".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Nested (Falsy)
//     desc: Nested falsy sections should be omitted.
//     data: { bool: false }
//     template: "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |"
//     expected: "| A  E |"
#[test]
fn test_spec_sections_nested_falsy_contents_do_not_render() {
    let data = HashBuilder::new()
        .insert_bool("bool", false);

    let rv = rustache::render_text("| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |", data);

    assert_eq!("| A  E |".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Context Misses
//     desc: Failed context lookups should be considered falsy.
//     data: { }
//     template: "[{{#missing}}Found key 'missing'!{{/missing}}]"
//     expected: "[]"
#[test]
fn test_spec_sections_failed_context_lookups_are_falsy() {
    let data = HashBuilder::new();

    let rv = rustache::render_text("[{{#missing}}Found key 'missing'!{{/missing}}]", data);

    assert_eq!("[]".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Implicit Iterator - String
//     desc: Implicit iterators should directly interpolate strings.
//     data:
//       list: [ 'a', 'b', 'c', 'd', 'e' ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(a)(b)(c)(d)(e)"'
// #[test]
// fn test_spec_sections_implicit_iterators_directly_interpolate_strings() {
//     let data = HashBuilder::new()
//         .insert_vector("list", |builder| {
//             builder
//                 .push_string("a")
//                 .push_string("b")
//                 .push_string("c")
//                 .push_string("d")
//                 .push_string("e")
//         });

// let rv = rustache::render_text("{{#list}}({{.}}){{/list}}", data);

//     assert_eq!("(a)(b)(c)(d)(e)".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Implicit Iterator - Integer
//     desc: Implicit iterators should cast integers to strings and interpolate.
//     data:
//       list: [ 1, 2, 3, 4, 5 ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(1)(2)(3)(4)(5)"'
// #[test]
// fn test_spec_sections_implicit_iterators_directly_interpolate_integers() {
//     let data = HashBuilder::new()
//         .insert_vector("list", |builder| {
//             builder
//                 .push_int(1)
//                 .push_int(2)
//                 .push_int(3)
//                 .push_int(4)
//                 .push_int(5)
//         });

//     let rv = rustache::render_text("{{#list}}({{.}}){{/list}}", data);

//     assert_eq!("(1)(2)(3)(4)(5)".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Implicit Iterator - Decimal
//     desc: Implicit iterators should cast decimals to strings and interpolate.
//     data:
//       list: [ 1.10, 2.20, 3.30, 4.40, 5.50 ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(1.1)(2.2)(3.3)(4.4)(5.5)"'
// #[test]
// fn test_spec_sections_implicit_iterators_directly_interpolate_floats() {
//     let data = HashBuilder::new()
//         .insert_vector("list", |builder| {
//             builder
//                 .push_float(1.10)
//                 .push_float(2.20)
//                 .push_float(3.30)
//                 .push_float(4.40)
//                 .push_float(5.50)
//         });

//     let rv = rustache::render_text("{{#list}}({{.}}){{/list}}", data);

//     assert_eq!("(1.1)(2.2)(3.3)(4.4)(5.5)".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Dotted Names - Truthy
//     desc: Dotted names should be valid for Section tags.
//     data: { a: { b: { c: true } } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == "Here"'
//     expected: '"Here" == "Here"'
// #[test]
// fn test_spec_sections_truthy_dotted_names_are_valid_section_tags() {
//     let data = HashBuilder::new()
//         .insert_hash("a", |builder| {
//             builder
//                 .insert_hash("b", |builder| {
//                     builder
//                         .insert_bool("c", true)
//             })
//         });

//     let rv = rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == 'Here'", data);

//     assert_eq!("'Here' == 'Here'".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Dotted Names - Falsy
//     desc: Dotted names should be valid for Section tags.
//     data: { a: { b: { c: false } } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == ""'
//     expected: '"" == ""'
#[test]
fn test_spec_sections_falsy_dotted_names_are_not_valid_section_tags() {
    let data = HashBuilder::new()
        .insert_hash("a", |builder| {
            builder
                .insert_hash("b", |builder| {
                    builder
                        .insert_bool("c", false)
            })
        });

    let rv = rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == ''", data);

    assert_eq!("'' == ''".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Dotted Names - Broken Chains
//     desc: Dotted names that cannot be resolved should be considered falsy.
//     data: { a: { } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == ""'
//     expected: '"" == ""'
#[test]
fn test_spec_sections_unresolved_dotted_names_are_not_valid_section_tags() {
    let data = HashBuilder::new()
        .insert_hash("a", |builder| {
            builder
        });

    let rv = rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == ''", data);

    assert_eq!("'' == ''".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Surrounding Whitespace
//     desc: Sections should not alter surrounding whitespace.
//     data: { boolean: true }
//     template: " | {{#boolean}}\t|\t{{/boolean}} | \n"
//     expected: " | \t|\t | \n"
#[test]
fn test_spec_sections_do_not_alter_surrounding_whitespace() {
    let data = HashBuilder::new()
        .insert_bool("boolean", true);

    let rv = rustache::render_text(" | {{#boolean}}\t|\t{{/boolean}} | \n", data);

    assert_eq!(" | \t|\t | \n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Internal Whitespace
//     desc: Sections should not alter internal whitespace.
//     data: { boolean: true }
//     template: " | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n"
//     expected: " |  \n  | \n"
#[test]
fn test_spec_sections_do_not_alter_internal_whitespace() {
    let data = HashBuilder::new()
        .insert_bool("boolean", true);

    let rv = rustache::render_text(" | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n", data);

    assert_eq!(" |  \n  | \n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Indented Inline Sections
//     desc: Single-line sections should not alter surrounding whitespace.
//     data: { boolean: true }
//     template: " {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n"
//     expected: " YES\n GOOD\n"
#[test]
fn test_spec_sections_single_line_sections_do_not_alter_surrounding_whitespace() {
    let data = HashBuilder::new()
        .insert_bool("boolean", true);

    let rv = rustache::render_text(" {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n", data);

    assert_eq!(" YES\n GOOD\n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

//   - name: Standalone Lines
//     desc: Standalone lines should be removed from the template.
//     data: { boolean: true }
//     template: |
//       | This Is
//       {{#boolean}}
//       |
//       {{/boolean}}
//       | A Line
//     expected: |
//       | This Is
//       |
//       | A Line
// #[test]
// fn test_spec_sections_standalone_lines_are_removed_from_template() {
//     let data = HashBuilder::new()
//         .insert_bool("boolean", true);

//     let rv = rustache::render_text("|
//                            | This Is
//                            {{#boolean}}
//                            |
//                            {{/boolean}}
//                            | A Line",
//                            &data,
//                            &mut w
//                          );

//     assert_eq!("|
//                 | This Is
//                 |
//                 | A Line".to_string(),
//                 String::from_utf8(rv.unwrap().unwrap()).unwrap()
//               );
// }

//   - name: Indented Standalone Lines
//     desc: Indented standalone lines should be removed from the template.
//     data: { boolean: true }
//     template: |
//       | This Is
//         {{#boolean}}
//       |
//         {{/boolean}}
//       | A Line
//     expected: |
//       | This Is
//       |
//       | A Line
// #[test]
// fn test_spec_sections_indented_standalone_lines_are_removed_from_template() {
//     let data = HashBuilder::new()
//         .insert_bool("boolean", true);

//     let rv = rustache::render_text("|
//                            | This Is
//                              {{#boolean}}
//                            |
//                              {{/boolean}}
//                            | A Line", 
//                            &data, 
//                            &mut w
//                          );

//     assert_eq!("|
//                 | This Is
//                 |
//                 | A Line".to_string(),
//                 String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Standalone Line Endings
//     desc: '"\r\n" should be considered a newline for standalone tags.'
//     data: { boolean: true }
//     template: "|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|"
//     expected: "|\r\n|"
// #[test]
// fn test_spec_sections_newline_standalone_tags() {
//     let data = HashBuilder::new()
//         .insert_bool("boolean", true);

//     let rv = rustache::render_text("|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|", data);

//     assert_eq!("|\r\n|".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Standalone Without Previous Line
//     desc: Standalone tags should not require a newline to precede them.
//     data: { boolean: true }
//     template: "  {{#boolean}}\n#{{/boolean}}\n/"
//     expected: "#\n/"
// #[test]
// fn test_spec_sections_standalone_tags_do_not_require_preceding_newline() {
//     let data = HashBuilder::new()
//         .insert_bool("boolean", true);

//     let rv = rustache::render_text("  {{#boolean}}\n#{{/boolean}}\n/", data);

//     assert_eq!("#\n/".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Standalone Without Newline
//     desc: Standalone tags should not require a newline to follow them.
//     data: { boolean: true }
//     template: "#{{#boolean}}\n/\n  {{/boolean}}"
//     expected: "#\n/\n"
// #[test]
// fn test_spec_sections_standalone_tags_do_not_require_following_newline() {
//     let data = HashBuilder::new()
//         .insert_bool("boolean", true);

//     let rv = rustache::render_text("#{{#boolean}}\n/\n  {{/boolean}}", data);

//     assert_eq!("#\n/\n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

//   - name: Padding
//     desc: Superfluous in-tag whitespace should be ignored.
//     data: { boolean: true }
//     template: '|{{# boolean }}={{/ boolean }}|'
//     expected: '|=|'
#[test]
fn test_spec_sections_superfluous_tag_whitespace_is_ignored() {
    let data = HashBuilder::new()
        .insert_bool("boolean", true);

    let rv = rustache::render_text("|{{# boolean }}={{/ boolean }}|", data);

    assert_eq!("|=|".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

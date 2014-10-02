extern crate rustache;

use std::io::MemWriter;
use rustache::HashBuilder;

/*overview: |
  Section tags and End Section tags are used in combination to wrap a section
  of the template for iteration

  These tags' content MUST be a non-whitespace character sequence NOT
  containing the current closing delimiter; each Section tag MUST be followed
  by an End Section tag with the same content within the same section.

  This tag's content names the data to replace the tag.  Name resolution is as
  follows:
    1) Split the name on periods; the first part is the name to resolve, any
    remaining parts should be retained.
    2) Walk the context stack from top to bottom, finding the first context
    that is a) a hash containing the name as a key OR b) an object responding
    to a method with the given name.
    3) If the context is a hash, the data is the value associated with the
    name.
    4) If the context is an object and the method with the given name has an
    arity of 1, the method SHOULD be called with a String containing the
    unprocessed contents of the sections; the data is the value returned.
    5) Otherwise, the data is the value returned by calling the method with
    the given name.
    6) If any name parts were retained in step 1, each should be resolved
    against a context stack containing only the result from the former
    resolution.  If any part fails resolution, the result should be considered
    falsy, and should interpolate as the empty string.
  If the data is not of a list type, it is coerced into a list as follows: if
  the data is truthy (e.g. `!!data == true`), use a single-element list
  containing the data, otherwise use an empty list.

  For each element in the data list, the element MUST be pushed onto the
  context stack, the section MUST be rendered, and the element MUST be popped
  off the context stack.

  Section and End Section tags SHOULD be treated as standalone when
  appropriate.*/

// tests:
//   - name: Truthy
//     desc: Truthy sections should have their contents rendered.
//     data: { boolean: true }
//     template: '"{{#boolean}}This should be rendered.{{/boolean}}"'
//     expected: '"This should be rendered."'

    #[test]
    fn test_spec_sections_truthy_should_render_contents() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("{{#boolean}}This should be rendered.{{/boolean}}", &data, &mut w);

        assert_eq!("This should be rendered.".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Falsy
//     desc: Falsy sections should have their contents omitted.
//     data: { boolean: false }
//     template: '"{{#boolean}}This should not be rendered.{{/boolean}}"'
//     expected: '""'

    #[test]
    fn test_spec_sections_falsy_should_not_render_contents() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", false);

        rustache::render_text("{{#boolean}}This should not be rendered.{{/boolean}}", &data, &mut w);

        assert_eq!("".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Context
//     desc: Objects and hashes should be pushed onto the context stack.
//     data: { context: { name: 'Joe' } }
//     template: '"{{#context}}Hi {{name}}.{{/context}}"'
//     expected: '"Hi Joe."'

    #[test]
    fn test_spec_sections_objects_and_hashes_should_be_pushed_onto_context_stack() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_hash("context", |builder| {
                builder
                    .insert_string("name", "Joe")
            });

        rustache::render_text("{{#context}}Hi {{name}}.{{/context}}", &data, &mut w);

        assert_eq!("Hi Joe.".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Deeply Nested Contexts
//     desc: All elements on the context stack should be accessible.
//     data:
//       a: { one: 1 }
//       b: { two: 2 }
//       c: { three: 3 }
//       d: { four: 4 }
//       e: { five: 5 }
//     template: |
      // {{#a}}
      // {{one}}
      // {{#b}}
      // {{one}}{{two}}{{one}}
      // {{#c}}
      // {{one}}{{two}}{{three}}{{two}}{{one}}
      // {{#d}}
      // {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
      // {{#e}}
      // {{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}
      // {{/e}}
      // {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
      // {{/d}}
      // {{one}}{{two}}{{three}}{{two}}{{one}}
      // {{/c}}
      // {{one}}{{two}}{{one}}
      // {{/b}}
      // {{one}}
      // {{/a}}
//     expected: |
      // 1
      // 121
      // 12321
      // 1234321
      // 123454321
      // 1234321
      // 12321
      // 121
      // 1

/*    #[test]
    fn test_spec_sections_all_elements_on_the_context_stack_should_be_accessible() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_hash("a", |builder| {
                builder
                    .insert_int("one", 1)
            })
            .insert_hash("b", |builder| {
                builder
                    .insert_int("two", 2)
            })
            .insert_hash("c", |builder| {
                builder
                    .insert_int("three", 3)
            })
            .insert_hash("d", |builder| {
                builder
                    .insert_int("four", 4)
            })
            .insert_hash("e", |builder| {
                builder
                    .insert_int("five", 5)
            });

        rustache::render_text("{{#a}}
                               {{one}}
                               {{#b}}
                               {{one}}{{two}}{{one}}
                               {{#c}}
                               {{one}}{{two}}{{three}}{{two}}{{one}}
                               {{#d}}
                               {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
                               {{#e}}
                               {{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}
                               {{/e}}
                               {{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}
                               {{/d}}
                               {{one}}{{two}}{{three}}{{two}}{{one}}
                               {{/c}}
                               {{one}}{{two}}{{one}}
                               {{/b}}
                               {{one}}
                               {{/a}}",
                               &data,
                               &mut w
                            );
        assert_eq!("1
                    121
                    12321
                    1234321
                    123454321
                    1234321
                    12321
                    121
                    1".to_string(),
                    String::from_utf8(w.unwrap()).unwrap()
                  );
    }*/

//   - name: List
//     desc: Lists should be iterated; list items should visit the context stack.
//     data: { list: [ { item: 1 }, { item: 2 }, { item: 3 } ] }
//     template: '"{{#list}}{{item}}{{/list}}"'
//     expected: '"123"'

    #[test]
    fn test_spec_sections_list_items_are_iterated() {
        let mut w = MemWriter::new();
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

        rustache::render_text("{{#list}}{{item}}{{/list}}", &data, &mut w);

        assert_eq!("123".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Empty List
//     desc: Empty lists should behave like falsy values.
//     data: { list: [ ] }
//     template: '"{{#list}}Yay lists!{{/list}}"'
//     expected: '""'

    #[test]
    fn test_spec_sections_empty_lists_behave_like_falsy_values() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_vector("list", |builder| {
                builder
            });

        rustache::render_text("{{#list}}Yay lists!{{/list}}", &data, &mut w);

        assert_eq!("".to_string(), String::from_utf8(w.unwrap()).unwrap());
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

/*    #[test]
    fn test_spec_sections_multiple_per_template_permitted() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("bool", true)
            .insert_string("two", "second");

        rustache::render_text("{{#bool}}
                               * first
                               {{/bool}}
                               * {{two}}
                               {{#bool}}
                               * third
                               {{/bool}}",
                               &data,
                               &mut w
                             );

        assert_eq!("* first
                    * second
                    * third".to_string(),
                    String::from_utf8(w.unwrap()).unwrap()
                  );
    }*/

//   - name: Nested (Truthy)
//     desc: Nested truthy sections should have their contents rendered.
//     data: { bool: true }
//     template: "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |"
//     expected: "| A B C D E |"

    #[test]
    fn test_spec_sections_nested_truthy_contents_render() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("bool", true);

        rustache::render_text("| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |", &data, &mut w);

        assert_eq!("| A B C D E |".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Nested (Falsy)
//     desc: Nested falsy sections should be omitted.
//     data: { bool: false }
//     template: "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |"
//     expected: "| A  E |"

/*    #[test]
    fn test_spec_sections_nested_falsy_contents_do_not_render() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("bool", false);

        rustache::render_text("| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |", &data, &mut w);

        assert_eq!("| A  E |".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Context Misses
//     desc: Failed context lookups should be considered falsy.
//     data: { }
//     template: "[{{#missing}}Found key 'missing'!{{/missing}}]"
//     expected: "[]"

    #[test]
    fn test_spec_sections_failed_context_lookups_are_falsy() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new();

        rustache::render_text("[{{#missing}}Found key 'missing'!{{/missing}}]", &data, &mut w);

        assert_eq!("[]".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   # Implicit Iterators

//   - name: Implicit Iterator - String
//     desc: Implicit iterators should directly interpolate strings.
//     data:
//       list: [ 'a', 'b', 'c', 'd', 'e' ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(a)(b)(c)(d)(e)"'

    // #[test]
    // fn test_spec_sections_implicit_iterators_directly_interpolate_strings() {
    //     let mut w = MemWriter::new();
    //     let data = HashBuilder::new()
    //         .insert_vector("list", |builder| {
    //             builder
    //                 .push_string("a")
    //                 .push_string("b")
    //                 .push_string("c")
    //                 .push_string("d")
    //                 .push_string("e")
    //         });

    //     rustache::render_text("{{#list}}({{.}}){{/list}}", &data, &mut w);

    //     assert_eq!("(a)(b)(c)(d)(e)".to_string(), String::from_utf8(w.unwrap()).unwrap());
    // }

//   - name: Implicit Iterator - Integer
//     desc: Implicit iterators should cast integers to strings and interpolate.
//     data:
//       list: [ 1, 2, 3, 4, 5 ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(1)(2)(3)(4)(5)"'

/*    #[test]
    fn test_spec_sections_implicit_iterators_directly_interpolate_integers() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_vector("list", |builder| {
                builder
                    .push_int(1)
                    .push_int(2)
                    .push_int(3)
                    .push_int(4)
                    .push_int(5)
            });

        rustache::render_text("{{#list}}({{.}}){{/list}}", &data, &mut w);

        assert_eq!("(1)(2)(3)(4)(5)".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Implicit Iterator - Decimal
//     desc: Implicit iterators should cast decimals to strings and interpolate.
//     data:
//       list: [ 1.10, 2.20, 3.30, 4.40, 5.50 ]
//     template: '"{{#list}}({{.}}){{/list}}"'
//     expected: '"(1.1)(2.2)(3.3)(4.4)(5.5)"'

/*    #[test]
    fn test_spec_sections_implicit_iterators_directly_interpolate_floats() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_vector("list", |builder| {
                builder
                    .push_float(1.10)
                    .push_float(2.20)
                    .push_float(3.30)
                    .push_float(4.40)
                    .push_float(5.50)
            });

        rustache::render_text("{{#list}}({{.}}){{/list}}", &data, &mut w);

        assert_eq!("(1.1)(2.2)(3.3)(4.4)(5.5)".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   # Dotted Names

//   - name: Dotted Names - Truthy
//     desc: Dotted names should be valid for Section tags.
//     data: { a: { b: { c: true } } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == "Here"'
//     expected: '"Here" == "Here"'

/*    #[test]
    fn test_spec_sections_truthy_dotted_names_are_valid_section_tags() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_hash("a", |builder| {
                builder
                    .insert_hash("b", |builder| {
                        builder
                            .insert_bool("c", true)
                })
            });

        rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == 'Here'", &data, &mut w);

        assert_eq!("'Here' == 'Here'".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Dotted Names - Falsy
//     desc: Dotted names should be valid for Section tags.
//     data: { a: { b: { c: false } } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == ""'
//     expected: '"" == ""'

    #[test]
    fn test_spec_sections_falsy_dotted_names_are_not_valid_section_tags() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_hash("a", |builder| {
                builder
                    .insert_hash("b", |builder| {
                        builder
                            .insert_bool("c", false)
                })
            });

        rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == ''", &data, &mut w);

        assert_eq!("'' == ''".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Dotted Names - Broken Chains
//     desc: Dotted names that cannot be resolved should be considered falsy.
//     data: { a: { } }
//     template: '"{{#a.b.c}}Here{{/a.b.c}}" == ""'
//     expected: '"" == ""'

    #[test]
    fn test_spec_sections_unresolved_dotted_names_are_not_valid_section_tags() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_hash("a", |builder| {
                builder
            });

        rustache::render_text("'{{#a.b.c}}Here{{/a.b.c}}' == ''", &data, &mut w);

        assert_eq!("'' == ''".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   # Whitespace Sensitivity

//   - name: Surrounding Whitespace
//     desc: Sections should not alter surrounding whitespace.
//     data: { boolean: true }
//     template: " | {{#boolean}}\t|\t{{/boolean}} | \n"
//     expected: " | \t|\t | \n"

    #[test]
    fn test_spec_sections_do_not_alter_surrounding_whitespace() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text(" | {{#boolean}}\t|\t{{/boolean}} | \n", &data, &mut w);

        assert_eq!(" | \t|\t | \n".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Internal Whitespace
//     desc: Sections should not alter internal whitespace.
//     data: { boolean: true }
//     template: " | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n"
//     expected: " |  \n  | \n"

    #[test]
    fn test_spec_sections_do_not_alter_internal_whitespace() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text(" | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n", &data, &mut w);

        assert_eq!(" |  \n  | \n".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

//   - name: Indented Inline Sections
//     desc: Single-line sections should not alter surrounding whitespace.
//     data: { boolean: true }
//     template: " {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n"
//     expected: " YES\n GOOD\n"

    #[test]
    fn test_spec_sections_single_line_sections_do_not_alter_surrounding_whitespace() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text(" {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n", &data, &mut w);

        assert_eq!(" YES\n GOOD\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
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

/*    #[test]
    fn test_spec_sections_standalone_lines_are_removed_from_template() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("|
                               | This Is
                               {{#boolean}}
                               |
                               {{/boolean}}
                               | A Line",
                               &data,
                               &mut w
                             );

        assert_eq!("|
                    | This Is
                    |
                    | A Line".to_string(),
                    String::from_utf8(w.unwrap()).unwrap()
                  );
    }*/

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

/*    #[test]
    fn test_spec_sections_indented_standalone_lines_are_removed_from_template() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("|
                               | This Is
                                 {{#boolean}}
                               |
                                 {{/boolean}}
                               | A Line", 
                               &data, 
                               &mut w
                             );

        assert_eq!("|
                    | This Is
                    |
                    | A Line".to_string(),
                    String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Standalone Line Endings
//     desc: '"\r\n" should be considered a newline for standalone tags.'
//     data: { boolean: true }
//     template: "|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|"
//     expected: "|\r\n|"

/*    #[test]
    fn test_spec_sections_newline_standalone_tags() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|", &data, &mut w);

        assert_eq!("|\r\n|".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Standalone Without Previous Line
//     desc: Standalone tags should not require a newline to precede them.
//     data: { boolean: true }
//     template: "  {{#boolean}}\n#{{/boolean}}\n/"
//     expected: "#\n/"

/*    #[test]
    fn test_spec_sections_standalone_tags_do_not_require_preceding_newline() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("  {{#boolean}}\n#{{/boolean}}\n/", &data, &mut w);

        assert_eq!("#\n/".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   - name: Standalone Without Newline
//     desc: Standalone tags should not require a newline to follow them.
//     data: { boolean: true }
//     template: "#{{#boolean}}\n/\n  {{/boolean}}"
//     expected: "#\n/\n"

/*    #[test]
    fn test_spec_sections_standalone_tags_do_not_require_following_newline() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("#{{#boolean}}\n/\n  {{/boolean}}", &data, &mut w);

        assert_eq!("#\n/\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }*/

//   # Whitespace Insensitivity

//   - name: Padding
//     desc: Superfluous in-tag whitespace should be ignored.
//     data: { boolean: true }
//     template: '|{{# boolean }}={{/ boolean }}|'
//     expected: '|=|'

    #[test]
    fn test_spec_sections_superfluous_tag_whitespace_is_ignored() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .insert_bool("boolean", true);

        rustache::render_text("|{{# boolean }}={{/ boolean }}|", &data, &mut w);

        assert_eq!("|=|".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

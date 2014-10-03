// extern crate rustache;

// use std::io::MemWriter;
// use rustache::HashBuilder;

//   - name: Basic Behavior
//     desc: The greater-than operator should expand to the named partial.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { text: 'from partial' }
//     expected: '"from partial"'
// #[test]
// fn test_spec_partials_basic_behavior() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb("\"{{>text}}\"", &data, &mut w);

//     assert_eq!("\"from partial\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Failed Lookup
//     desc: The empty string should be used when the named partial is not found.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { }
//     expected: '""'
// #[test]
// fn test_spec_partials_failed_lookup() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb("\"{{>text}}\"", &data, &mut w);

//     assert_eq!("\"\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Context
//     desc: The greater-than operator should operate within the current context.
//     data: { text: 'content' }
//     template: '"{{>partial}}"'
//     partials: { partial: '*{{text}}*' }
//     expected: '"*content*"'
// #[test]
// fn test_spec_partials_context() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("text", "content");

//     rustache::render_text_from_hb("\"{{>partial}}\"", &data, &mut w);

//     assert_eq!("\"*content*\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Recursion
//     desc: The greater-than operator should properly recurse.
//     data: { content: "X", nodes: [ { content: "Y", nodes: [] } ] }
//     template: '{{>node}}'
//     partials: { node: '{{content}}<{{#nodes}}{{>node}}{{/nodes}}>' }
//     expected: 'X<Y<>>'
// #[test]
// fn test_spec_partials_recursion() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("content", "X")
//                 .insert_vector("nodes", |v| {
//                     v.push_hash(|h| {
//                         h.insert_string("content", "Y")
//                          .insert_vector("nodes", |v| {
//                             v
//                          })
//                     })
//                 });

//     rustache::render_text_from_hb("{{>node}}", &data, &mut w);

//     assert_eq!("X<Y<>>".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Surrounding Whitespace
//     desc: The greater-than operator should not alter surrounding whitespace.
//     data: { }
//     template: '| {{>partial}} |'
//     partials: { partial: "\t|\t" }
//     expected: "| \t|\t |"
// #[test]
// fn test_spec_partials_surrounding_whitespace() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb("| {{>partial}} |", &data, &mut w);

//     assert_eq!("| \t|\t |".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Inline Indentation
//     desc: Whitespace should be left untouched.
//     data: { data: '|' }
//     template: "  {{data}}  {{> partial}}\n"
//     partials: { partial: ">\n>" }
//     expected: "  |  >\n>\n"
// #[test]
// fn test_spec_partials_inline_indentation() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("data", "|");

//     rustache::render_text_from_hb("  {{data}}  {{> partial}}\n", &data, &mut w);

//     assert_eq!("  |  >\n>\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Standalone Line Endings
//     desc: '"\r\n" should be considered a newline for standalone tags.'
//     data: { }
//     template: "|\r\n{{>partial}}\r\n|"
//     partials: { partial: ">" }
//     expected: "|\r\n>|"
// #[test]
// fn test_spec_partials_standalone_line_endings() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb("|\r\n{{>partial}}\r\n|", &data, &mut w);

//     assert_eq!("|\r\n>|".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Standalone Without Previous Line
//     desc: Standalone tags should not require a newline to precede them.
//     data: { }
//     template: "  {{>partial}}\n>"
//     partials: { partial: ">\n>"}
//     expected: "  >\n  >>"
// #[test]
// fn test_spec_partials_standalone_without_previous_line() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb("  {{>partial}}\n>", &data, &mut w);

//     assert_eq!("  >\n  >>".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Standalone Without Newline
//     desc: Standalone tags should not require a newline to follow them.
//     data: { }
//     template: ">\n  {{>partial}}"
//     partials: { partial: ">\n>" }
//     expected: ">\n  >\n  >"
// #[test]
// fn test_spec_partials_standalone_without_newline() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     rustache::render_text_from_hb(">\n  {{>partial}}", &data, &mut w);

//     assert_eq!(">\n  >\n  >".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Standalone Indentation
//     desc: Each line of the partial should be indented before rendering.
//     data: { content: "<\n->" }
//     template: |
//       \
//        {{>partial}}
//       /
//     partials:
//       partial: |
//         |
//         {{{content}}}
//         |
//     expected: |
//       \
//        |
//        <
//       ->
//        |
//       /
// #[test]
// fn test_spec_partials_standalone_indentation() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_string("content", "<\n->");

//     rustache::render_text_from_hb("|\n\\\n {{>partial}}\n/\n", &data, &mut w);

//     assert_eq!("|\n\\\n |\n <\n ->\n |\n/\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Padding Whitespace
//     desc: Superfluous in-tag whitespace should be ignored.
//     data: { boolean: true }
//     template: "|{{> partial }}|"
//     partials: { partial: "[]" }
//     expected: '|[]|'
// #[test]
// fn test_spec_partials_padding_whitespace() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new()
//                 .insert_bool("boolean", true);

//     rustache::render_text_from_hb("|{{> partial }}|", &data, &mut w);

//     assert_eq!("|[]|".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

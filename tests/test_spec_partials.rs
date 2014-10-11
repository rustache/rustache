extern crate rustache;

use std::io::MemWriter;
use rustache::HashBuilder;





// bobthecow
// it is supposed to recurse infinitely.
// bobthecow
// or rather, it's supposed to recurse until it has no reason to recurse.
// bobthecow
// your problem is that you're not rendering the partial with the correct context.
// bobthecow
// you're probably rendering it with the original context, not the section context?
// fleur__
// ohhhh.  i see now, in the data.  got it.  thanks!
// bobthecow
// no problem.
// bobthecow
// oh, make sure you render it with the full context stack, not just the section's context.








//   - name: Basic Behavior
//     desc: The greater-than operator should expand to the named partial.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { text: 'from partial' }
//     expected: '"from partial"'
#[test]
fn test_spec_partials_basic_behavior() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    let rv = rustache::render_text_from_hb("\"{{>test_data/test_spec_partials_basic}}\"", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("\"from partial\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Failed Lookup
//     desc: The empty string should be used when the named partial is not found.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { }
//     expected: '""'
#[test]
fn test_spec_partials_failed_lookup() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    let rv = rustache::render_text_from_hb("\"{{>text}}\"", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("\"\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Context
//     desc: The greater-than operator should operate within the current context.
//     data: { text: 'content' }
//     template: '"{{>partial}}"'
//     partials: { partial: '*{{text}}*' }
//     expected: '"*content*"'
#[test]
fn test_spec_partials_context() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("text", "content");

    let rv = rustache::render_text_from_hb("\"{{>test_data/test_spec_partials_context}}\"", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("\"*content*\"".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

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

//     let rv = rustache::render_text_from_hb("{{>test_data/test_spec_partials_recursion}}", &data, &mut w);
//    match rv { _ => {} }

//     assert_eq!("X<Y<>>".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Surrounding Whitespace
//     desc: The greater-than operator should not alter surrounding whitespace.
//     data: { }
//     template: '| {{>partial}} |'
//     partials: { partial: "\t|\t" }
//     expected: "| \t|\t |"
#[test]
fn test_spec_partials_surrounding_whitespace() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new();

    let rv = rustache::render_text_from_hb("| {{>test_data/test_spec_partials_whitespace}} |", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("| \t|\t |".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

//   - name: Inline Indentation
//     desc: Whitespace should be left untouched.
//     data: { data: '|' }
//     template: "  {{data}}  {{> partial}}\n"
//     partials: { partial: ">\n>" }
//     expected: "  |  >\n>\n"
#[test]
fn test_spec_partials_inline_indentation() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new().insert_string("data", "|");

    let rv = rustache::render_text_from_hb("  {{data}}  {{> test_data/test_spec_partials_inline_indentation}}\n", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("  |  >\n>\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

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

//     let rv = rustache::render_text_from_hb("|\r\n{{>partial}}\r\n|", &data, &mut w);
//    match rv { _ => {} }

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

//     let rv = rustache::render_text_from_hb("  {{>test_data/test_spec_partials_standalone_without_previous_line}}\n>", &data, &mut w);
//    match rv { _ => {} }

//     assert_eq!("  >\n  >>".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Standalone Without Newline
//     desc: Standalone tags should not require a newline to follow them.
//     data: { }
//     template: ">\n  {{>partial}}"
//     partials: { partial: ">\n>" }
//     expected: ">\n  >\n  >"
// ??? is this test really right?
// #[test]
// fn test_spec_partials_standalone_without_newline() {
//     let mut w = MemWriter::new();
//     let data = HashBuilder::new();

//     let rv = rustache::render_text_from_hb(">\n  {{>test_data/test_spec_partials_standalone_without_newline}}", &data, &mut w);
//    match rv { _ => {} }

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
//     let data = HashBuilder::new().insert_string("content", "<\n->");

//     let rv = rustache::render_text_from_hb("|\n\\\n {{>test_data/test_spec_partials_standalone_indentation}}\n/\n", &data, &mut w);
//    match rv { _ => {} }

//     assert_eq!("|\n\\\n |\n <\n ->\n |\n/\n".to_string(), String::from_utf8(w.unwrap()).unwrap());
// }

//   - name: Padding Whitespace
//     desc: Superfluous in-tag whitespace should be ignored.
//     data: { boolean: true }
//     template: "|{{> partial }}|"
//     partials: { partial: "[]" }
//     expected: '|[]|'
#[test]
fn test_spec_partials_padding_whitespace() {
    let mut w = MemWriter::new();
    let data = HashBuilder::new()
                .insert_bool("boolean", true);

    let rv = rustache::render_text_from_hb("|{{> test_data/test_spec_partials_padding_whitespace }}|", &data, &mut w);
    match rv { _ => {} }

    assert_eq!("|[]|".to_string(), String::from_utf8(w.unwrap()).unwrap());
}

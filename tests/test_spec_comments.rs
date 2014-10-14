extern crate rustache;

use rustache::HashBuilder;

// - name: Inline
//   desc: Comment blocks should be removed from the template.
//   data: { }
//   template: '12345{{! Comment Block! }}67890'
//   expected: '1234567890'
#[test]
fn test_spec_inline_comment_with_bang() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("12345{{! Comment Block! }}67890", data);

    assert_eq!("1234567890".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Multiline
//   desc: Multiline comments should be permitted.
//   data: { }
//   template: |
//     12345{{!
//       This is a
//       multi-line comment...
//     }}67890
//   expected: |
//     1234567890
#[test]
fn test_spec_multiline_comment() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("12345{{!\nThis is a\nmulti-line comment...\n}}67890", data);

    assert_eq!("1234567890".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Standalone
//   desc: All standalone comment lines should be removed.
//   data: { }
//   template: |
//     Begin.
//     {{! Comment Block! }}
//     End.
//   expected: |
//     Begin.
//     End.
#[test]
fn test_spec_standalone_comment() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("Begin.\n{{! Comment Block! }}\nEnd.", data);

    assert_eq!("Begin.\nEnd.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Indented Standalone
//   desc: All standalone comment lines should be removed.
//   data: { }
//   template: |
//     Begin.
//       {{! Indented Comment Block! }}
//     End.
//   expected: |
//     Begin.
//     End.
// #[test]
// fn test_spec_indented_standalone_comment() {
//     let data = HashBuilder::new();

//     let rv =rustache::render_text("Begin.\n\t{{! Indented Comment Block! }}\nEnd.", data);
//     assert_eq!("Begin\nEnd.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Line Endings
//   desc: '"\r\n" should be considered a newline for standalone tags.'
//   data: { }
//   template: "|\r\n{{! Standalone Comment }}\r\n|"
//   expected: "|\r\n|"
#[test]
fn test_spec_standalone_line_ending_comment() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("|\r\n{{! Standalone Comment }}\r\n|", data);

    assert_eq!("|\r\n|".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Standalone Without Previous Line
//   desc: Standalone tags should not require a newline to precede them.
//   data: { }
//   template: "  {{! I'm Still Standalone }}\n!"
//   expected: "!"
// #[test]
// fn test_spec_standalone_without_prev_line_comment() {
//     let data = HashBuilder::new();

//     let rv =rustache::render_text("  {{! I'm Still Standalone }}\n!", data);
//     assert_eq!("!".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Standalone Without Newline
//   desc: Standalone tags should not require a newline to follow them.
//   data: { }
//   template: "!\n  {{! I'm Still Standalone }}"
//   expected: "!\n"
// #[test]
// fn test_spec_standalone_without_newline_comment() {
//     let data = HashBuilder::new();

//     let rv =rustache::render_text("!\n  {{! I'm Still Standalone }}", data);
//     assert_eq!("!\n".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
// }

// - name: Multiline Standalone
//   desc: All standalone comment lines should be removed.
//   data: { }
//   template: |
//     Begin.
//     {{!
//     Something's going on here...
//     }}
//     End.
//   expected: |
//     Begin.
//     End.
#[test]
fn test_spec_multiline_standalone_comment() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("Begin.\n{{!\nSomething's going on here...\n}}\nEnd.", data);

    assert_eq!("Begin.\nEnd.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Indented Multiline Standalone
//   desc: All standalone comment lines should be removed.
//   data: { }
//   template: |
//     Begin.
//       {{!
//         Something's going on here...
//       }}
//     End.
//   expected: |
//     Begin.
//     End.
#[test]
fn test_spec_indented_multiline_standalone_comment() {
    let data = HashBuilder::new();

    let rv = rustache::render_text("Begin.\n{{!\n\tSomething's going on here...\n}}\nEnd.", data);

    assert_eq!("Begin.\nEnd.".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}

// - name: Indented Inline
//   desc: Inline comments should not strip whitespace
//   data: { }
//   template: "  12 {{! 34 }}\n"
//   expected: "  12 \n"

// - name: Surrounding Whitespace
//   desc: Comment removal should preserve surrounding whitespace.
//   data: { }
//   template: '12345 {{! Comment Block! }} 67890'
//   expected: '12345  67890'
#[test]
fn test_spec_indented_inline_comment() {
    let data = HashBuilder::new();

    let rv =rustache::render_text("12345 {{! Comment Block! }} 67890", data);

    assert_eq!("12345  67890".to_string(), String::from_utf8(rv.unwrap().unwrap()).unwrap());
}




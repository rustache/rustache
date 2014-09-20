pub struct Template;

fn render_template_with_data<W: Writer>(writer: &mut W, data: &str) {
    writer.write_str(data).unwrap();
}

// TODO: find out how to get around the limitation of traits in test-function signatures.
// this does not work.
// #[test]
// fn should_render_template() {
//     let fake_template:&str = "<div>";
//     render_test_helper(fake_template);
// }



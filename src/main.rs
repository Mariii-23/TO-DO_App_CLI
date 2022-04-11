pub mod actions;

static FILENAME: &'static str = "todo_list";

fn main() {
    actions::render_cli(FILENAME);
}

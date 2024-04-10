use crate::components::Application;

mod components;

fn main() {
    let test_source = String::from("");
    let application = Application::default();

    application.run(&test_source);
}

use crate::components::Program;

mod components;

fn main() {
    let test_source = String::from("(){}+-; and # for var <= >= \"hey\" random   // comment");
    let mut interpreter = Program::default();

    interpreter.run(&test_source);
}

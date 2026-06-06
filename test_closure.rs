use std::rc::Rc;
fn needs_fn<F: Fn() -> String>(_: F) {}
fn main() {
    let x = Rc::new(String::from("hi"));
    needs_fn(move || x.as_ref().clone());
}

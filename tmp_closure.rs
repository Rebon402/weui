use std::rc::Rc;
fn main() {
    let x = Rc::new(String::from("hi"));
    let c = move || x.as_ref().clone();
    let _ = c();
}

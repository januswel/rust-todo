extern crate todo;

#[test]
fn get_name_works() {
    let todo = todo::todo::Todo {name: "test", is_done: false};
    assert_eq!("test", todo.get_name())
}
#[test]
fn is_done_works() {
    let todo = todo::todo::Todo {name: "test", is_done: false};
    assert_eq!(false, todo.is_done())
}
#[test]
fn done_works() {
    let mut todo = todo::todo::Todo {name: "test", is_done: false};
    todo.done();
    assert_eq!(true, todo.is_done())
}
#[test]
fn undone_works() {
    let mut todo = todo::todo::Todo {name: "test", is_done: false};
    todo.done();
    todo.undone();
    assert_eq!(false, todo.is_done())
}

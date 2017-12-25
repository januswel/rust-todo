extern crate todo;

use todo::todo::Todo;

#[test]
fn get_name_works() {
    let todo = Todo::new("test");
    assert_eq!("test", todo.get_name())
}
#[test]
fn is_done_works() {
    let todo = Todo::new("test");
    assert_eq!(false, todo.is_done())
}
#[test]
fn done_works() {
    let mut todo = Todo::new("test");
    todo.done();
    assert_eq!(true, todo.is_done())
}
#[test]
fn undone_works() {
    let mut todo = Todo::new("test");
    todo.done();
    todo.undone();
    assert_eq!(false, todo.is_done())
}

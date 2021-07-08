mod simple;

#[derive(Debug)]
pub enum List {
    Elem(i32, List),
    Empty
}
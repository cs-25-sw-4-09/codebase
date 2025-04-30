
#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<StackElement<T>>,
}

#[derive(Debug)]
enum StackElement<T> {
    Value(String, T),
    Scope,
}


impl<T> Stack<T> {
    pub fn new() -> Self {
       Stack { elements: Vec::new() }
    }
}
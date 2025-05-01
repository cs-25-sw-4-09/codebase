
#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<StackElement<T>>,
}

#[derive(Debug)]
pub enum StackElement<T> {
    Value(String, T),
    Scope,
}


impl<T> Stack<T> {
    pub fn new() -> Self {
       Stack { elements: Vec::new() }
    }

    pub fn push(&mut self, identifier: String, element: T) {
        self.elements.push(StackElement::Value(identifier, element))
    }

    pub fn pop(&mut self) -> Option<StackElement<T>> {
        self.elements.pop()
    }

    pub fn push_scope(&mut self) {
        self.elements.push(StackElement::Scope)
    }
}
#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<StackElement<T>>,
}

#[derive(Debug, PartialEq)]
pub enum StackElement<T> {
    Value(String, T),
    FScope,
    VScope
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, identifier: String, element: T) {
        self.elements.push(StackElement::Value(identifier, element))
    }

    pub fn pop(&mut self) -> Option<StackElement<T>> {
        self.elements.pop()
    }

    pub fn push_vscope(&mut self) {
        self.elements.push(StackElement::VScope)
    }

    pub fn push_fscope(&mut self) {
        self.elements.push(StackElement::FScope)
    }

    pub fn pop_scope(&mut self) {
        while let Some(elem) = self.elements.pop() {
            match elem {
                StackElement::Value(_, _) => continue,
                _ => break
            }
        }
    }

    pub fn find(&mut self, identifier: String) -> Option<T> where T: Clone {
        for element in self.elements.iter() {
            match element {
                StackElement::Value(ref id, ref value) => {
                    if id == &identifier {
                        return Some(value.clone())
                    }
                },
                StackElement::VScope => { return None },
                StackElement::FScope => continue
            }
        }
        None
    }
}

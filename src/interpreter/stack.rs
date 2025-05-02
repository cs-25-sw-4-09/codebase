use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    elements: Vec<HashMap<String, T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self, identifier: String, element: T) {
        self.elements
            .iter_mut()
            .last()
            .unwrap()
            .insert(identifier, element);
    }

    pub fn pop_scope(&mut self) {
        if self.elements.len() > 1 {
            self.elements.pop();
        }
    }

    pub fn push_scope(&mut self) {
        self.elements.push(HashMap::new());
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.elements.push(HashMap::new());
    }

    pub fn find(&mut self, identifier: String) -> Option<T>
    where
        T: Clone,
    {
        for map in self.elements.iter().rev() {
            if let Some(value) = map.get(&identifier) {
                return Some(value.clone());
            }
        }
        None
    }
}

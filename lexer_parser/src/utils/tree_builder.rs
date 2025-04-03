pub struct TreeBuilderStr {
    lines: Vec<(String, usize)>
}

impl TreeBuilderStr {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn add(mut self, content: &str, indent: usize) -> Self {
        self.lines.push((content.to_string(), indent));
        self
    }

    pub fn build(self) -> String {
        self.lines.into_iter()
        .map(|(content, indentation)| format!("{}{}", "  ".repeat(indentation), content))
        .collect::<Vec<_>>()
        .join("\n")
    }

}
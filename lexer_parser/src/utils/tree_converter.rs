use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;
use super::tree_builder::TreeBuilderStr;

pub fn stringify_tree(node: AstNode) -> String {
    helper(node, 0, TreeBuilderStr::new()).build()
}

fn helper(node: AstNode, tabs: usize, tree: TreeBuilderStr) -> TreeBuilderStr {
    let tree = tree.add(
        node.get_value().map_or_else(
            || node.get_symbol().to_string(), 
            |v| v.to_string()).as_str(),
        tabs
    );
    node.children().iter().fold(tree, |tree, child| {
        helper(child, tabs + 1, tree)
    })
}


pub fn print(node: AstNode, crossings: &[bool]) {

    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{node}, {:?}", node.get_value());

    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.to_owned();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), &child_crossings);
        i += 1;
    }
}
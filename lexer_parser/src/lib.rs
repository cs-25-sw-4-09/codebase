pub mod grammar;

#[cfg(test)]
pub mod test;

pub mod programs {
    use hime_redist::ast::AstNode;
    use hime_redist::symbols::SemanticElementTrait;

    pub fn stringify_tree(node: AstNode) -> String {
        helper(node, 0).trim_end().to_string()
    }
    fn helper(node: AstNode, tabs: usize) -> String {
        //Insert tabs
        format!("{}{}\n{}",
            "  ".repeat(tabs),
            node.get_value().map_or_else(
                || node.get_symbol().to_string(), 
                |v| v.to_string()
            ),
            node.children().iter().map(|child| helper(child, tabs + 1)).collect::<String>()
        )
    }


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
            self.lines.iter()
            .map(|(content, indentation)| format!("{}{}", "  ".repeat(*indentation), content))
            .collect::<Vec<_>>()
            .join("\n")
        }

    }

    /*
    fn helper(node: AstNode, tabs: usize) -> String {
        //Insert tabs
        let mut res = "".to_string();
        if node.get_value().is_some() {
            res += &("  ".repeat(tabs - 2) + node.get_value().unwrap() + "\n"); 
        }
        res += node.children().iter().fold(
            "".to_string(), 
            |res, child| {
                    res + &helper(child, tabs + 1)
                }
            ).as_str();
        res
    } */

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

    /*fn helper2(node: AstNode, tabs: usize) -> String {
        //Insert tabs
        let mut res = "".to_string();
        if node.get_value().is_some() {
            res += &("  ".repeat(tabs) + node.get_value().unwrap() + "\n"); 
        } else {
            res += &("  ".repeat(tabs) + format!("{}", node.get_symbol()).as_str() + "\n");
        }
        res += node.children().iter().fold(
            "".to_string(), 
            |res, child| {
                    res + &helper(child, tabs + 1)
                }
            ).as_str();
        res
    }*/
    

    
    const EX1: &str = 
"width: int;
height: int;
fill: color;

begin

draw (0,0)--(width,0)--(width,height)--(0,height)--* (| fill = fill|);";

const EX2: &str = 
"import triangle \"./triangle.EXTENSION\";
import rectangle \"./rectangle.EXTENSION\";
import window \"./window.EXTENSION\";

width: int;
height: int;
color_roof: color;
color_base: color;

begin

base: shape = rectangle(|height=height, width=width, fill = color_base|);
roof: shape = triangle(|width=width, height=height-width, fill = color_roof|);

window: shape = window(|width = width/8, fill = (0,255,255,255), border = (0,0,0,255)|);
door: shape = rectangle(|height=(height-width)*2, width=width/6, fill = (133,72,54,255)|);

house: shape = place roof top base;
house = place window on house (width/2- width/4, height-width + width/2- width/4);
house = place door bottom house (width/2, door.height);

draw house;"; 

const EX3: &str = "import rectangle \"./rectangle.EXTENSION\";

square_size: int;
star_color: color;
square_amount: int;

begin

star: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);

for i in 1..square_amount { 
    cur_square: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);
    cur_square = rotate cur_square by (90 / square_amount) * i;
    star = place cur_square on star;
}

draw star;";

const EX4: &str = "import square \"./square.EXTENSION\";

scale_size: int; 
border_color: color; 
square_count: int;

begin

spiral: shape = square(|size=1, border_color=border_color|);

fib_1: int = 1;
fib_2: int = 1;

for i in 1..square_count {
    next_fib: int = fib_1 + fib_2;
    fib_1 = fib_2;
    fib_2 = next_fib;

    cur_square: shape = square(|size=next_fib, border_color=border_color|);

    fork (
        (i % 4 == 0) -> { spiral = place cur_square right spiral;}
        (i % 4 == 1) -> { spiral = place cur_square bottom spiral; }
        (i % 4 == 2) -> { spiral = place cur_square left spiral; }
        (i % 4 == 3) -> { spiral = place cur_square top spiral; }
    )
}

spiral = scale spiral by scale_size;

draw spiral;";


    pub fn get_example(idx: usize) -> &'static str {
        let arr = vec![EX1, EX2, EX3, EX4];
        arr[idx]
    }
}
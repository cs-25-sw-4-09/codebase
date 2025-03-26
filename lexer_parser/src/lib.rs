pub mod grammar;

#[cfg(test)]
pub mod test;

pub mod programs {
        
    use hime_redist::ast::AstNode;
    use hime_redist::symbols::SemanticElementTrait;

    pub fn stingify_tree(node: AstNode) -> String {
        helper(node, &mut 0)
    }

    fn helper(node: AstNode, tabs: &mut usize) -> String {
        //Insert tabs
        let mut res = "".to_string();
        let children = node.children();
        let indent = children.iter().any(|child| child.get_value().is_some());
    
        if node.get_value().is_none() {
            let children = node.children();
            if indent { *tabs += 1 }
            for child in children.iter() {
                //if child.get_value().is_some() { *tabs += 1 } else { *tabs = (*tabs).saturating_sub(1) }
                //println!("Node: {node}, val: {:?} and Tabs: {}, indent: {indent}", child.get_value(), tabs, );
                //if indent { res += &(node.to_string() + " ") }
                res += &helper(child, tabs);
            }
            if indent { *tabs -= 1 }
            return res;
        }    
        "  ".repeat(*tabs).to_string() + node.get_value().unwrap() + "\n"
    }

    
    const ex1: &str = 
"width: int;
height: int;
fill: color;

begin

draw (0,0)--(width,0)--(width,height)--(0,height)--* (| fill = fill|);";

const ex2: &str = 
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

const ex3: &str = "import rectangle \"./rectangle.EXTENSION\";

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

const ex4: &str = "import square \"./square.EXTENSION\";

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
        let arr = vec![ex1, ex2, ex3, ex4];
        arr[idx]
    }
}

#[allow(dead_code)]
pub fn get_programs() -> Vec<&'static str> {
    vec![
        PROGRAM1 , 
        PROGRAM2, 
        PROGRAM3 , 
        PROGRAM4
        ]
}

const PROGRAM1: &str = 
"width: int = 5;
height: int = 5;
fill: color = (0,0,0,0);

begin

draw (0,0)--(width,0)--(width,height)--(0,height)--* (|fill = fill|);";

const PROGRAM2: &str = 
"import triangle \"./src/typechecker/tests/test_imports/triangle.extension\";
import rectangle \"./src/typechecker/tests/test_imports/rectangle.extension\";
import window \"./src/typechecker/tests/test_imports/window.extension\";

width: int = 5;
height: int = 5;
color_roof: color = (0,0,0,0);
color_base: color = (0,0,0,0);

begin

base: shape = rectangle(|height=height, width=width, fill = color_base|);
roof: shape = triangle(|width=width, height=height-width, fill = color_roof|);

window: shape = window(|width = width/8, fill = (0,255,255,255), border = (0,0,0,255)|);
door: shape = rectangle(|height=(height-width)*2, width=width/6, fill = (133,72,54,255)|);

house: shape = place roof top base;
house = place window ontop (width/2- width/4, height-width + width/2- width/4) offset house;
house = place door bottom (width/2, door.height) offset house;

draw house;"; 

const PROGRAM3: &str = "import rectangle \"./src/typechecker/tests/test_imports/rectangle.extension\";

square_size: int = 5;
star_color: color = (0,0,0,0);
square_amount: int = 5;

begin

star: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);

for i in 1 to square_amount { 
    cur_square: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);
    cur_square = rotate cur_square by (90 / square_amount) * i;
    star = place cur_square ontop star;
}

draw star;";

const PROGRAM4: &str = "import square \"./src/typechecker/tests/test_imports/square.extension\";

scale_size: int = 5; 
border_color: color = (0,0,0,0); 
square_count: int = 5;

begin

spiral: shape = square(|size=1, border_color=border_color|);

fib_1: int = 1;
fib_2: int = 1;

for i in 1 to square_count {
    next_fib: int = fib_1 + fib_2;
    fib_1 = fib_2;
    fib_2 = next_fib;

    cur_square: shape = square(|size=next_fib, border_color=border_color|);

    fork {
        (i % 4 == 0) -> { spiral = place cur_square right spiral;}
        (i % 4 == 1) -> { spiral = place cur_square bottom spiral; }
        (i % 4 == 2) -> { spiral = place cur_square left spiral; }
        (i % 4 == 3) -> { spiral = place cur_square top spiral; }
    }
}

spiral = scale spiral by scale_size;

draw spiral;";



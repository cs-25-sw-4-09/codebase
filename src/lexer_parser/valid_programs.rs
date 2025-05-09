
pub fn get_programs() -> Vec<&'static str> {
    vec![
        PROGRAM1, 
        PROGRAM2, 
        PROGRAM3, 
        PROGRAM4,
        ]
}


pub fn get_programs2() -> Vec<&'static str> {
    vec![
        EXPR_VALUE, 
        EXPR_ARRAY,
        EXPR_PATH,
        EXPR_POLY,
        EXPR_POINT, 
        EXPR_SCALL,
        EXPR_SCALL_WITHFEILDS,
        EXPR_PLACE,
        EXPR_SCALE,
        EXPR_ROTATE,
        EXPR_WILD,
        EXPR_SCALL_BIG,
        ]
}

const EXPR_WILD: &str ="
begin
_ = scale place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20;
_ = place scale place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20 top scale place  (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20;
_ = rotate place scale place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20 top scale place  (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20 by 123123;
_ = place x(||) right rotate place scale place  (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20 top scale place  (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (x,0)--(x,x)--x--(0,0)(|x = x|) by 20 by 123123;

";

const EXPR_SCALL: &str ="
begin
_ = (x,0)--(x,x)--x--(0,0)(||);
_ = (x,0)--(x,x)--x--(0,0)--*(||);
_ = testName(||);
";


const EXPR_SCALL_BIG: &str ="
begin
_ = (x,0)--(x,x)--x--(0,0)(|x = x,x=5|);
_ = (x,0)--(x,x)--x--(0,0)--*(|x = (1,1,1,1),x =512, x=123123|);
_ = testName(|x = [1]|);
_ = (x,0)--(x,x)--x--(0,0)(|x = 12+23+(-123)|);
_ = (x,0)--(x,x)--x--(0,0)--*(|x = (1,1,1,1)|);
_ = testName(|x = [1]|);
";
const EXPR_SCALL_WITHFEILDS: &str ="
begin
_ = (x,0)--(x,x)--x--(0,0)(|x = x|);
_ = (x,0)--(x,x)--x--(0,0)--*(|x = (1,1,1,1)|);
_ = testName(|x = [1]|);
_ = (x,0)--(x,x)--x--(0,0)(|x = 12+23+(-123)|);
_ = (x,0)--(x,x)--x--(0,0)--*(|x = (1,1,1,1)|);
_ = testName(|x = [1]|);
";

const EXPR_SCALE: &str = "
begin
_ = scale (x,0)--(x,x)--x--(0,0)(|x = x|) by x;
_ = scale x by x;
_ = scale (x,0)--(x,x)--x--(0,0)(|x = x|) by 1;
_ = scale x by 1;
_ = scale (x,0)--(x,x)--x--(0,0)(|x = x|) by 1.2;
_ = scale x by 1.2;
_ = scale (x,0)--(x,x)--x--(0,0)(|x = x|) by -1.2;
_ = scale x by -1.2;
";

const EXPR_ROTATE: &str = "
begin
_ = rotate (x,0)--(x,x)--x--(0,0)(|x = x|) by x;
_ = rotate x by x;
_ = rotate (x,0)--(x,x)--x--(0,0)(|x = x|) by 1;
_ = rotate x by 1;
_ = rotate (x,0)--(x,x)--x--(0,0)(|x = x|) by 1.2;
_ = rotate x by 1.2;
_ = rotate (x,0)--(x,x)--x--(0,0)(|x = x|) by -1.2;
_ = rotate x by -1.2;
";

const EXPR_PLACE: &str ="
begin
_ = place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (0,0) offset (x,0)--(x,x)--x--(0,0)(|x = x|);
_ = place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (0,0) offset (x,0)--(x,x)--x--(0,0)(|x = x|);
_ = place x ontop (0,0) offset (x,0)--(x,x)--x--(0,0)(|x = x|);
_ = place (x,0)--(x,x)--x--(0,0)(|x = x|) ontop (0,0) offset x;
_ = place x ontop (0,0) offset x;
_ = place x ontop (0,0) offset x;
";

const EXPR_ARRAY: &str ="
begin
_ = [];
_ = [1];
_ = [1,2];
_ = [x,2];

";

const EXPR_VALUE: &str ="
begin
x:int = (5 + 5) * 6 / 7 % 20 + 7 - 8;
x:int = 6 * (5 % 20 + 10) / 7;
x: bool = (true || false) && (5 < 6) || (((5 + 9) == 0) && (true)) || (false) || (4 <= 4) && (5 > 1) && (4 >= 7);
x: bool = x[2];
x: bool = x.x;
x: bool = x;
";

const EXPR_PATH: &str ="
begin
_ = (0,0);
x:path = x--y;
x:path = (x,0)--(x,x)--x--(0,0);
";

const EXPR_POLY: &str ="
begin
x:polygon = (0,0)--(0,0)--*;
x:polygon = x--y--*;
x:polygon = (x,0)--(x,x)--x--*;
";

const EXPR_POINT: &str ="
begin
x:point = ((5 + 5) * 6 / 7 % 20 + 7 - 8,(5 + 5) * 6 / 7 % 20 + 7 - 8);
x:point = (x,5);
x:point = (x,x);
x:point = (5,x);
x:point = ((x),x);
";

const PROGRAM1: &str = 
"width: int;
height: int;
fill: color;

begin

draw (0,0)--(width,0)--(width,height)--(0,height)--* (|fill = fill|);";

const PROGRAM2: &str = 
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
house = place window ontop (width/2- width/4, height-width + width/2- width/4) offset house;
house = place door bottom (width/2, door.height) offset house;

draw house;"; 

const PROGRAM3: &str = "import rectangle \"./rectangle.EXTENSION\";

square_size: int;
star_color: color;
square_amount: int;

begin

star: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);

for i in 1 to square_amount { 
    cur_square: shape = rectangle(|height=square_size, width=square_size, fill=star_color|);
    cur_square = rotate cur_square by (90 / square_amount) * i;
    star = place cur_square ontop star;
}

draw star;";

const PROGRAM4: &str = "import square \"./square.EXTENSION\";

scale_size: int; 
border_color: color; 
square_count: int;

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



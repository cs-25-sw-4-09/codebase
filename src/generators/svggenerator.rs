use std::{error::Error, fs::File, io::Write};

use crate::interpreter::{
    data_types::{figure::Figure, figurearray::FigureArray},
    value::Value,
};

use super::{
    errors,
    generator::Generator,
};



impl Generator for SvgGenerator {

    fn generate(&mut self, draw_array: &FigureArray, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(format!("{}.svg", file_name)).unwrap();
        
        self.calc_viewbox(draw_array)?;
        self.calc_paths(draw_array)?;

        writeln!(file, "{}", self.svg_string().as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct SvgGenerator {
    view_box: String,
    paths: Vec<String>
}

impl SvgGenerator {
    pub fn new() -> Self {
        Self {
            view_box: String::new(),
            paths: Vec::new()
        }
    }

    pub fn calc_viewbox(&mut self, draw_array: &FigureArray) -> Result<(), Box<dyn Error>> {
        let mut x_min = f64::MAX;
        let mut y_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_max = f64::MIN;
        let mut line_thickness_max = 1;

        for fig in draw_array.get_figures().iter() {
            if let Some(thickness_val) = fig.get_attributes().get("thickness") {
                if let Ok(thickness) = thickness_val.get_int() {
                    if thickness > line_thickness_max {
                        line_thickness_max = thickness;
                    }
                }
            }

            for line in fig.get_lines() {
                for point in line.get_points() {
                    let x_val = match point.get_x() {
                        crate::interpreter::value::Value::Integer(x) => *x as f64,
                        crate::interpreter::value::Value::Float(x) => *x,
                        _ => unreachable!(),
                    };

                    let y_val = match point.get_y() {
                        crate::interpreter::value::Value::Integer(y) => *y as f64,
                        crate::interpreter::value::Value::Float(y) => *y,
                        _ => unreachable!(),
                    };

                    if x_val < x_min {
                        x_min = x_val;
                    }
                    if y_val < y_min {
                        y_min = y_val;
                    }
                    if x_val > x_max {
                        x_max = x_val;
                    }
                    if y_val > y_max {
                        y_max = y_val;
                    }
                }
            }
        }


        self.view_box = format!("{} {} {} {}", 
            x_min - line_thickness_max as f64, 
            y_min - line_thickness_max as f64, 
            (x_max - x_min) + 2.0 * line_thickness_max as f64, 
            (y_max - y_min) + 2.0 * line_thickness_max as f64
        );
        Ok(())
    }

    pub fn calc_paths(&mut self, draw_array: &FigureArray) -> Result<(), Box<dyn Error>>{
        let paths = draw_array.get_figures().iter().map(|fig| figure_to_path_str(fig));
        for path in paths {
            self.paths.push(path?);
        }
        Ok(())
    }

    pub fn svg_string(&self) -> String {
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{}\">\n{}\n</svg>",
            self.view_box,
            self.paths.join("\n")
        )
    }


}


fn figure_to_path_str(fig: &Figure) -> Result<String, Box<dyn Error>> {
    let path_str = map_points_to_svg_path(fig)?;
    let attr_str = map_attributes_svg_att(fig)?;
    
    Ok( format!("<path d=\"{}\" {}/>", path_str, attr_str) )
}

fn map_points_to_svg_path(
    fig: &Figure
) -> Result<String, Box<dyn Error>> {
    let line = fig.get_lines().first().ok_or_else(|| Box::new(errors::NoLines))?;

    //Define String the path will be added to
    let mut path_str = String::new();
    //Define M
    path_str.push_str(
        &format!("M{}", line.get_first_point()?.svg_format())
    );
    //Define the rest
    for points in fig.get_lines().iter().map(|line| line.get_points().as_slice()) {
        path_str.push_str(
            &match points {
                [_, p2] => format!("L{}", p2.svg_format()),
                [_, p2, p3] => format!("Q{} {}", p2.svg_format(), p3.svg_format()), 
                [_, p2, p3, p4] => format!("C{} {} {}", p2.svg_format(), p3.svg_format(), p4.svg_format()),
                _ => return Err(Box::new(errors::TooManyPoints(points.len().to_string())))
            }
        );
    }
    Ok(path_str)
}


fn map_attributes_svg_att(fig: &Figure) -> Result<String, Box<dyn Error>> {
    let is_fig_closed = fig.is_closed()?;
    let attributes = fig.get_attributes().iter()
    .map(|attr| map_fig_att_to_svg_att(attr, is_fig_closed));
    
    
    let mut attr_str = String::new();
    for att in attributes {
        attr_str.push_str(att?.as_str());
    }
    Ok(attr_str)
}

fn map_fig_att_to_svg_att(
    att: (&String, &Value),
    is_fig_closed: bool,
) -> Result<String, Box<dyn Error>> {
    match att.0.as_str() {
        "fill" => match att.1 {
            Value::Color(value1, value2, value3, value4) => {
                if is_fig_closed {
                    return Ok(format!(
                        "fill=\"rgba({},{},{},{})\" ",
                        value1.get_int()?,
                        value2.get_int()?,
                        value3.get_int()?,
                        (value4.get_int()? as f64) / (255 as f64)
                    ));
                }
                Err(Box::new(errors::AttributeNotValid(att.0.into())))
            }
            _ => unreachable!(),
        },
        "thickness" => match att.1 {
            Value::Integer(value) => {
                return Ok(format!("stroke-width=\"{}\" ", value,));
            }
            _ => unreachable!(),
        },
        "stroke" => match att.1 {
            Value::Color(value1, value2, value3, value4) => {
                return Ok(format!(
                    "stroke=\"rgba({},{},{},{})\" ",
                    value1.get_int()?,
                    value2.get_int()?,
                    value3.get_int()?,
                    (value4.get_int()? as f64) / (255 as f64)
                ));
            }
            _ => unreachable!(),
        },
        attribute => Err(Box::new(errors::AttributeNotValid(attribute.into()))),
    }
}

use std::{error::Error, fs::File, io::Write};

use crate::interpreter::{
    data_types::{figure::Figure, figurearray::FigureArray},
    value::Value,
};

use super::{errors, generator::Generator};

impl Generator for SvgGenerator {
    fn generate(
        &mut self,
        mut draw_array: FigureArray,
        file_name: String,
    ) -> Result<(), Box<dyn Error>> {
        
        //Flips all y-values for the drawArray
        draw_array.flip_y();

        //Two primary algorithms: View Box Calculation Algorithm and Path Conversion Algorithm. 
        self.calc_viewbox(&draw_array)?;
        self.calc_paths(&draw_array)?;

        let mut file = File::create(format!("{}.svg", file_name))?;
        writeln!(file, "{}", self.svg_string().as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct SvgGenerator {
    view_box: String,
    paths: Vec<String>,
}

impl SvgGenerator {
    pub fn new() -> Self {
        Self {
            view_box: String::new(),
            paths: Vec::new(),
        }
    }

    pub fn calc_viewbox(&mut self, draw_array: &FigureArray) -> Result<(), Box<dyn Error>> {
        //Calc Max_Line_Width
        let line_thickness_max = draw_array.get_figures().iter().fold(1, |max_line, fig| {
            fig.get_attributes()
                .get("thickness")
                .map(|thick| thick.get_int().ok())
                .flatten()
                .unwrap_or(max_line)
                .max(max_line)
        });

        //Get all points in the form (x, y)
        let x_y_cords = draw_array
        .get_figures()
        .iter()
        .flat_map(|fig| fig.get_lines())
        .flat_map(|line| line.get_points())
        .map(|point| (point.get_x_f64(), point.get_y_f64()));

        //Calc maxX, minX, maxY, minY
        let mut x_min = f64::MAX;
        let mut y_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_max = f64::MIN;
        for (x_val, y_val) in x_y_cords {
            x_min = x_min.min(x_val);
            y_min = y_min.min(y_val);
            x_max = x_max.max(x_val);
            y_max = y_max.max(y_val);
        }

        //Format viewbox and does calculations for: 
        //viewBoxX 
        //viewBoxY
        //width
        //height
        self.view_box = format!(
            "{} {} {} {}",
            x_min - line_thickness_max as f64,
            y_min - line_thickness_max as f64,
            (x_max - x_min) + 2.0 * line_thickness_max as f64,
            (y_max - y_min) + 2.0 * line_thickness_max as f64
        );
        Ok(())
    }

    pub fn calc_paths(&mut self, draw_array: &FigureArray) -> Result<(), Box<dyn Error>> {
        
        let paths = draw_array
            .get_figures()
            .iter()
            .map(|fig| SvgGenerator::figure_to_path_str(fig));
        for path in paths {
            self.paths.push(path?);
        }
        Ok(())
    }

    pub fn svg_string(&self) -> String {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{}\">\n{}\n</svg>",
            self.view_box,
            self.paths.join("\n")
        )
    }


    fn figure_to_path_str(fig: &Figure) -> Result<String, Box<dyn Error>> {
    //linesToPath Operation
    let path_str = SvgGenerator::map_path(fig)?;
    let attr_str = SvgGenerator::map_all_attributes(fig)?;

    Ok(format!("<path d=\"{}\" {}/>", path_str, attr_str))
}

fn map_path(fig: &Figure) -> Result<String, Box<dyn Error>> {
    let line = fig
        .get_lines()
        .first()
        .ok_or_else(|| Box::new(errors::NoLines))?;

    //This will be the string all points are concatenated with, starts with call to M from design
    let mut path_str = format!("M{}", line.get_first_point()?.svg_format());
    
    let lines_points = fig
        .get_lines()
        .iter()
        .map(|line| line.get_points().as_slice());

    //addPoints function from design with the three cases illustrated (Contains error handling)
    for points in lines_points {
        path_str.push_str(
            match points {
                [_, p2] => format!("L{}", p2.svg_format()),
                [_, p2, p3] => format!("Q{} {}", p2.svg_format(), p3.svg_format()),
                [_, p2, p3, p4] => format!(
                    "C{} {} {}",
                    p2.svg_format(),
                    p3.svg_format(),
                    p4.svg_format()
                ),
                _ => return Err(Box::new(errors::TooManyPoints(points.len().to_string()))),
            }
            .as_str(),
        );
    }
    Ok(path_str)
}

fn map_all_attributes(fig: &Figure) -> Result<String, Box<dyn Error>> {
    let attributes = fig
        .get_attributes()
        .iter()
        .zip(Some(fig.is_closed()?))
        .map(|(attr, is_closed)| SvgGenerator::map_attribute(attr, is_closed));

    let mut attr_str = String::new();
    for att in attributes {
        attr_str.push_str(att?.as_str());
    }
    Ok(attr_str)
}

fn map_attribute(
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



}


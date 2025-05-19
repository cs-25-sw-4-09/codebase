use std::{error::Error, fs::File, io::Write};

use crate::interpreter::{
    data_types::{figure::Figure, figurearray::FigureArray, point::Point},
    value::Value,
};

use super::{
    errors::{self},
    generator::Generator,
};

pub struct SvgGenerator;

impl Generator for SvgGenerator {
    fn generate(&self, draw_array: &FigureArray, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(format!("{}.svg", file_name)).unwrap();

        let (x1, y1, x2, y2) = get_viewbox_coordiantes(draw_array)?;

        writeln!(
            file,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">",
            x1, y1, x2, y2
        )?;

        for fig in draw_array.get_figures() {
            writeln!(file, "{}", figure_to_path_str(fig.clone())?)?;
        }

        writeln!(file, "</svg>").map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn get_viewbox_coordiantes(
    draw_array: &FigureArray,
) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let mut x_min = f64::INFINITY;
    let mut y_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    let mut line_thickness_max = 1;

    for mut fig in draw_array.get_figures().clone() {
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

    Ok((
        x_min - line_thickness_max as f64,
        y_min - line_thickness_max as f64,
        (x_max - x_min) + 2.0 * line_thickness_max as f64,
        (y_max - y_min) + 2.0 * line_thickness_max as f64,
    ))
}

fn figure_to_path_str(mut fig: Figure) -> Result<String, Box<dyn Error>> {
    let is_fig_closed = fig.is_closed()?;
    //Loop lines
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
    //Define attributes
    let mut attr_str = String::new();
    for att in fig.get_attributes() {
        attr_str.push_str(&map_fig_att_to_svg_att(att, is_fig_closed)?);
    }
    Ok(
        format!("<path d=\"{}\" {}/>", path_str, attr_str)
    )
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

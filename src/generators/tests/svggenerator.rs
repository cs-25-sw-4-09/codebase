use std::{
    collections::HashMap,
    fs::{self, File},
};

use crate::{
    generators::{
        basic_c, basic_line, basic_line_with_fill, basic_line_with_stroke, basic_line_with_thickness, basic_q, generator::Generator, svggenerator::SvgGenerator
    },
    interpreter::{
        data_types::point::Point, environment::IEnvironment, errors, value::Value, InterpretE,
        InterpretS,
    },
    program::{
        expression::Expr, operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        }, program::Program, statement::Stmt, r#type::Type
    },
};

#[test]
fn straight_line() {
    let i1 = basic_line().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 3 2">
<path d="M0,0L1,0" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "straightLineTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("straightLineTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("straightLineTest.svg").expect("Failed to delete the generated SVG file");
}

#[test]
fn q_bezier() {
    let i1 = basic_q().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 12">
<path d="M0,0Q10,0 10,10" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "qLineTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("qLineTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("qLineTest.svg").expect("Failed to delete the generated SVG file");
}

#[test]
fn c_bezier() {
    let i1 = basic_c().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 22">
<path d="M0,0C10,0 10,10 10,20" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "cLineTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("cLineTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("cLineTest.svg").expect("Failed to delete the generated SVG file");
}

#[test]
fn stroke() {
    let i1 = basic_line_with_stroke().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 22">
<path d="M0,0C10,0 10,10 10,20" stroke="rgba(255,255,255,1)" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "strokeTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("strokeTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("strokeTest.svg").expect("Failed to delete the generated SVG file");
}

#[test]
fn thickness() {
    let i1 = basic_line_with_thickness().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 22">
<path d="M0,0C10,0 10,10 10,20" stroke-width="1" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "thicknessTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("thicknessTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("thicknessTest.svg").expect("Failed to delete the generated SVG file");
}

#[test]
fn fill() {
    let i1 = basic_line_with_fill().get_shape().unwrap();
    let expected_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 12">
<path d="M0,0C10,0 10,10 0,0" fill="rgba(255,255,255,1)" />
</svg>"#;

    let gen = Box::new(SvgGenerator);
    let _ = gen.generate(&i1, "fillTest".into());

    // Read the generated file
    let actual_content =
        fs::read_to_string("fillTest.svg").expect("Failed to read the generated SVG file");

    assert_eq!(
        actual_content.trim(),
        expected_content.trim(),
        "Generated SVG does not match expected output"
    );

    fs::remove_file("fillTest.svg").expect("Failed to delete the generated SVG file");
}

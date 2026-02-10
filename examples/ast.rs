fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = r#"R {\sqrt{1-{\frac {v^{2}}{c^{2}}}}}, \ R, \ R "#;

    let input = latex2mathml::replace(text)?;
    let ast = latex2mathml::latex_to_ast(&input)?;

    println!("{:#?}", ast);

    Ok(())
}

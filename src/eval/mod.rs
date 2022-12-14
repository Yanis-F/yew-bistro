use crate::eval::num_parser::NumParser;

mod num_parser;


pub fn evaluate(expr: &str, base: &str) -> anyhow::Result<String> {
    log::info!("evaluating '{}'", expr);

	let parser = NumParser::from_base(base)?;

	Ok(expr.to_string())
}

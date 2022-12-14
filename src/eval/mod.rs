pub fn evaluate(expr: &str, _base: &str) -> String {
    log::info!("evaluating '{}'", expr);

    expr.to_string()
}

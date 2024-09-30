pub fn bool_to_js_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
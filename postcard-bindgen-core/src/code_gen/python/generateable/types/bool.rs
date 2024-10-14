pub fn bool_to_python_bool(value: bool) -> &'static str {
    if value {
        "True"
    } else {
        "False"
    }
}

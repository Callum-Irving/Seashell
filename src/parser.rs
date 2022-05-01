pub fn parse_line(line: String) -> Vec<String> {
    return line.split_whitespace().map(|s| s.to_owned()).collect();
}

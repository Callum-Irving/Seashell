pub fn parse_line<'a>(line: &'a str) -> Vec<&'a str> {
    return line.split_whitespace().collect();
}

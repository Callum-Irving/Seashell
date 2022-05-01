pub fn parse_line(line: String) -> Vec<String> {
    // Split line by whitespace
    let toks: Vec<_> = line.split_whitespace().map(|s| s.to_owned()).collect();
    return toks;
}

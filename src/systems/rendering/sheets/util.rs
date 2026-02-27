pub fn read_ascii_content(string: &str, scale: (usize, usize)) -> Vec<char> {
    let (width, height) = scale;

    let mut lines: Vec<Vec<char>> = string
        .lines()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            // Pad or truncate to exact width
            chars.resize(width, ' ');
            chars
        })
        .collect();

    // Pad or truncate to exact height
    lines.resize_with(height, || vec![' '; width]);

    lines.into_iter().flatten().collect()
}
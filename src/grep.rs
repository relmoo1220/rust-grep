use crate::parse::Config;

pub fn grep(contents: &String, pattern: &String, config: &Config) -> Vec<(usize, String)> {
    let lowercase_pattern = pattern.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            // Handle case-sensitive and case-insensitive
            let contains_pattern = if config.case_insensitive {
                line.to_lowercase().contains(&lowercase_pattern)
            } else {
                line.contains(pattern)
            };

            // Handle match and invert match
            let is_match = if config.invert_match {
                !contains_pattern
            } else {
                contains_pattern
            };

            // Handle line count and line
            is_match.then(|| (index + 1, line.to_string()))
        })
        .collect()
}

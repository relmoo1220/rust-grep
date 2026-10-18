pub struct CliArgs {
    pub pattern: String, // the pattern or query
    pub paths: Vec<std::path::PathBuf>,
    pub config: Config,
}

#[derive(Default)]
pub struct Config {
    pub case_insensitive: bool,
    pub line_numbers: bool,
    pub recursive: bool,
    pub invert_match: bool,
    pub count_matches: bool,
}

impl CliArgs {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut config = Config::default();
        let mut positional = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-i" => config.case_insensitive = true,
                "-n" => config.line_numbers = true,
                "-r" => config.recursive = true,
                "-v" => config.invert_match = true,
                "-c" => config.count_matches = true,
                _ if arg.starts_with('-') => {
                    return Err(format!("unknown option: {arg}"));
                }
                _ => positional.push(arg),
            }
        }

        // Convert vec into iter
        let mut positional_iter = positional.into_iter();

        // Store the pattern and advance iter position
        let pattern = positional_iter
            .next()
            .ok_or_else(|| "missing pattern".to_string())?;

        // Store the paths
        let paths = positional_iter.map(std::path::PathBuf::from).collect();

        Ok(Self {
            pattern,
            paths,
            config,
        })
    }
}

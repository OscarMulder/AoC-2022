pub mod aoc {
    use std::{path::Path, str::FromStr};
    use anyhow::Result;

    pub fn read_one_per_line<T>(path: &Path) -> Result<Vec<T>>
    where
        T: FromStr,
    {
        Ok(std::fs::read_to_string(path)?
            .split("\n")
            .filter_map(|line| line.parse::<T>().ok())
            .collect())
    }
}
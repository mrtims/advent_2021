use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

pub fn lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let data = fs::read_to_string(filename)?;
    Ok(data.lines().map(str::to_owned).collect())
}

pub fn as_1d<T: FromStr>(filename: &str) -> Vec<T> {
    if let Ok(lines) = lines(filename) {
        lines.iter().flat_map(|x| str::parse::<T>(x).ok()).collect()
    } else {
        Vec::new()
    }
}

pub fn as_csv<T: FromStr>(filename: &str) -> Vec<T> {
    if let Ok(lines) = lines(filename) {
        let mut result = Vec::new();
        for line in lines {
            let values = line.split(',');
            for value in values {
                if let Ok(x) = str::parse::<T>(value) {
                    result.push(x);
                }
            }
        }
        result
    } else {
        Vec::new()
    }
}

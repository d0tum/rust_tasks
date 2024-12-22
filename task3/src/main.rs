use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let path = Path::new("file.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut rows: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<String> = line.split_whitespace().map(String::from).collect();
        rows.push(columns);
    }

    let num_columns = if !rows.is_empty() { rows[0].len() } else { 0 };

    let mut transposed: Vec<Vec<String>> = vec![Vec::new(); num_columns];

    for row in rows {
        for (i, value) in row.into_iter().enumerate() {
            if i < num_columns {
                transposed[i].push(value);
            }
        }
    }

    for column in transposed {
        println!("{}", column.join(" "));
    }

    Ok(())
}

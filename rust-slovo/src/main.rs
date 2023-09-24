use std::error::Error;
use std::io;

use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let n = 5;
    let file_name = "dict_ru.txt";
    let words = rust_slovo::read_words_from_file(&file_name)?;
    let grid = rust_slovo::read_grid(n)?;

    let now = Instant::now();
    let res = rust_slovo::solve(n, &grid, &words);
    let elapsed_time = now.elapsed();

    println!("Total {} words in {}us:", res.len(), elapsed_time.as_micros());
    for r in res.iter() {
        r.pretty_print(n, &grid);
        println!();
        io::stdin().read_line(&mut String::new())?;
    }

    println!("Total {} words in {}us.", res.len(), elapsed_time.as_micros());

    Ok(())
}

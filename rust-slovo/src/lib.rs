use std::hash::Hash;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::rc::Rc;

pub fn read_words_from_file(file_name: &str) -> Result<BTreeSet<Box<str>>, Box<dyn Error>> {
    let mut words: BTreeSet<Box<str>> = BTreeSet::new();
    let file = File::open(file_name)?;

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        words.insert(line.into());
    }

    Ok(words)
}

pub fn read_grid(n: usize) -> Result<Box<[Box<[char]>]>, Box<dyn Error>> {
    let mut grid: Vec<Box<[char]>> = Vec::with_capacity(n);
    let mut stdin = io::stdin().lines();
    for _ in 0..n {
        let line = stdin.next();
        match line {
            Some(Err(err)) => return Err(err.into()),
            None => return Err("EOF".into()),
            Some(Ok(v)) => {
                let chars = v.chars();
                let char_slice: Box<[char]> = chars.take(n).collect();
                if char_slice.len() < n {
                    return Err(format!("{} should contain {} symbols", v, n).into());
                }
                grid.push(char_slice);
            }
        }
    }

    let grid: Box<[Box<[char]>]> = Box::from(grid); 
    Ok(grid)
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Res {
    pub word: Rc<str>,
    pub path: Box<[Point]>,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


pub fn solve(n: usize, grid: &Box<[Box<[char]>]>, words: &BTreeSet<Box<str>>) -> Box<[Res]> {
    let mut res: Vec<Res> = Vec::new();
    let mut path: Vec<Point> = Vec::new();
    let mut s: Vec<char> = Vec::new();
    for x in 0..n {
        for y in 0..n {
            let p = Point { x: x as i32, y: y as i32 };
            search(&p, &grid, &words, &mut s, &mut path, &mut res);
        }
    }
    res.distinct_by_key(|r| { r.word.clone() });
    res.sort_by_key(|r| { -(r.word.len() as i32) });
    Box::from(res)
}

fn search(
    p: &Point,
    grid: &Box<[Box<[char]>]>, 
    words: &BTreeSet<Box<str>>,
    s: &mut Vec<char>,
    path: &mut Vec<Point>,
    res: &mut Vec<Res>,
) {
    let (x, y) = (p.x, p.y);
    if !(0..grid[0].len() as i32).contains(&x) { return }
    if !(0..grid.len() as i32).contains(&y) { return }
    let (ux, uy) = (x as usize, y as usize);

    if path.contains(&p) { return }
    path.push(p.clone());
    s.push(grid[uy][ux]);

    'body: {
        let s_string: String = s.iter().collect();
    
        match is_possible_answer(&words, &s_string) {
            IsAnswer::No => { break 'body },
            IsAnswer::Yes if s_string.len() > 1 => {
                let r = Res { word: Rc::from(s_string), path: Box::from(path.clone()) };
                res.push(r);
            },
            _ => {},
        }
    
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                let np = Point { x: (x + dx), y: (y + dy) };
                search(&np, grid, words, s, path, res);
            }
        }
    }

    s.pop();
    path.pop();
}

enum IsAnswer {
    Yes,
    No,
    Maybe,
}

fn is_possible_answer(words: &BTreeSet<Box<str>>, w: &str) -> IsAnswer {
    let ceil = words.range(Box::from(w)..).next();

    match ceil {
        None => IsAnswer::No,
        Some(v) if v.as_ref() == w => IsAnswer::Yes, // TODO
        Some(v) => if v.starts_with(w) {
            IsAnswer::Maybe
        } else {
            IsAnswer::No
        },
    }
}

trait DistinctBy<T> {
    fn distinct_by_key<F, K>(&mut self, key: F)
    where
        F: Fn(&T) -> K,
        K: Eq + Hash;
}

impl <T> DistinctBy<T> for Vec<T> {
    fn distinct_by_key<F, K>(&mut self, key: F)
    where
        F: Fn(&T) -> K,
        K: Eq + Hash
    {
        let mut hs: HashSet<K> = HashSet::new();
        let mut j: usize = 0;

        for i in 0..self.len() {
            let v = &self[i];
            let k = key(&v);
            if hs.contains(&k) { continue }
            hs.insert(k);
            self.swap(i, j);
            j += 1;
        }

        self.truncate(j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_by_works() {
        let mut v = vec![1, 2, 3, 10, 100, 4];
        v.distinct_by_key(|x| { x / 10 });
        assert_eq!(v, vec![1, 10, 100]);
    }
}

impl Res {
    pub fn pretty_print(&self, n: usize, grid: &Box<[Box<[char]>]>) {
        println!("{}", self.word);
        print!("+");
        for _ in 0..n {
            print!("--");
        }
        println!("+");
        for y in 0..n {
            print!("|");
            for x in 0..n {
                let pp = Point { x: x as i32, y: y as i32 };
                if self.path.contains(&pp) {
                    print!("{} ", grid[y][x]);
                } else {
                    print!("  ");
                }
            }
            print!("|");
            println!();
        }
        print!("+");
        for _ in 0..n {
            print!("--");
        }
        println!("+");
    }
}






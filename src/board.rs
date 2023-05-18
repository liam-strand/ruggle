use linked_hash_set::LinkedHashSet;
use pad::PadStr;
use std::fmt::Display;
use yansi::Paint;

use crate::utils::pause;
use crate::lexicon::Lexicon;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Board {
    chars: Vec<Vec<char>>,
    dim: usize,
}

fn sqrt(x: usize) -> Option<usize> {
    let fdim = (x as f64).sqrt();
    if fdim.fract() == 0.0 {
        Some(fdim as usize)
    } else {
        None
    }
}

impl Board {
    pub fn new(cs: &Vec<char>) -> Option<Self> {
        let dim = sqrt(cs.len())?;
        let mut chars = Vec::with_capacity(dim);
        let mut i = 0;
        for _ in 0..dim {
            let mut row = Vec::new();
            for _ in 0..dim {
                row.push(cs[i]);
                i += 1;
            }
            chars.push(row);
        }
        Some(Board { chars, dim })
    }

    pub fn find_words(&self, dict: &Lexicon) -> BoardResult {
        let mut visited = vec![vec![false; self.dim]; self.dim];
        let mut result = BoardResult::new();

        for i in 0..self.dim {
            for j in 0..self.dim {
                let mut w = Word {
                    text: "".to_owned(),
                    path: Vec::new(),
                    board: self
                };
                self.find_words_util(&mut visited, i, j, &mut w, dict, &mut result);
            }
        }

        result
    }

    fn find_words_util<'a>(
        &self,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        w: &mut Word<'a>,
        d: &Lexicon,
        res: &mut BoardResult<'a>,
    ) {
        visited[i][j] = true;
        w.push(self.chars[i][j], (i, j));
        if d.is_word(&w.text) && w.text.len() > 2 {
            res.add(w.clone());
        }

        #[rustfmt::skip]
        let bnds = ((
                if i == 0 { 0 } else { i - 1 },
                if i == self.dim - 1 { self.dim - 1 } else { i + 1 },
            ), (
                if j == 0 { 0 } else { j - 1 },
                if j == self.dim - 1 { self.dim - 1 } else { j + 1 },
        ));

        for r in bnds.0 .0..=bnds.0 .1 {
            for c in bnds.1 .0..=bnds.1 .1 {
                if !visited[r][c] && d.is_prefix(&w.text) {
                    self.find_words_util(visited, r, c, w, d, res);
                }
            }
        }

        w.pop();
        visited[i][j] = false;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.dim {
            for j in 0..self.dim {
                write!(f, "{}", self.chars[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Word<'a> {
    text: String,
    path: Vec<(usize, usize)>,
    board: &'a Board
}

impl<'a> Word<'a> {
    fn pop(&mut self) -> Option<(char, (usize, usize))> {
        let t = self.text.pop()?;
        let p = self.path.pop()?;
        Some((t, p))
    }

    fn push(&mut self, c: char, p: (usize, usize)) {
        self.text.push(c);
        self.path.push(p);
    }

    fn len(&self) -> usize {
        self.text.len()
    }
}

impl<'a> Display for Word<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", Paint::red(&self.text))?;
        for i in 0..self.board.dim {
            for j in 0..self.board.dim {
                if self.path.contains(&(i, j)) {
                    write!(f, "{}", Paint::bold(Paint::blue(self.board.chars[i][j])))?;
                } else {
                    write!(f, "{}", self.board.chars[i][j])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct BoardResult<'a>(LinkedHashSet<Word<'a>>);

impl<'a> BoardResult<'a> {
    fn new() -> Self {
        BoardResult(LinkedHashSet::new())
    }

    fn add(&mut self, w: Word<'a>) {
        self.0.insert(w);
    }

    pub fn slow_print(&self) {

        let mut sorted: Vec<&Word> = self.0.iter().collect();
        sorted.sort_by_key(|&e| e.len());
        sorted.reverse();

        for res in sorted {
            println!("{}", res);
            pause();
        }
    }
}


impl<'a> Display for BoardResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return writeln!(f);
        }

        let width = termsize::get().ok_or(std::fmt::Error)?.cols as usize;
        let max_width = self
            .0
            .iter()
            .map(|e| &e.text)
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        let cols: Vec<String> = self
            .0
            .iter()
            .map(|e| e.text.clone())
            .map(|s| s.pad_to_width(max_width))
            .collect();

        let colwidth = cols.first().ok_or(std::fmt::Error)?.len() + 1;

        let perline = (width - 4) / colwidth;
        for (i, col) in cols.iter().enumerate() {
            write!(f, "{} ", col)?;
            if i % perline == perline - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

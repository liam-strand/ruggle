use sequence_trie::SequenceTrie;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lexicon(SequenceTrie<char, bool>);

impl Lexicon {
    pub fn new(s: &str) -> Option<Self> {
        let f = File::open(s).ok()?;
        let lines = BufReader::new(f).lines();
        let mut trie = SequenceTrie::new();

        for line in lines {
            trie.insert(&line.ok()?.chars().collect::<Vec<_>>(), true);
        }
        Some(Lexicon(trie))
    }

    pub fn from_dfile(path: &str) -> Option<Self> {
        let rdr = BufReader::new(File::open(path).ok()?);
        ron::de::from_reader(rdr).ok()
    }

    pub fn to_dfile(&self, path: &str) {
        let wtr = BufWriter::new(File::create(path).expect("Should be able to open a file here"));
        ron::ser::to_writer(wtr, self).expect("Should be serializable");
    }

    pub fn is_word(&self, s: &str) -> bool {
        *self.0.get(&s.chars().collect::<Vec<_>>()).unwrap_or(&false)
    }

    pub fn is_prefix(&self, s: &str) -> bool {
        self.0.get_node(&s.chars().collect::<Vec<_>>()).is_some()
    }
}

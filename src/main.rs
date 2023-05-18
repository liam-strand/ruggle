use boggle::{
    board::Board,
    cli::{parse_args, Arguments},
    lexicon::Lexicon,
};
use text_io::read;

fn main() {
    match parse_args() {
        Arguments::Dump(dict, dest) => {
            let dict = Lexicon::new(&dict).expect("should be a dictionary file");
            dict.to_dfile(&dest);
        }
        Arguments::Load(lexicon) => {
            let dict = Lexicon::from_dfile(&lexicon).expect("should be ron serialized lexicon");
            run_boggle(dict);
        }
        Arguments::Dync(dictionary) => {
            let dict = Lexicon::new(&dictionary).expect("should be a dictionary file");
            run_boggle(dict);
        }
    }
}

fn run_boggle(dict: Lexicon) {
    let input: String = read!();
    let b = Board::new(&input.chars().collect()).expect("should be square chars");
    println!("{}", b);
    let result = b.find_words(&dict);
    result.slow_print();
    // println!("{}", result);
}

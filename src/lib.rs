pub mod builder_1;
pub mod builder_obj;
pub mod builder_obj_log;
pub mod builder_vec;
pub mod builder_vec_large;
pub mod builder_vec_log;
pub mod grid_constraint;
pub mod grid_constraint_solve;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate derivative;

use std::time::{Instant, Duration};
use std::collections::{HashMap, HashSet};

// pub mod text_grid;

pub const SHOW_ELAPSED_TIME: bool = false;

const NO_VALUE: u8 = 0;
const NO_VALUE_USIZE: usize = 0;
const SYMBOL_NO_VALUE: char = '·';

pub const SYMBOLS_STANDARD: &str = "123456789";
pub const SYMBOLS_EXTENDED: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const SYMBOLS_GREEK_UPPER: &str = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩαβγδεζηθικλμνξοπρστυφχψω";
pub const SYMBOLS_GREEK_LOWER: &str = "αβγδεζηθικλμνξοπρστυφχψωΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";
pub const SYMBOLS_HEARTS: &str = "🖤💙💚💛💜🤍🤎🧡";
pub const SYMBOLS_ANIMAL_FACES: &str = "🐶🐺🦊🐱🐯🐵🐷🐗🐼🐨🐮🐻🐰🐹🐭🐔";

pub fn main() {
    run_gen_unicode_symbols();
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BuildController {
    pub time_start: Instant,
    pub time: Option<Duration>,
    pub time_limit: Option<Duration>,
    pub continue_build: bool,
    pub continue_branch: bool,
    pub event_counts: HashMap<String, u32>,
    #[derivative(Debug="ignore")]
    pub filled_cell_counts: Vec<u16>,
    #[derivative(Debug="ignore")]
    pub tried_grids: HashSet<u64>,
    pub max_tried_grid_count: Option<usize>,
}

/*
#[derive(Hash)]
struct TriedGrid {
    values: Vec<(u8, u8)>,
}
*/

impl BuildController {
    pub fn new(max_tried_grid_count: Option<usize>, duration_limit: Option<Duration>) -> Self {
        Self {
            time_start: Instant::now(),
            time: None,
            time_limit: duration_limit,
            continue_build: true,
            continue_branch: true,
            event_counts: HashMap::new(),
            filled_cell_counts: vec![],
            tried_grids: HashSet::new(),
            max_tried_grid_count,
        }
    }

    pub fn log_event(&mut self, label: &str) {
        let counter = self.event_counts.entry(label.to_string()).or_insert(0);
        *counter += 1;
        self.check_time();
    }

    pub fn log_done(&mut self) {
        self.time = Some(Instant::now() - self.time_start);
    }

    pub fn log_filled_cell_count(&mut self, filled_cell_count: u16) {
        self.filled_cell_counts.push(filled_cell_count);
    }

    fn check_time(&mut self) {
        if let Some(duration_limit) = self.time_limit {
            let duration = Instant::now() - self.time_start;
            if duration > duration_limit {
                self.continue_build = false;
            }
        }
    }
}

/*
impl TriedGrid {
    fn new() -> Self {
        Self {
            values: vec![],
        }
    }
}
*/

pub fn gen_unicode_symbols(start: u32, end: u32, skip: Option<&[u32]>) {
    let skip = match skip {
        Some(skip) => skip.iter().map(|x| *x).collect::<Vec<_>>(),
        None => vec![]
    };
    let mut s = String::new();
    for code in start..=end {
        if !skip.contains(&code) {
            let c = std::char::from_u32(code).unwrap();
            s.push(c);
        }
    }
    println!("{}", s);
}

pub fn gen_unicode_symbols_from_codes(codes: &[u32]) {
    let mut s = String::new();
    for code in codes {
        let c = std::char::from_u32(*code).unwrap();
        s.push(c);
    }
    println!("{}", s);
}

pub fn gen_char_array(str: &str) -> Vec<char> {
    str.chars().collect()
}

fn run_gen_unicode_symbols() {
    // https://emojipedia.org/
    // 🐾🐕🐶🐺🦊🐩🐈🐱😸😻😼😿🐆🐅🐯🦍🐒🐵🙉🙈🙊🐖🐷🐽🐎🏇🐴🐐🐏🐑🐗🦏🐘🐼🐨🐪🐫🐄🐮🐂🐻🐃🐇🐰🐿🐹🐭🐓🐔🐣🐤🦃🐦🕊🦅🦉🦆🐧🐢🐙🦀🦐🦈🐬🐳🐋🐟🐠🐡🐍🐊🦎🐛🐜🐌🐝🐞🦋
    // Greek uppercase.
    // gen_unicode_symbols(0x391, 0x3a9, Some(&[0x3a2]));
    // Greek lowercase.
    // gen_unicode_symbols(0x3b1, 0x3c9, Some(&[0x3c2]));
    // gen_unicode_symbols_from_codes(&[0x1f5a4, 0x1f499, 0x1f49a, 0x1f49b, 0x1f49c, 0x1f90d, 0x1f90e, 0x1f9e1]);
    // println!("🐾🐕🐶🐺🦊🐩🐈🐱😸😻😼😿🐆🐅🐯🦍🐒🐵🙉🙈🙊🐖🐷🐽🐎🏇🐴🐐🐏🐑🐗🦏🐘🐼🐨🐪🐫🐄🐮🐂🐻🐃🐇🐰🐿🐹🐭🐓🐔🐣🐤🦃🐦🕊🦅🦉🦆🐧🐢🐙🦀🦐🦈🐬🐳🐋🐟🐠🐡🐍🐊🦎🐛🐜🐌🐝🐞🦋");
    // println!("{}", SYMBOLS_HEARTS);
    // SYMBOL_NO_VALUE.
    gen_unicode_symbols_from_codes(&[0x1F784]);
}
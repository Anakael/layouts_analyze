use ignore::Walk;
use std::collections::HashMap;
use std::path::Path;
use std::{collections::HashSet, error::Error};
use std::{env, fs};

fn get_files_extensions_for_project(project: &str) -> HashSet<&str> {
    match project {
        "cs" => HashSet::from(["cs", "html"]),
        _ => panic!("unknown project type"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BadPattern {
    WeekPosition(char),
    UnboundPosition(char),
    SameFinger(char, char),
}

trait Layout {
    fn find_bad_patterns(&self, word: &str) -> Vec<BadPattern> {
        let mut result = Vec::new();
        let word_bytes = word.as_bytes();
        for i in 0..word.len() {
            let current_char = word_bytes[i] as char;
            let current_position = self.get_finger(&current_char);
            if current_position == 0 {
                continue;
            }

            if current_position == -1 || current_position == 1 {
                result.push(BadPattern::UnboundPosition(current_char))
            }

            if current_position >= 5 {
                result.push(BadPattern::WeekPosition(current_char))
            }

            if i >= word.len() - 1 {
                continue;
            }

            let next_char = word_bytes[i + 1] as char;

            if current_char == next_char {
                continue;
            }

            let next_position = self.get_finger(&next_char);
            if current_position == next_position {
                result.push(BadPattern::SameFinger(current_char, next_char));
            }
        }

        result
    }

    fn get_finger(&self, c: &char) -> i8;
    fn get_name(&self) -> &str;
}

struct ColemakLayout {
    chars_map: HashMap<char, i8>,
}

struct QwertyLayout {
    chars_map: HashMap<char, i8>,
}

struct DvorakLayout {
    chars_map: HashMap<char, i8>,
}

impl ColemakLayout {
    const CHARS_LAYOUT: [(char, i8); 62] = [
        ('1', -5), ('!', -5), ('q', -5), ('a', -5), ('z', -5),
        ('2', -4), ('@', -4), ('w', -4), ('r', -4), ('x', -4),
        ('3', -3), ('#', -3), ('f', -3), ('s', -3), ('c', -3),
        ('4', -2), ('$', -2), ('p', -2), ('t', -2), ('v', -2),
        ('5', -1), ('%', -1), ('g', -1), ('d', -1), ('b', -1),

        ('6', 1), ('^', 1), ('j', 1), ('h', 1), ('k', 1),
        ('7', 2), ('&', 2), ('l', 2), ('n', 2), ('m', 2),
        ('8', 3), ('*', 3), ('u', 3), ('e', 3), (',', 3), ('<', 3),
        ('9', 4), ('(', 4), ('y', 4), ('i', 4), ('.', 4), ('>', 3),
        ('0', 5), (')', 5), (';', 5), ('o', 5), ('/', 5),
        ('-', 6), ('_', 6), ('[', 6), ('{', 6), ('\'', 6), ('"', 6),
        ('=', 7), ('+', 7), (']', 7), ('}', 7),
    ];

    fn new() -> Self {
        ColemakLayout {
            chars_map: HashMap::from(ColemakLayout::CHARS_LAYOUT),
        }
    }
}

impl QwertyLayout {
    const CHARS_LAYOUT: [(char, i8); 62] = [
        ('1', -5), ('!', -5), ('q', -5), ('a', -5), ('z', -5),
        ('2', -4), ('@', -4), ('w', -4), ('s', -4), ('x', -4),
        ('3', -3), ('#', -3), ('e', -3), ('d', -3), ('c', -3),
        ('4', -2), ('$', -2), ('r', -2), ('f', -2), ('v', -2),
        ('5', -1), ('%', -1), ('t', -1), ('g', -1), ('b', -1),

        ('6', 1), ('^', 1), ('y', 1), ('h', 1), ('n', 1),
        ('7', 2), ('&', 2), ('u', 2), ('j', 2), ('m', 2),
        ('8', 3), ('*', 3), ('i', 3), ('k', 3), (',', 3), ('<', 3),
        ('9', 4), ('(', 4), ('o', 4), ('l', 4), ('.', 4), ('>', 3),
        ('0', 5), (')', 5), ('p', 5), (';', 5), ('/', 5),
        ('-', 6), ('_', 6), ('[', 6), ('{', 6), ('\'', 6), ('"', 6),
        ('=', 7), ('+', 7), (']', 7), ('}', 7),
    ];

    fn new() -> Self {
        QwertyLayout {
            chars_map: HashMap::from(QwertyLayout::CHARS_LAYOUT),
        }
    }
}

impl DvorakLayout {
    const CHARS_LAYOUT: [(char, i8); 62] = [
        ('1', -5), ('!', -5), ('/', -5), ('?', -5), ('a', -5), (';', -5), (':', -5),
        ('2', -4), ('@', -4), (',', -4), ('<', -4), ('o', -4), ('q', -3),
        ('3', -3), ('#', -3), ('.', -3), ('>', -3), ('e', -3), ('j', -3),
        ('4', -2), ('$', -2), ('p', -2), ('u', -2), ('k', -2),
        ('5', -1), ('%', -1), ('y', -1), ('i', -1), ('x', -1),

        ('6', 1), ('^', 1), ('f', 1), ('d', 1), ('b', 1),
        ('7', 2), ('&', 2), ('g', 2), ('h', 2), ('m', 2),
        ('8', 3), ('*', 3), ('c', 3), ('t', 3), ('w', 3),
        ('9', 4), ('(', 4), ('r', 4), ('n', 4), ('v', 4),
        ('0', 5), (')', 5), ('l', 5), ('s', 5), ('z', 5),
        ('[', 6), ('{', 6), ('-', 6), ('_', 6),
        (']', 7), ('}', 7), ('+', 7), ('=', 7),
    ];

    fn new() -> Self {
        DvorakLayout {
            chars_map: HashMap::from(DvorakLayout::CHARS_LAYOUT),
        }
    }
}

impl Layout for ColemakLayout {
    fn get_finger(&self, c: &char) -> i8 {
        *self.chars_map.get(c).unwrap_or(&0)
    }

    fn get_name(&self) -> &str {
        "colemak"
    }
}

impl Layout for QwertyLayout {
    fn get_finger(&self, c: &char) -> i8 {
        *self.chars_map.get(c).unwrap_or(&0)
    }

    fn get_name(&self) -> &str {
        "qwerty"
    }
}

impl Layout for DvorakLayout {
    fn get_finger(&self, c: &char) -> i8 {
        *self.chars_map.get(c).unwrap_or(&0)
    }

    fn get_name(&self) -> &str {
        "dvorak"
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let project_type = &args[1];
    let project_files = get_files_extensions_for_project(project_type);
    let root_dir = Path::new(&args[2]);

    let mut patterns: HashMap<String, Vec<BadPattern>> = HashMap::new();
    let mut words: HashMap<(String, String), Vec<BadPattern>> = HashMap::new();

    let colemak = Box::new(ColemakLayout::new());
    let qwerty = Box::new(QwertyLayout::new());
    let dvorak = Box::new(DvorakLayout::new());

    let layouts: [Box<dyn Layout>; 3] = [colemak, qwerty, dvorak];

    for entry in Walk::new(root_dir) {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }

        let ext = entry.path().extension();
        if ext.is_none() {
            continue;
        }

        let ext = ext.unwrap();
        if !project_files.contains(ext.to_str().unwrap()) {
            continue;
        }

        for raw_word in fs::read_to_string(entry.path())?.split_whitespace() {
            let lower_raw_word = raw_word.trim().to_lowercase();
            let word = lower_raw_word.as_str();
            if word.is_empty() || word.len() == 1 {
                continue;
            }

            for layout in &layouts {
                let layout_name = layout.get_name();
                if let Some(result) = words.get_mut(&(word.to_string(), layout_name.to_string())) {
                    if result.is_empty() {
                        continue;
                    }

                    patterns
                        .entry(layout_name.to_string())
                        .and_modify(|x| x.append(result))
                        .or_insert_with(|| result.clone());
                } else {
                    let mut result = layout.find_bad_patterns(word);
                    if result.is_empty() {
                        continue;
                    }

                    words.insert((word.to_string(), layout_name.to_string()), result.clone());
                    patterns
                        .entry(layout_name.to_string())
                        .and_modify(|x| x.append(&mut result))
                        .or_insert_with(|| result.clone());
                }
            }
        }
    }

    for (pattern, result) in patterns {
        let result_count: usize = result.len();
        println!("{} result: {} patterns", pattern, result_count);
        let week_position_count = result
            .iter()
            .filter(|x| matches!(**x, BadPattern::WeekPosition(_)))
            .count();

        let same_finger_count = result
            .iter()
            .filter(|x| matches!(**x, BadPattern::SameFinger(_, _)))
            .count();

        let unbound_position_count = result
            .iter()
            .filter(|x| matches!(**x, BadPattern::UnboundPosition(_)))
            .count();

        println!("{} result: {} WeekPosition", pattern, week_position_count);
        println!("{} result: {} SameFinger", pattern, same_finger_count);
        println!("{} result: {} UnboundPosition", pattern, unbound_position_count);
        println!();
    }

    Ok(())
}

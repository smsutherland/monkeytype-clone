const ENGLISH_1K: &str = include_str!("../data/english_1k.list");

#[derive(Debug, Clone, Copy)]
pub enum Language {
    English1k,
}

pub type Word<'a> = &'a str;

pub fn generate_string(language: Language, length: usize) -> Box<[Word<'static>]> {
    match language {
        Language::English1k => {
            generate_string_from_word_list(ENGLISH_1K.lines().collect::<Box<_>>().as_ref(), length)
        }
    }
}

pub fn generate_string_from_word_list<'a>(word_list: &[&'a str], length: usize) -> Box<[Word<'a>]> {
    let len = word_list.len();
    let mut result = Vec::with_capacity(length);
    for _ in 0..length {
        let i = fastrand::usize(..len);
        result.push(word_list[i]);
    }
    result.into()
}

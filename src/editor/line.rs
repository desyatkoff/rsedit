use std::{
    ops::Range,
    fmt,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => {
                return other.saturating_add(1);
            },
            Self::Full => {
                return other.saturating_add(2);
            },
        }
    }
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
    start_byte_index: usize,
}

#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    pub fn from(s: &str) -> Self {
        return Self {
            fragments: Self::str_to_fragments(s),
            string: String::from(s),
        };
    }

    pub fn str_to_fragments(s: &str) -> Vec<TextFragment> {
        return s
            .grapheme_indices(true)
            .map(|(byte_index, grapheme)| {
                let (replacement, rendered_width) = Self::get_char_replacement(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };

                            return (None, rendered_width);
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                );

                TextFragment {
                    grapheme: String::from(grapheme),
                    rendered_width,
                    replacement,
                    start_byte_index: byte_index,
                }
            }
        ).collect();
    }

    fn rerender_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    fn get_char_replacement(s: &str) -> Option<char> {
        let width = s.width();

        match s {
            " " => {
                return None;
            },
            "\t" => {
                return Some(' ');
            },
            _ if width > 0 && s.trim().is_empty() => {
                return Some('␣');
            },
            _ if width == 0 => {
                let mut chars = s.chars();

                if let Some(c) = chars.next() {
                    if c.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }

                return Some('·');
            }
            _ => {
                return None;
            },
        }
    }

    pub fn insert_char(&mut self, character: char, at_where: usize) {
        if let Some(fragment) = self.fragments.get(at_where) {
            self.string.insert(fragment.start_byte_index, character);
        } else {
            self.string.push(character);
        }

        self.rerender_fragments();
    }

    pub fn append_char(&mut self, character: char) {
        self.insert_char(
            character,
            self.grapheme_count()
        );
    }

    pub fn remove_char(&mut self, at_where: usize) {
        if let Some(fragment) = self.fragments.get(at_where) {
            let start = fragment.start_byte_index;
            let end = fragment
                .start_byte_index
                .saturating_add(fragment.grapheme.len());

            self.string.drain(start..end);
            self.rerender_fragments();
        }
    }

    pub fn remove_last_char(&mut self) {
        self.remove_char(self.grapheme_count().saturating_sub(1));
    }

    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.rerender_fragments();
    }

    pub fn split(&mut self, at_where: usize) -> Self {
        if let Some(fragment) = self.fragments.get(at_where) {
            let remainder = self.string.split_off(fragment.start_byte_index);

            self.rerender_fragments();

            return Self::from(&remainder);
        } else {
            return Self::default();
        }
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);

            if current_pos >= range.end {
                break;
            }

            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }

            current_pos = fragment_end;
        }

        return result;
    }

    pub fn grapheme_count(&self) -> usize {
        return self.fragments.len();
    }

    fn byte_index_to_grapheme_index(&self, byte_index: usize) -> usize {
        for (grapheme_index, fragment) in self.fragments.iter().enumerate() {
            if fragment.start_byte_index >= byte_index {
                return grapheme_index;
            }
        }

        return 0;
    }

    pub fn search(&self, query: &str) -> Option<usize> {
        return self.string
            .find(query)
            .map(
                |byte_index| {
                    return self.byte_index_to_grapheme_index(byte_index);
                }
            );
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        return self.fragments
            .iter()
            .take(grapheme_index)
            .map(
                |fragment| match fragment.rendered_width {
                    GraphemeWidth::Half => 1,
                    GraphemeWidth::Full => 2,
                }
            )
            .sum();
    }

    pub fn width(&self) -> usize {
        return self.width_until(self.grapheme_count());
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            formatter,
            "{}",
            self.string
        );
    }
}
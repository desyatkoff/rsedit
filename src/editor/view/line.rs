use std::ops::Range;
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
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(s: &str) -> Self {
        let fragments = Self::str_to_fragments(s);

        return Self {
            fragments,
        };
    }

    pub fn str_to_fragments(s: &str) -> Vec<TextFragment> {
        let fragments = s
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::char_replacement(grapheme)
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
                }
            })
            .collect();

        return fragments;
    }

    fn char_replacement(s: &str) -> Option<char> {
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
                return None
            },
        }
    }

    pub fn insert_char(&mut self, character: char, grapheme_index: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == grapheme_index {
                result.push(character);
            }

            result.push_str(&fragment.grapheme);
        }

        if grapheme_index >= self.fragments.len() {
            result.push(character);
        }

        self.fragments = Self::str_to_fragments(&result);
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
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
}


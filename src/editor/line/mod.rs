mod graphemewidth;
mod textfragment;

use std::{
    ops::{
        Deref,
        Range,
    },
    fmt::Display,
    fmt,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use graphemewidth::GraphemeWidth;
use textfragment::TextFragment;
use super::{
    AnnotatedString,
    AnnotationType,
    Column
};

type GraphemeIndex = usize;
type ByteIndex = usize;
type ColumnIndex = usize;

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        return Self {
            fragments: Self::str_to_fragments(line_str),
            string: String::from(line_str),
        };
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        return line_str
            .grapheme_indices(true)
            .map(
                |(byte_index, grapheme)| {
                    let (replacement, rendered_width) = Self::get_char_replacement(grapheme)
                        .map_or_else(
                            || {
                                let unicode_width = grapheme.width();
                                let rendered_width = match unicode_width {
                                    0 | 1 => {
                                        GraphemeWidth::Half
                                    },
                                    _ => {
                                        GraphemeWidth::Full
                                    },
                                };

                                return (
                                    None,
                                    rendered_width,
                                );
                            },
                            |replacement| {
                                return (
                                    Some(replacement),
                                    GraphemeWidth::Half
                                );
                            },
                        );

                    return TextFragment {
                        grapheme: grapheme.to_string(),
                        rendered_width,
                        replacement,
                        start_byte_index: byte_index,
                    };
                }
            )
            .collect();
    }

    fn rerender_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    fn get_char_replacement(for_str: &str) -> Option<char> {
        let width = for_str.width();

        match for_str {
            " " => {
                return None;
            },
            "\t" => {
                return Some(' ');
            },
            _ if width > 0 && for_str.trim().is_empty() => {
                return Some('␣');
            },
            _ if width == 0 => {
                let mut chars = for_str.chars();

                if let Some(c) = chars.next() {
                    if c.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }

                return Some('·');
            },
            _ => {
                return None;
            },
        }
    }

   pub fn get_visible_graphemes(&self, range: Range<ColumnIndex>) -> String {
        return self.get_annotated_visible_substr(range, None, None).to_string()
    }

    pub fn get_annotated_visible_substr(&self, range: Range<ColumnIndex>, query: Option<&str>, selected_match: Option<GraphemeIndex>) -> AnnotatedString {
        if range.start >= range.end {
            return AnnotatedString::default();
        }

        let mut result = AnnotatedString::from(&self.string);

        if let Some(query) = query {
            if !query.is_empty() {
                self.find_all(query, 0..self.string.len()).iter().for_each(
                    |(start_byte_index, grapheme_index)| {
                        if let Some(selected_match) = selected_match {
                            if *grapheme_index == selected_match {
                                result.add_annotation(
                                    AnnotationType::SelectedMatch,
                                    *start_byte_index,
                                    start_byte_index.saturating_add(query.len()),
                                );

                                return;
                            }
                        }

                        result.add_annotation(
                            AnnotationType::Match,
                            *start_byte_index,
                            start_byte_index.saturating_add(query.len()),
                        );
                    },
                );
            }
        }

        let mut fragment_start = self.width();

        for fragment in self.fragments.iter().rev() {
            let fragment_end = fragment_start;

            fragment_start = fragment_start.saturating_sub(fragment.rendered_width.into());

            if fragment_start > range.end {
                continue;
            }

            if fragment_start < range.end && fragment_end > range.end {
                result.replace(fragment.start_byte_index, self.string.len(), "⋯");

                continue;
            } else if fragment_start == range.end {
                result.replace(fragment.start_byte_index, self.string.len(), "");

                continue;
            }

            if fragment_end <= range.start {
                result.replace(
                    0,
                    fragment
                        .start_byte_index
                        .saturating_add(fragment.grapheme.len()),
                    "",
                );

                break;
            } else if fragment_start < range.start && fragment_end > range.start {
                result.replace(
                    0,
                    fragment
                        .start_byte_index
                        .saturating_add(fragment.grapheme.len()),
                    "⋯",
                );

                break;
            }

            if fragment_start >= range.start && fragment_end <= range.end {
                if let Some(replacement) = fragment.replacement {
                    let start_byte_index = fragment.start_byte_index;
                    let end_byte_index = start_byte_index.saturating_add(fragment.grapheme.len());

                    result.replace(start_byte_index, end_byte_index, &replacement.to_string());
                }
            }
        }

        return result;
    }

    pub fn grapheme_count(&self) -> GraphemeIndex {
        return self.fragments.len();
    }

    pub fn width_until(&self, grapheme_index: GraphemeIndex) -> Column {
        return self.fragments
            .iter()
            .take(grapheme_index)
            .map(
                |fragment| match fragment.rendered_width {
                    GraphemeWidth::Half => {
                        return 1;
                    },
                    GraphemeWidth::Full => {
                        return 2;
                    },
                }
            )
            .sum()
    }

    pub fn width(&self) -> Column {
        return self.width_until(self.grapheme_count());
    }

    pub fn insert_char(&mut self, character: char, at: GraphemeIndex) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_index, character);
        } else {
            self.string.push(character);
        }

        self.rerender_fragments();
    }

    pub fn append_char(&mut self, character: char) {
        self.insert_char(character, self.grapheme_count());
    }

    pub fn remove_char(&mut self, at: GraphemeIndex) {
        if let Some(fragment) = self.fragments.get(at) {
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

    pub fn split(&mut self, at: GraphemeIndex) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start_byte_index);

            self.rerender_fragments();

            return Self::from(&remainder);
        } else {
            return Self::default();
        }
    }

    fn byte_index_to_grapheme_index(&self, byte_index: ByteIndex) -> Option<GraphemeIndex> {
        if byte_index > self.string.len() {
            return None;
        }

        return self.fragments
            .iter()
            .position(
                |fragment| {
                    return fragment.start_byte_index >= byte_index;
                }
            );
    }

    fn grapheme_index_to_byte_index(&self, grapheme_index: GraphemeIndex) -> ByteIndex {
        if grapheme_index == 0 || self.grapheme_count() == 0 {
            return 0;
        }

        return self.fragments.get(grapheme_index).map_or_else(
            || {
                return 0;
            },
            |fragment| {
                return fragment.start_byte_index;
            },
        );
    }

    pub fn search_next(&self, query: &str, from_grapheme_index: GraphemeIndex) -> Option<GraphemeIndex> {
        if from_grapheme_index == self.grapheme_count() {
            return None;
        }

        let start_byte_index = self.grapheme_index_to_byte_index(from_grapheme_index);

        return self.find_all(query, start_byte_index..self.string.len())
            .first()
            .map(
                |(_, grapheme_index)| {
                    return *grapheme_index;
                }
            );
    }

    pub fn search_previous(&self, query: &str, from_grapheme_index: GraphemeIndex) -> Option<GraphemeIndex> {
        if from_grapheme_index == 0 {
            return None;
        }

        let end_byte_index = if from_grapheme_index == self.grapheme_count() {
            self.string.len()
        } else {
            self.grapheme_index_to_byte_index(from_grapheme_index)
        };

        return self.find_all(query, 0..end_byte_index)
            .last()
            .map(
                |(_, grapheme_index)| {
                    return *grapheme_index;
                }
            );
    }

    fn find_all(&self, query: &str, range: Range<ByteIndex>) -> Vec<(ByteIndex, GraphemeIndex)> {
        let start_byte_index = range.start;
        let end_byte_index = range.end;

        return self.string
            .get(start_byte_index..end_byte_index)
            .map_or_else(
                Vec::new,
                |substr| {
                    return substr
                        .match_indices(query)
                        .filter_map(
                            |(relative_start_index, _)| {
                                let absolute_start_index = relative_start_index.saturating_add(start_byte_index);

                                return self
                                    .byte_index_to_grapheme_index(absolute_start_index)
                                    .map(
                                        |grapheme_index| {
                                            return (absolute_start_index, grapheme_index);
                                        }
                                    );
                            }
                        )
                        .collect();
                }
            );
    }
}

impl Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            formatter,
            "{}",
            self.string
        );
    }
}

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return &self.string;
    }
}

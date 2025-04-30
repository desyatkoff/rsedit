pub mod annotationtype;
mod annotation;
mod annotatedstringpart;
mod annotatedstringiterator;

pub use annotationtype::AnnotationType;
use std::{
    cmp::{
        min,
        max,
    },
    fmt,
    fmt::Display,
};
use annotation::Annotation;
use annotatedstringpart::AnnotatedStringPart;
use annotatedstringiterator::AnnotatedStringIterator;

#[derive(Default, Debug)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        return Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation_type: AnnotationType, start_byte_index: usize, end_byte_index: usize) {
        self.annotations.push(
            Annotation {
                annotation_type,
                start_byte_index,
                end_byte_index,
            }
        );
    }

    pub fn replace(&mut self, start_byte_index: usize, end_byte_index: usize, new_string: &str) {
        let end_byte_index = min(end_byte_index, self.string.len());

        if start_byte_index > end_byte_index {
            return;
        }

        self.string.replace_range(start_byte_index..end_byte_index, new_string);

        let replaced_range_length = end_byte_index.saturating_sub(start_byte_index);
        let shortened = new_string.len() < replaced_range_length;
        let length_difference = new_string.len().abs_diff(replaced_range_length);

        if length_difference == 0 {
            return;
        }

        self
            .annotations
            .iter_mut()
            .for_each(
                |annotation| {
                    annotation.start_byte_index = if annotation.start_byte_index >= end_byte_index {
                        if shortened {
                            annotation.start_byte_index.saturating_sub(length_difference)
                        } else {
                            annotation.start_byte_index.saturating_add(length_difference)
                        }
                    } else if annotation.start_byte_index >= start_byte_index {
                        if shortened {
                            max(
                                start_byte_index,
                                annotation.start_byte_index.saturating_sub(length_difference),
                            )
                        } else {
                            min(
                                end_byte_index,
                                annotation.start_byte_index.saturating_add(length_difference),
                            )
                        }
                    } else {
                        annotation.start_byte_index
                    };

                    annotation.end_byte_index = if annotation.end_byte_index >= end_byte_index {
                        if shortened {
                            annotation.end_byte_index.saturating_sub(length_difference)
                        } else {
                            annotation.end_byte_index.saturating_add(length_difference)
                        }
                    } else if annotation.end_byte_index >= start_byte_index {
                        if shortened {
                            max(
                                start_byte_index,
                                annotation.end_byte_index.saturating_sub(length_difference),
                            )
                        } else {
                            min(
                                end_byte_index,
                                annotation.end_byte_index.saturating_add(length_difference),
                            )
                        }
                    } else {
                        annotation.end_byte_index
                    }
                }
            );

        self.annotations.retain(
            |annotation| {
                return
                    annotation.start_byte_index < annotation.end_byte_index &&
                    annotation.start_byte_index < self.string.len();
            }
        );
    }
}

impl Display for AnnotatedString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            formatter,
            "{}",
            self.string
        );
    }
}

impl<'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return AnnotatedStringIterator {
            annotated_string: self,
            current_index: 0,
        };
    }
}

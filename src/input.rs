use serde::{Serialize, Deserialize};
use std::collections::{HashMap};
use itertools::Itertools;
use std::cmp::Ordering;
use ratatui::{prelude::*, widgets::*};
use textwrap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentValueMetadata {
    pub is_primary_content: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentValue {
    pub meta: ContentValueMetadata,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentMetadataRecursive {
    pub is_root: bool,
    pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentMetadata {
    pub recursive: Option<ContentMetadataRecursive>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    pub id: String,
    pub meta: ContentMetadata,
    pub values: Vec<ContentValue>,
    pub children: Vec<Content>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub data: Content,
}

impl Content {
    pub fn go_down_depth(&self, depth: usize, results: &mut Vec<Content>) {
        if depth == 0 {
            results.push(self.clone());
        } else {
            for child in &self.children {
                child.go_down_depth(depth - 1, results);
            }
        }
    }

    pub fn to_lines(
        &self,
        filter_secondary_content: &bool,
        main_content_color: &Color,
        text_color: &Color,
        background_color: &Color,
        result: &mut Vec<Line>
    ) {
        let values: Vec<ContentValue> = self.values.iter()
            .into_iter()
            .sorted_by(|a, b| {
                match (a.meta.is_primary_content, b.meta.is_primary_content) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name)
                }
            })
            .cloned()
            .collect();

        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Line = Line::from(Vec::new());

        for item in values.iter() {
            let value = item.value.trim();
            let fg = if item.meta.is_primary_content {
                *main_content_color
            } else {
                *text_color
            };
            let current_line_length: usize = current_line.spans
                .iter()
                .map(|span| span.content.len()).sum();

            if value.len() > 160 {
                if current_line_length > 0 {
                    lines.push(current_line);
                    current_line = Line::from(Vec::new());
                }

                let wrapped = textwrap::wrap(value, &textwrap::Options::new(160));

                for segment in wrapped {
                    lines.push(
                        Line::from(
                            Span::styled(
                                format!("{}", segment),
                                Style::new()
                                    .fg(fg)
                                    .bg(*background_color)
                            )
                        )
                    );
                }
            } else {
                if value.len() + current_line_length > 160 {
                    lines.push(current_line);
                    current_line = Line::from(
                        Span::styled(
                            format!("{}", value),
                            Style::new()
                                .fg(fg)
                                .bg(*background_color)
                        )
                    );
                } else {
                    current_line.spans.push(
                        Span::styled(
                            format!(" {}", value),
                            Style::new()
                                .fg(fg)
                                .bg(*background_color)
                        )
                    );
                }
            }
        }

        let current_line_length: usize = current_line.spans
            .iter()
            .map(|span| span.content.len()).sum();

        if current_line_length > 0 {
            lines.push(current_line);
        }

        result.append(&mut lines);

        for child in &self.children {
            child.to_lines(filter_secondary_content, main_content_color, text_color, background_color, result);
        }
    }

}

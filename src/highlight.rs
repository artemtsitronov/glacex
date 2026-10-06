use std::ops::Range;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator, Tree};
use tree_sitter_rust;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Keyword,
    Function,
    String,
    Number,
    Comment,
    Operator,
    Type,
    Plain,
}

fn token_for_capture(name: &str) -> Option<Token> {
    let head = name.split('.').next()?;
    Some(match head {
        "keyword" => Token::Keyword,
        "function" => Token::Function,
        "string" => Token::String,
        "number" | "constant" => Token::Number,
        "comment" => Token::Comment,
        "operator" => Token::Operator,
        "type" => Token::Type,
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
}

impl Language {
    fn grammar(self) -> (tree_sitter::Language, &'static str) {
        match self {
            Language::Rust => (
                tree_sitter_rust::LANGUAGE.into(),
                tree_sitter_rust::HIGHLIGHTS_QUERY,
            ),
        }
    }
}

pub struct Highlighter {
    parser: Parser,
    query: Query,
    cursor: QueryCursor,
    tokens: Vec<Option<Token>>,
    tree: Option<Tree>,
    source: String,
}

impl Highlighter {
    pub fn new(l: Language) -> Self {
        let (language, highlights) = l.grammar();

        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .expect("grammar/runtime version mismatch");

        let query = Query::new(&language, highlights)
            .unwrap_or_else(|e| panic!("bad highlights query: {e}"));

        let tokens = query
            .capture_names()
            .iter()
            .map(|n| token_for_capture(n))
            .collect();

        Highlighter {
            parser,
            query,
            cursor: QueryCursor::new(),
            tokens,
            tree: None,
            source: String::new(),
        }
    }

    pub fn update(&mut self, text: &str) -> bool {
        if self.tree.is_some() && self.source == text {
            return false;
        }
        self.source.clear();
        self.source.push_str(text);
        self.tree = self.parser.parse(&self.source, None);
        true
    }

    pub fn paint(&mut self, range: Range<usize>) -> Vec<Token> {
        let mut out = vec![Token::Plain; range.len()];
        let Some(tree) = self.tree.as_ref() else {
            return out;
        };

        self.cursor.set_byte_range(range.clone());

        // (pattern_index, start, end, token)
        let mut hits: Vec<(usize, usize, usize, Token)> = Vec::new();

        let mut caps = self
            .cursor
            .captures(&self.query, tree.root_node(), self.source.as_bytes());

        while let Some((m, i)) = caps.next() {
            let cap = m.captures[*i];
            let Some(token) = self.tokens[cap.index as usize] else {
                continue;
            };
            let r = cap.node.byte_range();
            let start = r.start.max(range.start);
            let end = r.end.min(range.end);
            if start < end {
                hits.push((m.pattern_index, start, end, token));
            }
        }

        hits.sort_by_key(|h| h.0);
        for (_, start, end, token) in hits {
            out[start - range.start..end - range.start].fill(token);
        }
        out
    }
}

pub fn line_runs(tokens: &[Token], base: usize, line: Range<usize>) -> Vec<(Range<usize>, Token)> {
    let slice = &tokens[line.start - base..line.end - base];
    let mut runs = Vec::new();
    let mut start = 0;
    for i in 1..=slice.len() {
        if i == slice.len() || slice[i] != slice[start] {
            runs.push((start..i, slice[start]));
            start = i;
        }
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlighter_test() {
        let src = "fn main() { let x = 42; // hi\n}";
        let mut h = Highlighter::new(Language::Rust);
        assert!(h.update(src));
        assert!(!h.update(src));
        let t = h.paint(0..src.len());
        println!("{:?}", &t[..2]);
        assert_eq!(t[0], Token::Keyword);
        assert_eq!(t[src.find("42").unwrap()], Token::Number);
        assert_eq!(t[src.find("//").unwrap()], Token::Comment);
    }
}

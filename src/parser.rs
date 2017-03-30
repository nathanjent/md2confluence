use nom::{IResult, alphanumeric, eol};
use std::str;

named!(end_of_line, alt!(eof!() | eol));

named!(read_line <&str>, map_res!(
  terminated!(alphanumeric, end_of_line),
  str::from_utf8
));

named!(read_lines <Vec<&str> >, many0!(read_line));

// TODO make a better parser
//named!(parse_markdown <Vec<Block> >, many0!(read_line));

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    /// # Header 1
    Header(Vec<Span>, usize),
    /// Normal text.
    Paragraph(Vec<Span>),
    /// > Quoted text.
    Blockquote(Vec<Block>),
    /// ```rust
    /// fn main() {
    ///     println!("Hello world");
    /// }
    /// ```
    CodeBlock(String, String),
    /// 1. item 1
    /// 2. item 2
    OrderedList(Vec<ListItem>),
    /// - item
    /// * item
    /// + item
    UnorderedList(Vec<ListItem>),
    /// ```
    /// Unformatted text.
    /// Markdown is ignored in here.
    /// ```
    Raw(String),
    /// Horizontal rule.
    /// ---
    /// ***
    /// ___
    Hr
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ListItem {
    Simple(Vec<Span>),
    Paragraph(Vec<Block>)
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Span {
    /// \n
    Break,
    /// Normal text.
    Text(String),
    /// `inline code` 
    Code(String),
    /// | Column Header | Header | C     |
    /// | ------------- |:------:| -----:|
    /// | left-aligned  | center | right |
    // TODO
    //Table(Vec<Span>, u32, u32),
    /// Link with text and tool-tip:
    /// [text](url "tool-tip")
    ///
    /// References:
    /// [text][1]
    /// ...
    /// [1]: url "tool-tip"
    Link(String, String, Option<String>),
    /// ![alt-text](url "tool-tip")
    /// Also uses references like link
    Image(String, String, Option<String>),
    /// *emphasis* or _emphasis_
    Emphasis(Vec<Span>),
    /// **strong** or __strong__
    Strong(Vec<Span>),
    /// ~~strike~~
    Strikethrough(Vec<Span>)
}

#[test]
fn read_lines_test() {
  let res = IResult::Done(&b""[..], vec!["Duck", "Dog", "Cow"]);

  assert_eq!(read_lines(&b"Duck\nDog\nCow\n"[..]), res);
  assert_eq!(read_lines(&b"Duck\nDog\nCow"[..]),   res);
}

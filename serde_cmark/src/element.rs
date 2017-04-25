
use std::fmt;
use std::borrow::Cow;
use std::marker::PhantomData;

use serde::{self, Serialize, Deserialize, DeserializeOwned, Deserializer};
use serde::de::{self, Unexpected, Visitor};
use pulldown_cmark::{Parser, Event, Tag};

use error::Error;
use ser::Serializer;

#[derive(Clone, Debug)]
pub enum Element<'a> {
    Start(Tag<'a>),
    End(Tag<'a>),
    Text(Cow<'a, str>),
    Html(Cow<'a, str>),
    InlineHtml(Cow<'a, str>),
    FootnoteReference(Cow<'a, str>),
    SoftBreak,
    HardBreak,
}

#[derive(Clone, Debug)]
pub enum ElementTag<'a> {
    Paragraph,
    Rule,
    Header(i32),
    BlockQuote,
    CodeBlock(Cow<'a, str>),
    List(Option<usize>),
    Item,
    FootnoteDefinition(Cow<'a, str>),
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Code,
    Link(Cow<'a, str>, Cow<'a, str>),
    Image(Cow<'a, str>,
    Cow<'a, str>),
}

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

impl<'a> From<Event<'a>> for Element<'a> {
    fn from(event: Event<'a>) -> Self {
        match event {
            Event::Start(tag) => Element::Start(tag),
            Event::End(tag) => Element::End(tag),
            Event::Text(text) => Element::Text(text),
            Event::Html(html) => Element::Html(html),
            Event::InlineHtml(iHtml) => Element::InlineHtml(iHtml),
            Event::FootnoteReference(name) => Element::FootnoteReference(name),
            Event::SoftBreak => Element::SoftBreak,
            Event::HardBreak => Element::HardBreak,
        }
    }
}

impl<'a> From<Tag<'a>> for ElementTag<'a> {
    fn from(event: Tag<'a>) -> Self {
        match event {
            Tag::Paragraph => ElementTag::Paragraph,
            Tag::Rule => ElementTag::Rule,
            Tag::Header(level) => ElementTag::Header(level),
            Tag::BlockQuote => ElementTag::BlockQuote,
            Tag::CodeBlock(unformatted_text) => ElementTag::CodeBlock(unformatted_text),
            Tag::List(start_opt) => ElementTag::List(start_opt),
            Tag::Item => ElementTag::Item,
            Tag::FootnoteDefinition(name) => ElementTag::FootnoteDefinition(name),
            Tag::Table(alignments) => {
                let mut als = Vec::new();
                for a in alignments {
                    let debug_string = format!("{:?}", a);
                    als.push((&*debug_string).into());
                }
                ElementTag::Table(als)
            }
            Tag::TableHead => ElementTag::TableHead,
            Tag::TableRow => ElementTag::TableRow,
            Tag::TableCell => ElementTag::TableCell,
            Tag::Emphasis => ElementTag::Emphasis,
            Tag::Strong => ElementTag::Strong,
            Tag::Code => ElementTag::Code,
            Tag::Link(dest, title) => ElementTag::Link(dest, title),
            Tag::Image(dest, title) => ElementTag::Image(dest, title),
        }
    }
}

impl<'a> From<&'a str> for Alignment {
    fn from(s: &'a str) -> Self {
        match s {
            "Alignment::None" => Alignment::None,
            "Alignment::Left" => Alignment::Left,
            "Alignment::Center" => Alignment::Center,
            "Alignment::Right" => Alignment::Right,
        }
    }
}


struct ElementVisitor<'a, T: 'a> {
    marker: PhantomData<&'a T>,
}

impl<'a, V: 'a> de::Visitor for ElementVisitor<'a, &'a V> {
    type Value = Element<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a document Element")
    }

//            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
//                where E: de::Error
//            {
//                if s.len() >= self.min {
//                    Ok(s.to_owned())
//                } else {
//                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
//                }
//            }
}

impl<'a> Deserialize for Element<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer
    {
        deserializer.deserialize(ElementVisitor<'a> {..})
    }
}

pub fn to_element<T>(element: T) -> Result<Element, Error>
where
    T: Serialize,
{
    element.serialize(Serializer)
}

pub fn from_element<T>(element: Element) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(element)
}

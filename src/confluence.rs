use markdown::Block;
use markdown::ListItem;
use markdown::Span;

// takes a number of elements and returns their collective text as a slug
fn slugify(elements: &[Span]) -> String {
    let mut ret = String::new();

    for el in elements {
        let next = match *el {
            Span::Break => "".to_owned(),
            Span::Text(ref text) |
            Span::Link(ref text, _, _) |
            Span::Image(ref text, _, _) |
            Span::Code(ref text) => text.trim().replace(" ", "_").to_lowercase().to_owned(),
            Span::Strong(ref content) |
            Span::Emphasis(ref content) => slugify(content),
        };
        if !ret.is_empty() {
            ret.push('_');
        }
        ret.push_str(&next);
    }

    ret
} 

pub fn to_confluence(blocks: &[Block]) -> String {
    let mut ret = String::new();
    for block in blocks.iter() {
        let next = match *block {
            Block::Header(ref elements, level) => format_header(elements, level),
            Block::Paragraph(ref elements) => format_paragraph(elements),
            Block::Blockquote(ref elements) => format_blockquote(elements),
            Block::CodeBlock(ref elements) => format_codeblock(elements),
            Block::UnorderedList(ref elements) => format_unordered_list(elements),
            Block::Raw(ref elements) => elements.to_owned(),
            Block::Hr => format!("<hr>"),
        };
        ret.push_str(&next)
    }
    ret = ret.trim().to_owned();
    ret.push('\n');
    ret
}

fn format_spans(elements: &[Span]) -> String {
    let mut ret = String::new();
    for element in elements.iter() {
        let next = match *element {
            Span::Break => format!("\n"),
            Span::Text(ref text) => format!("{}", text),
            Span::Code(ref text) => format!("{{code}}{}{{code}}", text),
            Span::Link(ref text, ref url, None) => format!("[{}|{}]", text, url),
            Span::Link(ref text, ref url, Some(ref tip)) => format!("[{}|{}|{}]", text, url, tip),
            Span::Image(ref text, ref url, None) => format!("!{}|alt={}!", url, text),
            Span::Image(ref text, ref url, Some(ref title)) => format!("!{}|title={},alt={}!", url, title, text),
            Span::Emphasis(ref content) => format!("_{}_", format_spans(content)),
            Span::Strong(ref content) => format!("*{}*", format_spans(content)),
        };
        ret.push_str(&next)
    }
    ret
}

fn escape(text: &str) -> String {
    text.replace("&", "&amp;") .replace("<", "&lt;") .replace("\"", "&quot;") .replace("'", "&#8217;") .replace(">", "&gt;")
}

fn format_unordered_list(elements: &[ListItem]) -> String {
    let mut ret = String::new();
    for list_item in elements {
        let mut content = String::new();
        match *list_item {
            ListItem::Simple(ref els) => {
                content.push_str(&format_spans(els))
            }
            ListItem::Paragraph(ref paragraphs) => {
                content.push_str(&format!("{}", to_confluence(paragraphs)))
            }
        }

        ret.push_str(&format!("- {}\n", content))
    }
    format!("{}", ret)
} 

fn format_codeblock(elements: &str) -> String {
    format!("{}\n", elements)
} 

fn format_blockquote(elements: &[Block]) -> String {
    format!("{{noformat}}\n{}\n{{noformat}}\n", to_confluence(elements))
}

fn format_paragraph(elements: &[Span]) -> String {
    format!("{}\n", format_spans(elements))
}

fn format_header(elements: &[Span], level: usize) -> String {
    format!("h{}. {}\n", level, format_spans(elements))
}

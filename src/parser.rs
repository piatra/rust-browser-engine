use std::collections::HashMap;
use dom;

pub struct Parser {
    pos: uint,
    input: String
}

impl Parser {
    fn next_char(&self) -> char {
        self.input.as_slice().char_at(self.pos)
    }

    fn starts_with(&self, c: &str) -> bool {
        self.input.as_slice().slice_from(self.pos).starts_with(c)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        return range.ch;
    }

    fn consume_while(&mut self, test: |char| -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char())
        }
        return result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<' && c != '-'))
    }

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        if self.next_char() == '!' {
            assert!(self.consume_char() == '!')
            assert!(self.consume_char() == '-')
            assert!(self.consume_char() == '-')
            let content = self.parse_text();
            assert!(self.consume_char() == '-')
            assert!(self.consume_char() == '-')
            assert!(self.consume_char() == '>');
            return dom::comment(content)
        }
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attrs = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }
        return attrs;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() ||
               self.starts_with("</") ||
               self.starts_with("--") {
                break;
            }
            nodes.push(self.parse_node());
        }

        return nodes;
    }
}

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0u, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0).unwrap()
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
pub struct CollapseHtml {
    current_hash: Vec<u8>,
    tags: HashMap<String, String>,
}

impl CollapseHtml {
    pub fn new() -> CollapseHtml {
        CollapseHtml {
            current_hash: vec![68, 48, 51, 50],
            tags: HashMap::new(),
        }
    }

    pub fn tag_list(html: &str) -> HashSet<&str> {
        let re = Regex::new(r"(<[^>]*>|<[^>]*/>|</[^>]*>|&[^;]+)").unwrap();
        re.captures_iter(html)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect::<::std::collections::HashSet<_>>()
    }

    fn get_replacement(&mut self, tag: String) -> String {
        if self.tags.contains_key(&tag) {
            self.tags.get(&tag).unwrap().clone()
        } else {
            let replacement = String::from_utf8(self.current_hash.clone()).unwrap();
            let mut index_add = 3;
            if self.current_hash[3] == 255 {
                self.current_hash[3] = 0;
                index_add = 2;
                if self.current_hash[2] == 255 {
                    self.current_hash[2] = 0;
                    index_add = 1;
                    if self.current_hash[1] == 255 {
                        self.current_hash[1] = 0;
                        index_add = 0;
                    }
                }
            }
            self.current_hash[index_add] = self.current_hash[index_add] + 1;
            self.tags.insert(tag, replacement.clone());
            replacement
        }
    }

    pub fn expand(&self, html: &str) -> String {
        let mut t = html.to_string();
        for (tag, replacement) in self.tags.iter() {
            t = t.replace(&format!(" {} ", replacement), tag);
            // if it starts with deleted
            t = t.replace(&format!("{} ", replacement), tag);
        }
        t
    }

    //
    // Replace HTML tags with single characters so that diffing the HTML
    // does not result in corrupted tags.
    //
    // This does not handle issues with unclosed tags, or tags that overlap.
    //
    // Based on the aglorithm here: http://code.google.com/p/google-diff-match-patch/wiki/Plaintext
    //
    pub fn collapse(&mut self, html: &str) -> String {
        let mut t = html.to_string();
        for tag in CollapseHtml::tag_list(html) {
            let replacement = self.get_replacement(tag.to_string());

            t = t.replace(tag, &format!(" {} ", replacement));
        }
        t
    }

    pub fn scrub_tags(&self, html: &str) -> String {
        let mut t = html.to_string();
        for (_, replacement) in self.tags.iter() {
            t = t.replace(&format!("{}", replacement), "");
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_collapses() {
        let mut collapser = CollapseHtml::new();
        let results = collapser.collapse("a<a>");
        let output = "a D032 ";
        assert_eq!(results, output);
    }

    #[test]
    fn it_tag_list() {
        let results = CollapseHtml::tag_list("<a>b</a><a>c</a><b>d</b>");
        let output = vec!["<a>", "</a>", "<a>", "</a>", "<b>", "</b>"]
            .iter()
            .map(|x| &**x)
            .collect::<::std::collections::HashSet<_>>();
        assert_eq!(results, output);
    }

    #[test]
    fn it_expands() {
        let mut collapser = CollapseHtml::new();
        let col = collapser.collapse("<a>b</a><a>c</a><b>d</b>");
        let results = collapser.expand(&col);
        let output = "<a>b</a><a>c</a><b>d</b>";
        assert_eq!(results, output);
    }
}

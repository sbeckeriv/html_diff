use difference::{Changeset, Difference};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
// diff injects html markup for additions and removals
//
//
// ```
// let results = diff("a 1", "a 2");
// let output = "a<span class='deleted'> 1</span><span class='inserted'> 2</span>\n";
// assert_eq!(results, output);
// ```
pub fn diff(current: &str, old: &str) -> String {
    let mut t = String::with_capacity(old.len());
    let mut collapser = CollapseHtml::new();
    let collapsed_current = collapser.collapse(current);
    let collapsed_old = collapser.collapse(old);
    let tag_regex = collapser.tag_regex();
    let Changeset { diffs, .. } = Changeset::new(&collapsed_current, &collapsed_old, " ");
    let mut first = true;
    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                println!("same 1 {}", z);
                if !first {
                    write!(t, " ").unwrap();
                }
                write!(t, "{}", z).unwrap();
            }
            Difference::Rem(ref z) => {
                let clean_z = tag_regex.replace_all(z, "");
                println!("delete 1 -{}-", z);
                println!("delete 2 -{}-", clean_z);
                if clean_z.trim().len() > 0 {
                    write!(t, "<span class='deleted'> {}</span>", clean_z).unwrap();
                } else {
                    println!("skipping {}", z);
                }
            }

            Difference::Add(ref z) => {
                write!(t, "<span class='inserted'> {}</span>", z).unwrap();
            }
        }
        first = false;
    }

    writeln!(t, "").unwrap();
    collapser.expand(&t)
}

// collapse html by removing the tags. replace them with unused ascii char set so that they might
// never be conflicted.
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

    pub fn tag_regex(&self) -> Regex {
        let string = if self.tags.len() > 0 {
            let mut string = "(".to_string();
            for key in self.tags.values() {
                string.push_str(&key);
                string.push_str("|")
            }
            string.pop();
            string.push_str(")");
            string
        } else {
            "".to_string()
        };

        println!("{}", string);

        Regex::new(&string).unwrap()
    }

    fn tag_list(html: &str) -> Vec<&str> {
        let re = Regex::new(r"(<[^>]*>|<[^>]*/>|</[^>]*>|&[^;]+)").unwrap();
        re.captures_iter(html)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect()
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
        }
        t
    }

    pub fn collapse(&mut self, html: &str) -> String {
        let mut t = html.to_string();
        let the_list = CollapseHtml::tag_list(html);
        for tag in the_list {
            let replacement = self.get_replacement(tag.to_string());
            t = t.replace(tag, &format!(" {}  ", replacement));
        }
        println!("{}", t);
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces() {
        let results = diff("a 1", "a 2");
        let output = "a<span class='deleted'> 1</span><span class='inserted'> 2</span>\n";
        println!("{}", output);

        assert_eq!(results, output);
    }
}

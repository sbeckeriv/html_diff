mod collapse_html;
pub use crate::collapse_html::CollapseHtml;

use difference::{Changeset, Difference};
use std::fmt::Write as FmtWrite;

use std::fmt;
pub struct CustomDisplayChangeset {
    changeset: Changeset,
    collapser: CollapseHtml,
    config: DiffConfig,
}

impl std::fmt::Display for CustomDisplayChangeset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let html_tag = &self.config.html_tag;
        let insert_class = &self.config.insert_class;
        let delete_class = &self.config.delete_class;
        for d in &self.changeset.diffs {
            match *d {
                Difference::Same(ref x) => {
                    write!(f, "{}{}{}", self.changeset.split, x, self.changeset.split).unwrap();
                }
                Difference::Add(ref x) => {
                    write!(
                        f,
                        "<{} class='{}'{}>{}</{}>",
                        html_tag, insert_class, "", x, html_tag,
                    )
                    .unwrap();
                }
                Difference::Rem(ref x) => {
                    let y = self
                        .collapser
                        .scrub_tags(&format!("{}{}", x, self.changeset.split));
                    if y.chars().any(|c| !c.is_whitespace()) {
                        write!(
                            f,
                            "<{} class='{}'{}>{}</{}>",
                            html_tag,
                            delete_class,
                            "",
                            y.trim(),
                            html_tag,
                        )
                        .unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}
pub struct DiffConfig {
    html_tag: String,
    separator: String,
    insert_class: String,
    delete_class: String,
}

impl Default for DiffConfig {
    /// Creates an empty `String`.
    #[inline]
    fn default() -> DiffConfig {
        DiffConfig {
            html_tag: "span".to_string(),
            separator: " ".to_string(),
            delete_class: "deleted".to_string(),
            insert_class: "inserted".to_string(),
        }
    }
}

// diff injects html markup for additions and removals
//
//
// ```
// let results = diff("a 1", "a 2");
// let output = " a <span class='deleted'>1</span><span class='inserted'>2</span>\n";
// assert_eq!(results, output);
// ```
pub fn diff(current: &str, old: &str, config: Option<DiffConfig>) -> String {
    let config = config.unwrap_or_default();
    let mut t = String::with_capacity(old.len());
    let mut collapser = CollapseHtml::new();
    let collapsed_current = collapser.collapse(current);
    let collapsed_old = collapser.collapse(old);
    let changeset = Changeset::new(&collapsed_current, &collapsed_old, &config.separator);
    let display = CustomDisplayChangeset {
        changeset,
        collapser,
        config,
    };
    write!(t, "{}", display).unwrap();
    display
        .collapser
        .expand(&t)
        .replacen("> <", "><", 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces() {
        let results = diff("a 1", "a 2", None);
        let output = " a <span class='deleted'>1</span><span class='inserted'>2</span>";
        println!("{}", output);

        assert_eq!(results, output);
    }
}

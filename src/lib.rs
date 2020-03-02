mod collapse_html;
pub use crate::collapse_html::CollapseHtml;

use difference::{Changeset, Difference};
use std::fmt::Write as FmtWrite;

use std::fmt;
pub struct CustomDisplayChangeset {
    changeset: Changeset,
    collapser: CollapseHtml,
}
impl std::fmt::Display for CustomDisplayChangeset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in &self.changeset.diffs {
            match *d {
                Difference::Same(ref x) => {
                    write!(f, "{}{}", x, self.changeset.split).unwrap();
                }
                Difference::Add(ref x) => {
                    write!(
                        f,
                        "<{} class='{}'{}>{}{}</{}>",
                        "span", "inserted", "", x, self.changeset.split, "span",
                    )
                    .unwrap();
                }
                Difference::Rem(ref x) => {
                    let y = self
                        .collapser
                        .scrub_tags(&format!("{}{}", x, self.changeset.split));
                    if y.chars().find(|c| !c.is_whitespace()).is_some() {
                        write!(
                            f,
                            "<{} class='{}'{}>{}{}</{}>",
                            "span",
                            "deleted",
                            "",
                            y.trim(),
                            self.changeset.split,
                            "span",
                        )
                        .unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}

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
    let changeset = Changeset::new(&collapsed_current, &collapsed_old, " ");
    let display = CustomDisplayChangeset {
        changeset,
        collapser,
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
        let results = diff("a 1", "a 2");
        let output = "a <span class='deleted'>1 </span><span class='inserted'>2 </span>";
        println!("{}", output);

        assert_eq!(results, output);
    }
}

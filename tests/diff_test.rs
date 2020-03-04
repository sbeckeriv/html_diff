use html_diff::diff;
use pretty_assertions::assert_eq;

#[test]
fn it_removes() {
    let results = diff("a 1 2", "a 2", None);
    let output = " a <span class='deleted'>1</span> 2 ";
    assert_eq!(results, output);
}

#[test]
fn it_adds() {
    let results = diff("a 2", "a 2 1", None);
    let output = " a 2 <span class='inserted'>1</span>";
    assert_eq!(results, output);
}

#[test]
fn it_adds_html() {
    let base_html = "
        <table class='mce-item-table'>
        <tbody>
        </tbody>
        </table>";
    let new_html = "
        <table class='mce-item-table'>
        <tbody>
        <tr>
        <td>Feature Name:</td>
        <td>Time</td>
        <td><br></td>
        </tr>
        </tbody>
        </table>";
    let output = " 
        <table class='mce-item-table'>
        <tbody>
         <span class=\'inserted\'><tr>
        <td>Feature Name:</td>
        <td>Time</td>
        <td><br></td>
        </tr>
        </span></tbody>
        </table> ";
    let results = diff(base_html, new_html, None);
    assert_eq!(results, output);
}
#[test]
fn it_removes_html() {
    let new_html = "
        <table class='mce-item-table'>
        <tbody>
        </tbody>
        </table>";
    let base_html = "
        <table class='mce-item-table'>
        <tbody>
        <tr>
        <td>Feature Name:</td>
        <td>Time</td>
        <td><br></td>
        </tr>
        </tbody>
        </table>";
    let output = " 
        <table class='mce-item-table'>
        <tbody>
         <span class=\'deleted\'>Feature Name:  \n          Time</span></tbody>
        </table> ";
    let results = diff(base_html, new_html, None);
    assert_eq!(results, output);
}
#[test]
fn it_shows_removed_added() {
    let base_html = "
      <table class='mce-item-table'>
        <tbody>
          <tr>
            <td>Feature Name:</td>
            <td>Time</td>
            <td><br></td>
          </tr>
        </tbody>
      </table>";

    let new_html = "
      <table class='mce-item-table'>
        <tbody>
          <tr>
            <td>This was changed</td>
            <td><br></td>
          </tr>
        </tbody>
      </table>";

    let expected_html = " 
      <table class='mce-item-table'>
        <tbody>
          <tr>
            <td><span class=\'deleted\'>Feature Name:</span><span class=\'inserted\'>This was changed</span></td>
            <td><span class=\'deleted\'>Time</span>  <br></td>
          </tr>
        </tbody>
      </table> ";

    let results = diff(base_html, new_html, None);
    assert_eq!(results, expected_html);
}

#[test]
fn it_does_not_change() {
    let base_html = "
      <table class=\"mce-item-table\">
        <tbody>
          <tr>
            <td>Unchanged</td>
            <td><br></td>
            <td><br></td>
          </tr>
        </tbody>
      </table>";

    let results = diff(base_html, base_html, None);
    assert_eq!(results.trim(), base_html.trim());
}
#[test]
fn it_mid_html_replace() {
    let base_html = "<span><b> bold </b></span>";
    let new_html = "<span>not bold </span>";
    let output = " <span><span class='inserted'>not</span> bold  </span> ";
    let results = diff(base_html, new_html, None);
    assert_eq!(results, output);
}

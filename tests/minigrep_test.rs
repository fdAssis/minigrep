use minigrep::{run, search, Config};

#[test]
fn search_line_with_string_parsed() {
    let query = "duct";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

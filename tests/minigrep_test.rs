use minigrep::{run, search, Config};

#[test]
fn search_line_with_string_parsed() {
    let query = "duct";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
#[should_panic(expected = "Not enough arguments")]
fn err_not_enough_arguments() {
    let args: Vec<String> = vec![];
    let config = Config::new(&args);

    run(config.unwrap())
        .map_err(|error| error.to_string())
        .unwrap_err();
}

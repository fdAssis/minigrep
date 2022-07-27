/**
 * Note!!
 * É convenção trazer o módulo pai para o escopo em vez da função.
 * Como resultado, nós podemos facilmente usar outras funções de std::env.
 * Também é menos ambíguo que adicionar use std::env::args
 * e depois chamando a função com apenas args porque args
 * pode ser facilmente confundido com uma função definida no módulo atual.
 */
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for: {} ", config.query);
    println!("In file: {} ", config.filename);

    let mut file = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);

    println!("file:\n{:?}", file);
}

struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    fn new(args: &'a Vec<String>) -> Self {
        let query = &args[1];
        let filename = &args[2];

        Self { query, filename }
    }
}

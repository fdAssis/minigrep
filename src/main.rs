extern crate minigrep;
use minigrep::Config;
/**
 * Note!!
 * É convenção trazer o módulo pai para o escopo em vez da função.
 * Como resultado, nós podemos facilmente usar outras funções de std::env.
 * Também é menos ambíguo que adicionar use std::env::args
 * e depois chamando a função com apenas args porque args
 * pode ser facilmente confundido com uma função definida no módulo atual.
 */
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    };
}

use std::collections::HashMap;
use std::io::{self, Write};

use crate::comandos::{aqui::cmd_aqui, br::cmd_br, cp::cmd_cp, ls::cmd_ls, mv::cmd_mv};

type CommandFn = fn(&[String]);

fn get_comandos() -> HashMap<&'static str, CommandFn> {
    let mut map: HashMap<&'static str, CommandFn> = HashMap::new();
    map.insert("ls", cmd_ls as CommandFn);
    map.insert("cp", cmd_cp as CommandFn);
    map.insert("mv", cmd_mv as CommandFn);
    map.insert("br", cmd_br as CommandFn);
    map.insert("aqui", cmd_aqui as CommandFn);
    map
}

fn promp() {
    print!(":V -> ");
}

pub fn leer_comando() -> String {
    let mut input = String::new();
    promp();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input.trim().to_string()
}

pub fn parse_input(input: &str) -> Option<(String, Vec<String>)> {
    let mut partes = input.split_whitespace();
    let comando = partes.next()?.to_string();
    let args: Vec<String> = partes.map(|s| s.to_string()).collect();
    Some((comando, args))
}

pub fn ejecutar_comando(comando: &str, args: &[String]) {
    let comandos = get_comandos();
    match comandos.get(comando) {
        Some(f) => f(args),
        None => println!("Comando no reconocido: {}", comando),
    }
}

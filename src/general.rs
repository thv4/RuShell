use crate::comandos::{aqui::cmd_aqui, br::cmd_br, cp::cmd_cp, ls::cmd_ls, mv::cmd_mv};
use std::io::{self, Write};

pub enum Comandos {
    Ls { path: Option<String> },
    Cp { origen: String, destino: String },
    Mv { origen: String, destino: String },
    Br { destino: String },
    Aqui,
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

pub fn map_comando(comando: String, args: Vec<String>) -> Option<Comandos> {
    match comando.as_str() {
        "ls" => {
            let path = args.get(0).cloned(); // Opción para la ruta (o `None`)
            Some(Comandos::Ls { path })
        }
        "cp" if args.len() == 2 => Some(Comandos::Cp {
            origen: args[0].clone(),
            destino: args[1].clone(),
        }),
        "mv" if args.len() == 2 => Some(Comandos::Mv {
            origen: args[0].clone(),
            destino: args[1].clone(),
        }),
        "br" if args.len() == 1 => Some(Comandos::Br {
            destino: args[0].clone(),
        }),
        "aqui" => Some(Comandos::Aqui),
        _ => {
            println!("Comando no reconocido: {}", comando);
            None
        }
    }
}

pub fn ejecutar_comando(comando: Comandos) {
    match comando {
        Comandos::Ls { path } => {
            cmd_ls(path);
        }
        Comandos::Cp { origen, destino } => {
            cmd_cp(&origen, &destino);
        }
        Comandos::Mv { origen, destino } => {
            cmd_mv(&origen, &destino);
        }
        Comandos::Br { destino } => {
            cmd_br(&destino);
        }
        Comandos::Aqui => {
            cmd_aqui();
        }
    }
}

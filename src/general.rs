use std::io::{self, Write};

pub enum Comandos {
    Ls { path: Option<String> },
    Cp { origen: String, destino: String },
    Mv { origen: String, destino: String },
    Br { origen: String, destino: String },
    Aqui,
}

fn promp() {
    print!(":V -> ");
}

pub fn leerComando() -> String {
    let mut input = String::new();
    promp();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input.trim().to_string()
}

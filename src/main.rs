use general::{ejecutar_comando, leer_comando, parse_input};

mod comandos;
mod errores;
mod general;

fn main() {
    loop {
        let input = leer_comando();
        if input == "salir" {
            break;
        }

        if let Some((comando, args)) = parse_input(&input) {
            ejecutar_comando(&comando, &args);
        }
    }
}

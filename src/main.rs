use general::leerComando;

mod comandos;
mod errores;
mod general;

fn main() {
    loop {
        let input = leerComando();
        if input == "salir" {
            break;
        }
    }
}

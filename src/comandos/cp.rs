use std::fs;

pub fn cmd_cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Uso: cp <origen> <destino>");
        return;
    }
    match fs::copy(&args[0], &args[1]) {
        Ok(_) => println!("Archivo copiado de {} a {}", args[0], args[1]),
        Err(_) => eprintln!("Error al copiar de {} a {}", args[0], args[1]),
    }
}

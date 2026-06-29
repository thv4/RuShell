use std::fs;

pub fn cmd_mv(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Uso: mv <origen> <destino>");
        return;
    }
    match fs::rename(&args[0], &args[1]) {
        Ok(_) => println!("Archivo movido de {} a {}", args[0], args[1]),
        Err(_) => eprintln!("Error al mover el archivo de {} a {}", args[0], args[1]),
    }
}

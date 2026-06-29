use std::fs;

pub fn cmd_ls(args: &[String]) {
    let dir = args.first().map(|s| s.as_str()).unwrap_or(".");
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let name = e.file_name().into_string().unwrap_or_default();
                        println!("{}", name);
                    }
                    Err(_) => eprintln!("Error al leer entrada en {}", dir),
                }
            }
        }
        Err(_) => eprintln!("Error al leer el directorio {}", dir),
    }
}

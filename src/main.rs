use fltk::{app, prelude::*, window::Window};
use rusqlite::{Connection, Result};
use std::path::Path;

fn main() -> Result<()>  {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "App de la gordis");
    wind.end();
    wind.show();
    app.run().unwrap();    
    let conn = Connection::open("datos.db")?;
    // Aquí puedes ejecutar consultas y realizar operaciones en la base de datos

    let mut database_path = "datos.db";
    // Descomentá la siguiente linea para comprobar que el condicional funciona:
    // database_path = "noexiste.db";
    if Path::new(database_path).exists() {
        // Si la base de datos es creada correctamente, o la misma ya existe en el sistema, la siguiente línea debería imprimir su nombre:
        println!("Base de datos creada, nombre: {}", database_path);
    } else {
        println!("No ha sido posible crear la base de datos o la misma no existe.");
    }

    Ok(())
}

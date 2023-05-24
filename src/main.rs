use fltk::{app, button::Button, enums::Color, frame::Frame, input::Input, prelude::*, window::Window};
use rusqlite::{Connection, Result};
use std::path::Path;

mod models;

// Colors for the interface
const RED: u32 = 0xe51c23;
const GREEN: u32 = 0x8bc34a;

// Componentes:
struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(title: &'static str) -> MyButton {
        let mut b = MyButton {
            btn: Button::new(50, 50, 90, 90, title),
        };
        b
    }
}

fn main() -> Result<()>  {
    let app = app::App::default();
    // set white bg color
    let mut window = fltk::window::Window::new(100, 100, 400, 500, "Mi Aplicación Rust");
    window.make_resizable(true); 
    window.set_color(Color::White);

    let mut frame = Frame::new(10, 10, 380, 30, "");

    let mut input_calle = Input::new(100, 50, 280, 30, "Calle:");
    let mut input_numero = Input::new(100, 90, 280, 30, "Número:");
    let mut input_piso = Input::new(100, 130, 280, 30, "Piso:");
    let mut input_codigo_postal = Input::new(100, 170, 280, 30, "Código Postal:");
    let mut input_metros_cuadrados = Input::new(100, 210, 280, 30, "Metros Cuadrados:");
    let mut input_cantidad_banios = Input::new(100, 250, 280, 30, "Cantidad de Baños:");
    let mut input_cantidad_habitaciones = Input::new(100, 290, 280, 30, "Cantidad de Habitaciones:");
    let mut input_tipo = Input::new(100, 330, 280, 30, "Tipo de Vivienda:");

    let mut button = Button::new(150, 370, 100, 30, "Cargar");
    window.end();
    window.show();
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

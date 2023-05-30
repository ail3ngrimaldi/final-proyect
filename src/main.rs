use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay},
};
use rusqlite::{params, Connection, Result};

mod database;
mod models;

pub use crate::models::Vivienda;

fn mostrar_datos_ventana(data: &str, conn: Connection) {
    let mut window = Window::new(100, 100, 1200, 300, "Datos");
    let mut text_buffer = TextBuffer::default();
    let mut text_display = TextDisplay::new(10, 10, 1180, 280, "");
    text_buffer.set_text(&data.to_string());
    text_display.set_buffer(Some(text_buffer));
    window.make_resizable(true);
    window.end();
    window.show();

    let lines = data.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let mut delete_button = Button::new(10, 300 + index as i32 * 30, 80, 20, "Delete");
        let line_copy = line.to_string();
        let conn_copy = &conn;

        delete_button.set_callback(move |_| {
            let elements: Vec<_> = line_copy.split(", ").collect();
            if elements.len() == 8 {
                let calle = elements[0].split(": ").nth(1).unwrap();
                let numero = elements[1].split(": ").nth(1).unwrap();
                let codigo_postal = elements[3].split(": ").nth(1).unwrap();
                if let Err(err) = database::eliminar_dato(&conn_copy, calle, numero, codigo_postal) {
                    println!("Error al eliminar dato: {:?}", err);
                } else {
                    println!("Dato eliminado correctamente");
                }
            }
        });
    }
}


fn main() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window = Window::new(100, 100, 400, 650, "Mi Aplicación Rust");
    let mut frame = Frame::new(0, 0, 400, 50, "Completá el formulario");
    let mut button_save = Button::new(110, 520, 80, 40, "Guardar");
    let mut button_see_data = Button::new(210, 520, 80, 40, "Ver datos");
    let mut input_calle = Input::new(100, 100, 200, 30, "Calle");
    let mut input_numero = Input::new(100, 150, 200, 30, "Número");
    let mut input_piso = Input::new(100, 200, 200, 30, "Piso");
    let mut input_codigo_postal = Input::new(100, 250, 200, 30, "Código Postal");
    let mut input_metros_cuadrados = Input::new(100, 300, 200, 30, "Metros Cuadrados");
    let mut input_cantidad_banios = Input::new(100, 350, 200, 30, "Cantidad de Baños");
    let mut input_cantidad_habitaciones =
        Input::new(100, 400, 200, 30, "Cantidad de Habitaciones");
    let mut input_tipo = Input::new(100, 450, 200, 30, "Tipo");

    window.set_color(Color::White);
    window.make_resizable(true);
    frame.set_frame(FrameType::FlatBox);
    frame.set_label_color(Color::from_u32(0x4D4D4D));

    window.end();
    window.show();

    let conn = Connection::open("datos.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS viviendas (
            id INTEGER PRIMARY KEY,
            calle TEXT,
            numero INTEGER,
            piso INTEGER,
            codigo_postal INTEGER,
            metros_cuadrados REAL,
            cantidad_banios INTEGER,
            cantidad_habitaciones INTEGER,
            tipo TEXT
        )",
        params![],
    )?;
    println!("Tabla 'viviendas' creada correctamente.");

    button_save.set_callback(move |_| {
        let calle = input_calle.value();
        let numero = input_numero.value();
        let piso = input_piso.value();
        let codigo_postal = input_codigo_postal.value();
        let metros_cuadrados = input_metros_cuadrados.value();
        let cantidad_banios = input_cantidad_banios.value();
        let cantidad_habitaciones = input_cantidad_habitaciones.value();
        let tipo = input_tipo.value();

        database::insertar_datos(
            &conn,
            &calle,
            &numero,
            &piso,
            &codigo_postal,
            &metros_cuadrados,
            &cantidad_banios,
            &cantidad_habitaciones,
            &tipo,
        )
        .expect("Error al insertar datos");
    
        mostrar_datos(&conn); 
         
    });

    button_see_data.set_callback(move |_| {
        if let Ok(conn) = Connection::open("datos.db") {
            mostrar_datos(&conn); // Mostrar datos al hacer clic en "Ver datos"
        } else {
            println!("Error al abrir la conexión");
        }
    });


    app.run().unwrap();

    Ok(())
}

// funcion mostrar datos, reutilizable
fn mostrar_datos(conn: &Connection) {
    if let Ok(data) = database::mostrar_datos(conn) {
        if data.is_empty() {
            println!("No hay datos para mostrar");
        } else {
            let data_text = data
                .iter()
                .map(|vivienda| format!(
                    "Calle: {}, Número: {}, Piso: {:?}, Código Postal: {}, Metros Cuadrados: {}, Cantidad de Baños: {}, Cantidad de Habitaciones: {}, Tipo: {}",
                    vivienda.calle,
                    vivienda.numero,
                    vivienda.piso,
                    vivienda.codigo_postal,
                    vivienda.metros_cuadrados,
                    vivienda.cantidad_banios,
                    vivienda.cantidad_habitaciones,
                    vivienda.tipo
                ))
                .collect::<Vec<String>>()
                .join("\n");

            mostrar_datos_ventana(&data_text.to_string());
        }
    } else {
        println!("Error al mostrar datos");
    }
}
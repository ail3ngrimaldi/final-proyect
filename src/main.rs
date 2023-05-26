use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
};
use rusqlite::{params, Connection, Result};

mod database;

fn main() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window = Window::new(100, 100, 400, 500, "Mi Aplicación Rust");
    let mut frame = Frame::new(0, 0, 400, 50, "");
    let mut button = Button::new(160, 420, 80, 40, "Guardar");
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

    button.set_callback(move |_| {
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
    if let Err(err) = database::mostrar_datos(&conn) {
        println!("Error al mostrar datos: {:?}", err);
    }
        // Resto del código para limpiar los campos de entrada o mostrar un mensaje de éxito
    });


    app.run().unwrap();

    Ok(())
}

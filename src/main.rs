use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay}, dialog::alert,
};
use rusqlite::{params, Connection, Result};

mod database;
mod models;

pub use crate::models::Vivienda;

fn mostrar_datos_ventana(data: &str) {
    let mut window = Window::new(100, 100, 1200, 300, "Datos");
    let mut text_buffer = TextBuffer::default();
    let mut text_display = TextDisplay::new(10, 10, 1180, 280, "");
    text_buffer.set_text(&data.to_string());
    text_display.set_buffer(Some(text_buffer));
    window.make_resizable(true);
    window.end();
    window.show();
}


fn main() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window = Window::new(100, 100, 500, 650, "Mi Aplicación Rust");
    let mut frame = Frame::new(0, 0, 500, 50, "Completá el formulario");
    let mut button_save = Button::new(210, 520, 80, 40, "Guardar");
    let mut button_see_data = Button::new(310, 520, 80, 40, "Ver datos");
    let mut delete_window = Button::new(210, 570, 80, 40, "Borrar");
    let mut button_edit = Button::new(310, 570, 80, 40, "Editar");
    let input_calle = Input::new(200, 100, 200, 30, "Calle");
    let input_numero = Input::new(200, 150, 200, 30, "Número");
    let input_piso = Input::new(200, 200, 200, 30, "Piso");
    let input_codigo_postal = Input::new(200, 250, 200, 30, "Código Postal");
    let input_metros_cuadrados = Input::new(200, 300, 200, 30, "Metros Cuadrados");
    let input_cantidad_banios = Input::new(200, 350, 200, 30, "Cantidad de Baños");
    let input_cantidad_habitaciones = Input::new(200, 400, 200, 30, "Cantidad de Habitaciones");
    let input_tipo = Input::new(200, 450, 200, 30, "Tipo");

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

        if calle.trim().is_empty()
        || numero.trim().is_empty()
        || piso.trim().is_empty()
        || codigo_postal.trim().is_empty()
        || metros_cuadrados.trim().is_empty()
        || cantidad_banios.trim().is_empty()
        || cantidad_habitaciones.trim().is_empty()
        || tipo.trim().is_empty()
    {
        // Muestra una alerta si alguno de los campos de entrada está vacío
        fltk::dialog::alert_default("El formulario no puede ser enviado vacío");
    } else {
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
         fltk::dialog::alert_default("Los datos fueron guardados exitosamente");
    }
    });

    button_see_data.set_callback(move |_| {
        if let Ok(conn) = Connection::open("datos.db") {
            mostrar_datos(&conn);
        } else {
            println!("Error al abrir la conexión");
        }
    });

    delete_window.set_callback(move |_| {
        let mut delete_window = Window::new(400, 300, 400, 300, "Borrar datos");
        let mut delete_button = Button::new(150, 200, 100, 40, "Borrar");
        let input_calle_delete = Input::new(100, 100, 200, 30, "Calle");
        let input_numero_delete = Input::new(100, 150, 200, 30, "Número");
        delete_window.end();
        delete_window.show();
    
        delete_button.set_callback(move |_| {
            if let Ok(conn) = Connection::open("datos.db") {
                let calle_delete = input_calle_delete.value();
                let numero_delete = input_numero_delete.value();
        
                match database::borrar_datos(&conn, &calle_delete, &numero_delete) {
                    Ok(rows) => {
                        if rows > 0 {
                            alert(200, 200, "Datos borrados exitosamente.");
                            delete_window.hide(); // Cerrar la ventana después de borrar
                        } else {
                            alert(200, 200, "No se encontraron datos para borrar.");
                        }
                    }
                    Err(_) => {
                        alert(200, 200, "Error al borrar datos.");
                    }
                }
            } else {
                alert(200, 200, "Error al abrir la conexión");
            }
        });
    });

    button_edit.set_callback(move |_| {
        let mut edit_window = Window::new(500, 400, 600, 650, "Editar datos");
        let mut edit_button = Button::new(250, 600, 150, 40, "Guardar cambios");
        let input_calle_edit = Input::new(250, 100, 200, 30, "Calle a editar");
        let input_numero_edit = Input::new(250, 150, 200, 30, "Número a editar");
        // campos para los nuevos datos
        let input_calle_new = Input::new(250, 200, 200, 30, "Nueva Calle");
        let input_numero_new = Input::new(250, 250, 200, 30, "Nuevo Número");
        let input_piso_new = Input::new(250, 300, 200, 30, "Nuevo Piso");
        let input_codigo_postal_new = Input::new(250, 350, 200, 30, "Nuevo Código Postal");
        let input_metros_cuadrados_new = Input::new(250, 400, 200, 30, "Nuevos Metros Cuadrados");
        let input_cantidad_banios_new = Input::new(250, 450, 200, 30, "Nueva Cantidad de Baños");
        let input_cantidad_habitaciones_new = Input::new(250, 500, 200, 30, "Nueva Cantidad de Habitaciones");
        let input_tipo_new = Input::new(250, 550, 200, 30, "Nuevo Tipo");
    
        edit_window.end();
        edit_window.show();
    
        edit_button.set_callback(move |_| {
            if let Ok(conn) = Connection::open("datos.db") {
                let calle_edit = input_calle_edit.value();
                let numero_edit = input_numero_edit.value();
                // obtener los nuevos valores
                let calle_new = input_calle_new.value();
                let numero_new = input_numero_new.value();
                let piso_new = input_piso_new.value();
                let codigo_postal_new = input_codigo_postal_new.value();
                let metros_cuadrados_new = input_metros_cuadrados_new.value();
                let cantidad_banios_new = input_cantidad_banios_new.value();
                let cantidad_habitaciones_new = input_cantidad_habitaciones_new.value();
                let tipo_new = input_tipo_new.value();
    
                match database::editar_datos(&conn, &calle_edit, &numero_edit, &calle_new, &numero_new, &piso_new, &codigo_postal_new, &metros_cuadrados_new, &cantidad_banios_new, &cantidad_habitaciones_new, &tipo_new) {
                    Ok(rows) => {
                        if rows > 0 {
                            alert(200, 200, "Datos editados exitosamente.");
                            // Cerrar la ventana después de editar
                            edit_window.hide(); 
                        } else {
                            alert(200, 200, "No se encontraron datos para editar.");
                        }
                    }
                    Err(_) => {
                        alert(200, 200, "Error al editar datos.");
                    }
                }
            } else {
                alert(200, 200, "Error al abrir la conexión");
            }
        });
    });
    
    
    app.run().unwrap();

    Ok(())
}

// funcion mostrar datos, reutilizable
fn mostrar_datos(conn: &Connection) {
    if let Ok(data) = database::mostrar_datos(conn) {
        if data.is_empty() {
            fltk::dialog::alert_default("No hay datos para mostrar");
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
        fltk::dialog::alert_default("Error al mostrar los datos");
    }
}
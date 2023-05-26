use rusqlite::{Connection, Result};

pub fn insertar_datos(
    conn: &Connection,
    calle: &str,
    numero: &str,
    piso: &str,
    codigo_postal: &str,
    metros_cuadrados: &str,
    cantidad_banios: &str,
    cantidad_habitaciones: &str,
    tipo: &str,
) -> Result<()> {
     // Convierte el valor del campo 'numero' a un entero (i32)
     let numero: i32 = match numero.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Número' debe ser un número entero.");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    };

    // Convierte el valor del campo 'piso' a un entero (i32)
    let piso: i32 = match piso.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Piso' debe ser un número entero.");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    };

    // Convierte el valor del campo 'metros_cuadrados' a un número real (f64)
    let metros_cuadrados: f64 = match metros_cuadrados.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Metros Cuadrados' debe ser un número real.");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    };

    // Convierte el valor del campo 'cantidad_banios' a un entero (i32)
    let cantidad_banios: i32 = match cantidad_banios.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Cantidad de Baños' debe ser un número entero.");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    };

    // Convierte el valor del campo 'cantidad_habitaciones' a un entero (i32)
    let cantidad_habitaciones: i32 = match cantidad_habitaciones.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Cantidad de Habitaciones' debe ser un número entero.");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    };

    // Resto del código para insertar los datos en la base de datos
    // Implementación de la función insertar_datos
    conn.execute(
        "INSERT INTO viviendas (calle, numero, piso, codigo_postal, metros_cuadrados, cantidad_banios, cantidad_habitaciones, tipo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[calle, &numero.to_string(), &piso.to_string(), codigo_postal, &metros_cuadrados.to_string(), &cantidad_banios.to_string(), &cantidad_habitaciones.to_string(), tipo],
    )?;
    Ok(())
}


pub fn mostrar_datos(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM viviendas")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,  // id
            row.get::<_, String>(1)?,  // calle
            row.get::<_, i32>(2)?,  // numero
            row.get::<_, i32>(3)?,  // piso
            row.get::<_, i32>(4)?,  // codigo_postal
            row.get::<_, f64>(5)?,  // metros_cuadrados
            row.get::<_, i32>(6)?,  // cantidad_banios
            row.get::<_, i32>(7)?,  // cantidad_habitaciones
            row.get::<_, String>(8)?,  // tipo
        ))
    })?;

    for row in rows {
        let (id, calle, numero, piso, codigo_postal, metros_cuadrados, cantidad_banios, cantidad_habitaciones, tipo) = row?;
        println!("ID: {}, Calle: {}, Número: {}, Piso: {}, Código Postal: {}, Metros Cuadrados: {}, Cantidad de Baños: {}, Cantidad de Habitaciones: {}, Tipo: {}", id, calle, numero, piso, codigo_postal, metros_cuadrados, cantidad_banios, cantidad_habitaciones, tipo);
    }

    Ok(())
}
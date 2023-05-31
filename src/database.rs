use rusqlite::{Connection, Result, params};
use crate::models::Vivienda;


// Función para insertar datos en la database
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

    let codigo_postal: i32 = match codigo_postal.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El campo 'Código Postal' debe ser un número entero.");
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
    conn.execute(
        "INSERT INTO viviendas (calle, numero, piso, codigo_postal, metros_cuadrados, cantidad_banios, cantidad_habitaciones, tipo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[calle, &numero.to_string(), &piso.to_string(), &codigo_postal.to_string(), &metros_cuadrados.to_string(), &cantidad_banios.to_string(), &cantidad_habitaciones.to_string(), tipo],
    )?;
    Ok(())
}


pub fn mostrar_datos(conn: &Connection) -> Result<Vec<Vivienda>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM viviendas")?;
    let rows = stmt.query_map([], |row| {
        Ok(Vivienda {
            calle: row.get(1)?,
            numero: row.get(2)?,
            piso: row.get(3).ok(),
            codigo_postal: row.get(4)?,
            metros_cuadrados: row.get(5)?,
            cantidad_banios: row.get(6)?,
            cantidad_habitaciones: row.get(7)?,
            tipo: row.get(8)?,
        })
    })?;

    let mut viviendas = Vec::new();
    for row in rows {
        match row {
            Ok(vivienda) => viviendas.push(vivienda),
            Err(err) => {
                println!("Error al obtener fila: {}", err);
                return Err(err);
            }
        }
    }

    Ok(viviendas)
}

// Función para borrar datos que ya se encuentran en la database
pub fn borrar_datos(conn: &Connection, calle: &str, numero: &str) -> Result<usize> {
    let affected_rows = conn.execute("DELETE FROM viviendas WHERE calle = ? AND numero = ?", params![calle, numero])?;
    Ok(affected_rows)
}

// Función para editar datos que ya se encuentran en la database
pub fn editar_datos(
    conn: &Connection,
    calle: &str,
    numero: &str,
    calle_new: &str,
    numero_new: &str,
    piso_new: &str,
    codigo_postal_new: &str,
    metros_cuadrados_new: &str,
    cantidad_banios_new: &str,
    cantidad_habitaciones_new: &str,
    tipo_new: &str,
) -> Result<usize> {
    conn.execute(
        "UPDATE viviendas 
        SET calle = ?, numero = ?, piso = ?, codigo_postal = ?, metros_cuadrados = ?, cantidad_banios = ?, cantidad_habitaciones = ?, tipo = ? 
        WHERE calle = ? AND numero = ?",
        params![
            calle_new,
            numero_new,
            piso_new,
            codigo_postal_new,
            metros_cuadrados_new,
            cantidad_banios_new,
            cantidad_habitaciones_new,
            tipo_new,
            calle,
            numero
        ],
    )
}

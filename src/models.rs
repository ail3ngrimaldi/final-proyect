pub struct Vivienda {
    pub calle: String,
    pub numero: u32,
    pub piso: Option<u32>,
    pub codigo_postal: String,
    pub metros_cuadrados: f64,
    pub cantidad_banios: u32,
    pub cantidad_habitaciones: u32,
    pub tipo: String,
}
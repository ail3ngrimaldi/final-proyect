pub struct Vivienda {
    pub calle: String,
    pub numero: u32,
    pub piso: Option<u32>,
    pub codigo_postal: i32,
    pub metros_cuadrados: f64,
    pub cantidad_banios: u32,
    pub cantidad_habitaciones: u32,
    pub tipo: String,
}


impl Vivienda {
    pub fn new(
        calle: String,
        numero: u32,
        piso: Option<u32>,
        codigo_postal: i32,
        metros_cuadrados: f64,
        cantidad_banios: u32,
        cantidad_habitaciones: u32,
        tipo: String,
    ) -> Vivienda {
        Vivienda {
            calle,
            numero,
            piso,
            codigo_postal,
            metros_cuadrados,
            cantidad_banios,
            cantidad_habitaciones,
            tipo,
        }
    }
}
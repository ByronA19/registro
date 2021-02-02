use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Usuario{
    pub Identificacion: String,
    pub Nombre: String,
    pub Genero: String,
    pub EstadoCivil: String,
    pub FechaNacimiento : String,
    pub Telefono: String,
    pub Direccion: String,
    pub Correo: String,
}

#[derive(Debug)]
pub struct Storage {
    pub database: Pool<PostgresConnectionManager<NoTls>>
}

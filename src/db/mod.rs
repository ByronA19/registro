use postgres::error::Error;
use postgres::NoTls;
use postgres::Row;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

use std::fs;
use crate::model::Usuario;

static DADO_DB: &str ="data/registro.json";

fn _registro()-> Result<Vec<Usuario>, serde_json::Error>{
    let data = fs::read_to_string(DADO_DB).expect("Error reading from file");
    let registro: Result<Vec<Usuario>, serde_json::Error> = serde_json::from_str(&data);
    registro
}
pub fn insert_registro(registro: &Usuario, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let statement = db
        .prepare(
            "insert into Usuario (Identificacion,Nombre,Genero,Estadocivil,FechaNacimiento,Telefono,Direccion,Email)",
        )?;

    db.query(&statement, &[&Usuario.Identificacion, &Usuario.Nombre,&Usuario.Genero,&Usuario.EstadoCivil,&Usuario.Telefono,&Usuario.Direccion,&Usuario.Email,&Usuario.FechaNacimiento])
}

pub fn read_registro() -> Option<Vec<Usuario>>{

    match _registro(){
        Ok(registro) => Some(registro),
        Err(_)=>None
    }

}
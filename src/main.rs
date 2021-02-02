
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use rocket::http::RawStr;
use lib::db;
use lib::model::Usuario;

//FUNCION PRINCIPAL
fn main() {
    rocket().launch();
}

//llamanda al cohete
#[get("/")]
fn  get_registro() -> Json<Option<Vec<Usuario>>> {
   Json(db::read_registro())
}


fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
    "/registroUsuario",
    routes![get_registro],


    )
}
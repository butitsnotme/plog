#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;

mod routes;
mod assets;
mod fairings;
mod guards;

fn rocket() -> rocket::Rocket {
    rocket::Rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::get_css_main,
            routes::get_css_reset,
            routes::get_css_style,
            routes::get_js_headroom,
            routes::get_js_main,
            routes::get_js_modernizr,
        ])
        .manage(guards::init_pool())
        .attach(fairings::InitDatabase)
}

fn main() {
    rocket().launch();
}

extern crate rocket;
extern crate rusqlite;

use assets;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rusqlite::Connection;

pub struct InitDatabase;

impl Fairing for InitDatabase {
	fn info(&self) -> Info {
		Info {
			name: "Database Initializer",
			kind: Kind::Attach
		}
	}
	
	fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
		let mut app_id: u32 = 0;
		let mut version: u32 = 0;
		
		let file_name: String;
		{ 
			file_name = String::from(rocket.config().get_str("database_file").unwrap_or("plog.db"));
		}
		
		let sql = Connection::open(file_name).unwrap();

		{
			let mut stmt = sql.prepare("PRAGMA application_id").unwrap();
			let mut rows = stmt.query(&[]).unwrap();
			while let Some(row) = rows.next() {
				app_id = row.unwrap().get(0);
			}
		}

		{
			let mut stmt = sql.prepare("PRAGMA user_version").unwrap();
			let mut rows = stmt.query(&[]).unwrap();
			while let Some(row) = rows.next() {
				version = row.unwrap().get(0);
			}
		}

		if 0 == app_id {
			if 0 != version {
				println!("Non-zero User Version with zero Application ID!");
				return Err(rocket);
			}
		} else if 215794801 != app_id {
			println!("This database belongs to a different application!");
			return Err(rocket);
		}
		
		if 0 == app_id && 0 == version {
			match sql.execute_batch(assets::sql::BASE_001_20180118) {
                Ok(_) => (),
                Err(_) => return Err(rocket),
            };
		}
		
		Ok(rocket)
	}
}

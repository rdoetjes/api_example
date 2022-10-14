

#[macro_use] extern crate rocket;

mod apiv1;
mod appconfig;

#[launch]
fn rocket() -> _ {
    appconfig::check_dbfile(appconfig::DATABASE);
    rocket::build().mount("/api/v1/test", routes![apiv1::test, apiv1::query])
}

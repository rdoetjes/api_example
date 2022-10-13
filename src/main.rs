
#[macro_use] extern crate rocket;

mod apiv1;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/test", routes![apiv1::test, apiv1::query])
}

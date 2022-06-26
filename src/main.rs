use rocket::{response::content, fs::{FileServer} };

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("templates/index.html"))
}
#[get("/guide")]
fn guide() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("templates/guide.html"))    
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, guide]).mount("/static", FileServer::from("/app/static"))
}
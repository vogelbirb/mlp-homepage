#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::fs::NamedFile;
use rocket::tokio::{
    task,
    time::{sleep, Duration},
};
use std::{fs, io};

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("index.html").await.ok().unwrap()
}

struct CheckKey(String);

#[async_trait]
impl<'r> FromRequest<'r> for CheckKey {
    type Error = &'static str;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("apiKey") {
            Some(key) => match key.parse::<u32>() {
                Ok(parsed_key) if parsed_key % 3 == 0 => Outcome::Success(CheckKey(key.to_string())),
                _ => Outcome::Error((Status::Forbidden, "API key is invalid.")),
            }
            None => Outcome::Error((Status::Forbidden, "API key not provided.")),
        }
    }
}

#[get("/key")]
fn key(guard: CheckKey) -> String {
    format!("Your API key is {}.", guard.0)
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[post("/world")]
fn post_world() -> &'static str {
    "POST: Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: Option<u64>) -> String {
    if let Some(seconds) = seconds {
        sleep(Duration::from_secs(seconds)).await;
        format!("Wait for {} seconds", seconds)
    } else {
        format!("Invalid seconds value")
    }
}

// Should use rocket::fs::NamedFiled or tokio::fs::file
#[get("/blocking")]
async fn blocking() -> io::Result<Vec<u8>> {
    let vec = task::spawn_blocking(|| fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![delay, blocking]).mount("/hello", routes![world])
// }

#[main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world, post_world])
        .mount("/", routes![delay, blocking, key, index])
        .launch()
        .await?;

    Ok(())
}

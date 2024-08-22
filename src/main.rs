use rocket::response::Redirect;
use rocket::{get, launch, post, routes};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use uuid::Uuid;
use std::path::Path;
use std::{fs, io, process};

mod upload;
use upload::UplaodFileForm;

pub const DATA_PATH: &str = "./data/";
const CLOUD_KEY_COOKIE_NAME: &str = "cloud_key";

fn get_cloud_key(jar: &CookieJar) -> String {
    if let Some(cookie) = jar.get(CLOUD_KEY_COOKIE_NAME) {
        cookie.value().to_string()
    } else {
        let cloud_key = Uuid::new_v4().to_string();
        jar.add(Cookie::new(CLOUD_KEY_COOKIE_NAME, cloud_key.clone()));

        cloud_key
    }
}

#[post("/send", data="<upload_form>")]
async fn upload_file(jar: &CookieJar<'_>, mut upload_form: Form<UplaodFileForm<'_>>) -> Redirect {
    let cloud_key = get_cloud_key(jar);
    
    upload_form.save_file(cloud_key).await.unwrap();

    Redirect::to("/")
}

#[get("/")]
fn index(jar: &CookieJar) -> Template {
    let cloud_key = get_cloud_key(jar);

    Template::render("index", context! {cloud_key: cloud_key})
}


#[get("/files")]
fn get_files(jar: &CookieJar) -> Template {
    let cloud_key = get_cloud_key(jar);

    Template::render("files", context! {cloud_key: cloud_key})
}


fn init_cloud() -> io::Result<()> {
    let path = Path::new(DATA_PATH);
    
    if !path.exists() {
        fs::create_dir(DATA_PATH)?;
    }
    
    Ok(())
}


#[launch]
fn rocket() -> _ {
    if let Err(e) = init_cloud() {
        eprintln!("{}", e);

        process::exit(-1);
    }

    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, upload_file, get_files])
}

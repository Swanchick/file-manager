use rocket::response::Redirect;
use rocket::{get, launch, post, routes};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use uuid::Uuid;
use std::path::{Path, PathBuf};
use std::{fs, io, process};

mod upload;
use upload::UplaodFileForm;

pub const DATA_PATH: &str = "./data/";
const CLOUD_KEY_COOKIE_NAME: &str = "cloud_key";

pub fn create_cloud_folder() -> io::Result<PathBuf> {
    let path = Path::new(DATA_PATH);
    
    if !path.exists() {
        fs::create_dir(DATA_PATH).unwrap();
    }
    
    Ok(path.to_path_buf())
}

pub fn get_cloud_directory(cloud_key: &str) -> io::Result<PathBuf> {
    let path = create_cloud_folder()?;
    let new_path = Path::new(cloud_key);
    
    let full_path = path.join(new_path);
    if !full_path.exists() {
        fs::create_dir(&full_path)?;
    }

    Ok(full_path)
}


fn get_cloud_key(jar: &CookieJar) -> String {
    if let Some(cookie) = jar.get(CLOUD_KEY_COOKIE_NAME) {
        cookie.value().to_string()
    } else {
        let cloud_key = Uuid::new_v4().to_string();
        jar.add(Cookie::new(CLOUD_KEY_COOKIE_NAME, cloud_key.clone()));

        cloud_key
    }
}

fn get_cloud_files(cloud_key: &str) -> io::Result<Vec<String>> {
    let mut file_names: Vec<String> = vec![];
    
    let cloud_dir = get_cloud_directory(cloud_key)?;
    if let Ok(files) = fs::read_dir(cloud_dir) {
        for file in files {
            let file = file.unwrap();

            if let Some(file_name) = file.file_name().to_str() {
                file_names.push(file_name.to_string());
            }
        }
    }

    Ok(file_names)
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
fn get_files(jar: &CookieJar) -> Result<Template, Redirect> {
    let cloud_key = get_cloud_key(jar);
    
    if let Ok(files) = get_cloud_files(&cloud_key) {
        Ok(Template::render("files", context! {files}))
    } else {
        Err(Redirect::to("/"))
    }
}

#[get("/donwload/<file_name>")]
fn donwload_file(file_name: String) {
    
}


#[launch]
fn rocket() -> _ {
    if let Err(e) = create_cloud_folder() {
        eprintln!("{}", e);

        process::exit(-1);
    }

    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, upload_file, get_files])
}

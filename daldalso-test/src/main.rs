#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::Redirect;
use rocket::http::RawStr;

#[macro_use] extern crate rocket;

const STATE: &str = "1"; //NOTE: Value must keep changing
const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";

#[get("/")]
fn index() -> Redirect {
    Redirect::to(daldalso::oauth_url(CLIENT_ID, STATE, "http://localhost:8000/oauth"))
}

#[get("/oauth?<state>&<code>")]
fn oauth(state: &RawStr, code: &RawStr) -> String  {
    if state.as_str() != STATE { return "".to_string(); }
    println!("State: {}, Code: {}", state.as_str(), code.as_str());
    let token = daldalso::get_oauth_token(CLIENT_ID, CLIENT_SECRET, daldalso::GrantType::AuthorizationCode, Some(code.as_str())).unwrap();
    let access_token  = token.access_token().unwrap();
    println!("Access Token: {}", access_token);
    let me = daldalso::me(&access_token).unwrap();
    return format!("Name: {}, Account: {}", me.name().unwrap(), me.account().unwrap())
}

fn main() {
    rocket::ignite().mount("/", routes![index, oauth]).launch();
}
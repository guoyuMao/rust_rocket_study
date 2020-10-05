#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;
use rocket::request::Form;

#[get("/")]
fn index() -> &'static str{
    "Hello, world! this is my first Rocket Application!"
}
#[get("/index2")]
fn index2() -> &'static str{
    "Hello, world! this is my first Rocket Application!2"
}

mod other{
    #[get("/world")]
    pub fn world() -> &'static str{
        "Hello, inner world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str{
    "Hellow, outside world!"
}

#[get("/test/<name>")]
fn test(name : &RawStr) -> String{
    format!("Hello, {}!",name)
}

#[get("/test2/<name>/<age>/<cool>")]
fn test2(name:String, age: u8, cool: bool) ->String{
    if cool {
        format!("You're a cool {} year old, {}", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/user/<id>")]
fn user(id:usize) -> String{
    format!("user_usize:{}",id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id:isize) -> String{
    format!("user_isize:{}",id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id:&RawStr) -> String{
    format!("user_str:{}",id)
}

#[get("/query?wave&<name>")]
fn query(name: &RawStr) -> String{
    format!("Hello, {}!",name.as_str())
}


#[get("/querys?wave&<name>")]
fn querys(name: Option<String>) -> String{
    name.map(|name| format!("Hi, {}!", name)).unwrap_or_else(|| "Hello!".into())
}
#[derive(FromForm,Debug)]
struct User{
    name:String,
    account:usize,
}

#[get("/item?<id>&<user..>")]
fn item(id:usize, user:Form<User>) -> String{
    println!("{:#?}",user);
    format!("the id is : {}",id)
}


use rocket::response::{Flash, Redirect};
use rocket::http::Cookies;

#[derive(FromForm,Debug)]
struct Template{
    name:String,
    info:String,
}
impl Template {
    fn new(name:String,info:String) -> Template{
        Template{name,info}
    }
}

#[get("/login")]
fn login() -> String{
    format!("success")
}


#[derive(FromForm,Debug)]
struct AdminUser{
    name:String,
    account:usize,
}

#[get("/admin?<admin..>")]
fn admin_panel(admin:Form<AdminUser>) -> & 'static str{
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin?<user..>", rank = 2)]
fn admin_panel_user(user: Form<User>) -> &'static str{
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect{
    Redirect::to(uri!(login))
}

#[get("/cookies")]
fn cookies(cookies : Cookies) -> Option<String> {
    cookies.get("message").map(|value| format!("Message:{}",value))
}

// use rocket::http::{Cookie, Cookies};
// use rocket::response::{Flash, Redirect};
use rocket::http::Cookie;

/// Retrieve the user's ID, if any.
#[get("/user_id")]
fn user_id(mut cookies: Cookies) -> Option<String>{
    cookies.get_private("user_id").map(|cookie| format!("User ID: {}", cookie.value()))
}

/// Remove the `user_id` cookie.
#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    // rocket::ignite().mount("/", routes![index,index2]).launch();
    // rocket::ignite().mount("/hello", routes![hello,other::world]).launch();
    // rocket::ignite().mount("/", routes![index]).mount("/hello", routes![hello,other::world]).launch();

     rocket::ignite()  
        .mount("/public", StaticFiles::from("static")) //静态资源
        .mount("/", routes![index,index2,hello,other::world,test,test2,user,user_int,user_str,query,querys,item])
        .launch();
}



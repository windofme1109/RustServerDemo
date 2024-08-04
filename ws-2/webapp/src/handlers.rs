use actix_web::{web, get, HttpResponse, post};

// awc 基本说明：https://github.com/actix/actix-web/blob/master/awc/README.md
use awc;

// tera 使用文档：https://keats.github.io/tera/docs/
use tera::Context;
use tera::Tera;

use crate::models::{Teacher, TeacherRegisterForm};

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {

    let mut cxt = Context::new();
    let template= tmpl.render("index.html", &cxt).unwrap();

    println!("{:?}", template);

    HttpResponse::Ok().content_type("text/html").body(template)
}


#[get("/teachers")]
pub async fn get_all_teachers(tmpl: web::Data<Tera>) -> HttpResponse {

    // 借助 awc 客户端发起 http 请求，获取数据
    // create client
    let mut client = awc::Client::default();

    // construct request
    let req = client.get("http://localhost:10000/teacher/");

    // send request and await response
    let res =
        req
            .send()
            .await
            .unwrap()
            .json::<Vec<Teacher>>()
            .await
            .unwrap();

    println!("Response: {:?}", res);

    let mut ctx = Context::new();

    ctx.insert("teachers", &res);

    let template= tmpl.render("teachers.html", &ctx).unwrap();


    HttpResponse::Ok().content_type("text/html").body(template)
}


#[get("/register")]
pub async fn show_register_form(tmpl: web::Data<Tera>) -> HttpResponse {



    let mut ctx = Context::new();

    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    ctx.insert("error", "");

    let template= tmpl.render("register.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(template)
}


#[post("/register-post")]
pub async fn handle_register(
    tmpl: web::Data<Tera>,
    data: web::Form<TeacherRegisterForm>
) -> HttpResponse {
    // 取出数据
    let form_data = data.into_inner();
    let s;
    if form_data.name == "Dave" {

        let mut ctx = Context::new();

        ctx.insert("current_name", &form_data.name);
        ctx.insert("current_imageurl", &form_data.imageurl);
        ctx.insert("current_profile", &form_data.profile);
        ctx.insert("error", "Dave already exists");

        let template= tmpl.render("register.html", &ctx).unwrap();

       s = template;

    } else {
        s = "Register Success".to_string();
    }
    HttpResponse::Ok().content_type("text/html").body(s)
}
use actix_web::{error::Error, get, post, web, HttpResponse};

// awc 基本说明：https://github.com/actix/actix-web/blob/master/awc/README.md
use awc;

use serde_json;
use std::str;

// tera 使用文档：https://keats.github.io/tera/docs/
use tera::Context;
use tera::Tera;

use crate::models::{Teacher, TeacherRegisterForm};

use crate::errors::MyError;

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, MyError> {
    let mut cxt = Context::new();
    let template = tmpl
        .render("index.html", &cxt)
        .map_err(|err| MyError::TeraError("Template rendering error".to_string()))
        .unwrap();

    println!("{:?}", template);

    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[get("/teachers")]
pub async fn get_all_teachers(tmpl: web::Data<Tera>) -> Result<HttpResponse, MyError> {
    // 借助 awc 客户端发起 http 请求，获取数据
    // create client
    let mut client = awc::Client::default();

    // construct request
    let req = client.get("http://localhost:10000/teacher/");

    // send request and await response
    let res = req
        .send()
        .await
        .unwrap()
        .json::<Vec<Teacher>>()
        .await
        .unwrap();

    println!("Response: {:?}", res);

    let mut ctx = Context::new();

    ctx.insert("teachers", &res);

    let template = tmpl
        .render("teachers.html", &ctx)
        .map_err(|err| MyError::TeraError("Template rendering error".to_string()))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[get("/register")]
pub async fn show_register_form(tmpl: web::Data<Tera>) -> Result<HttpResponse, MyError> {
    let mut ctx = Context::new();

    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    ctx.insert("error", "");

    let template = tmpl
        .render("register.html", &ctx)
        .map_err(|err| MyError::TeraError("Template rendering error".to_string())).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[post("/register-post")]
pub async fn handle_register(
    tmpl: web::Data<Tera>,
    data: web::Form<TeacherRegisterForm>,
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

        let template = tmpl.render("register.html", &ctx).unwrap();

        s = template;
    } else {
        // s = "Register Success".to_string();

        // let teacher_data = TeacherRegisterForm {
        //     name: form_data.name,
        //     imageurl: form_data.imageurl,
        //     profile: form_data.profile
        // };

        // 转换为 json

        let teacher_body = serde_json::json!({
            "name": &form_data.name,
            "picture_url": &form_data.imageurl,
            "profile": &form_data.profile
        });

        let mut client = awc::Client::default();

        println!("new_teacher {:?}", teacher_body);

        // 发送 post 请求

        // let res = client.post("http://localhost:10000/teacher/")
        //     .send_json(&teacher_body)
        //     .await
        //     .unwrap()
        //     .json::<Teacher>()
        //     .await
        //     .unwrap();

        // 借助 body方法得到字节（u8 类型）形式的返回值
        let origin_res = client
            .post("http://localhost:10000/teacher/")
            .send_json(&teacher_body)
            .await
            .unwrap()
            .body()
            .await
            .unwrap();

        // 将返回值转换为 json
        let res = serde_json::from_str::<Teacher>(&str::from_utf8(&origin_res).unwrap()).unwrap();

        s = format!("Register success, the new teacher id is {} !", res.id);
    }
    HttpResponse::Ok().content_type("text/html").body(s)
}

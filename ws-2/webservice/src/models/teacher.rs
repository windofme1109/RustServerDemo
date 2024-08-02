
use serde::{Serialize, Deserialize};
use sqlx;
use actix_web::web;


// https://doc.rust-lang.org/stable/reference/attributes/derive.html
// 使用 derive 属性给 Course 实现 Debug、Clone、Serialize、sqlx::FromRow 这个几个 trait
// 编译器自动生成这些 trait 的基本实现
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Teacher {
    // 描述从数据库中获取的基本的教师信息
    pub id: Option<i32>,
    pub name: String,
    pub picture_url: String,
    pub profile: String,

}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTeacher {

    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTeacher {
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,

}

// web::Json 是 actix_web 提供的 excutor，也就是请求的 body 数据（json）
// 这里为 CreateTeacher 实现 from trait，目的是将 json 转换为 CreateTeacher 类型
impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone()
        }
    }
}


// 
impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone()
        }
    }
}
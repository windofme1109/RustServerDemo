
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx;
use actix_web::web;

// https://doc.rust-lang.org/stable/reference/attributes/derive.html
// 使用 derive 属性给 Course 实现 Debug、Clone、Serialize、sqlx::FromRow 这个几个 trait
// 编译器自动生成这些 trait 的基本实现
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Course {
    // 描述从数据库中获取的基本的课程信息

    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourse {
    // 创建课程，只需要创建指定 teacher_id 和 name 即可

    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,

}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCourse {
    // 更新课程，所有字段均是可选

    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,

}


impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            time: course.time.clone(),
            description:course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price.clone(),
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            
            time: course.time.clone(),

            description:course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price.clone(),
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
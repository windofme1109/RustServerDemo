use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use actix_web::web::Json;
use crate::errors::MyError;

// #[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
// 去除反序列 Deserialize 这个 trait
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    // pub teacher_id: usize,
    // pub id: Option<usize>,
    pub teacher_id: i32,
    // pub id: Option<i32>,
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

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure:Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    // pub teacher_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure:Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>
}


impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone()
        }
    }
}

// impl From<web::Json<Course>> for Course {
//     fn from(course: web::Json<Course>) -> Self {
//         Course {
//             teacher_id: course.teacher_id,
//             id: course.id,
//             name: course.name.clone(),
//             time: course.time
//         }
//     }
// }


// 给 CreateCourse 实现 From trait，因为 CreateCourse 是创建课程时使用的
// 需要使用 From trait 进行转化
// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course: web::Json<CreateCourse>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone()
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {

    type Error = MyError;
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(
            CreateCourse {
                teacher_id: course.teacher_id,
                name: course.name.clone(),
                description: course.description.clone(),
                format: course.format.clone(),
                structure: course.structure.clone(),
                duration: course.duration.clone(),
                price: course.price,
                language: course.language.clone(),
                level: course.level.clone()
            }
        )
    }
}
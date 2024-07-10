use std::fmt::format;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

use crate::models::course::Course;
use crate::models::course::CreateCourse;
use crate::models::course::UpdateCourse;
use crate::errors::MyError;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {

    // let rows = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time
    //     FROM course
    //     WHERE teacher_id = $1"#,
    //     teacher_id
    // ).fetch_all(pool).await.unwrap();

    // 使用 ? 捕捉错误
    // let rows = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time
    //     FROM course
    //     WHERE teacher_id = $1"#,
    //     teacher_id
    // ).fetch_all(pool).await?;


    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1"#,
        teacher_id
    ).fetch_all(pool).await?;


    Ok(rows)

    // let courses: Vec<Course> = rows.iter()
    //     .map(|r| Course {
    //         id: Some(r.id),
    //         teacher_id: r.teacher_id,
    //         time: Some(NaiveDateTime::from(r.time.unwrap())),
    //         name: r.name.clone()
    //     })
    //     .collect();
    //
    // match courses.len() {
    //     0 => Err(MyError::NotFound("Courses not found for teacher".into())),
    //     _ => Ok(courses)
    // }

}

pub async fn get_course_details_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    // let row = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time
    //     FROM course
    //     WHERE teacher_id = $1 and id = $2"#,
    //     teacher_id,
    //     course_id
    // ).fetch_one(pool).await.unwrap();

    // let row = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time
    //     FROM course
    //     WHERE teacher_id = $1 and id = $2"#,
    //     teacher_id,
    //     course_id
    // ).fetch_one(pool).await;


    let row: Option<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;

   if let Some(course) = row {
       Ok(course)
   } else {
       Err(MyError::NotFound("Course Id not found".into()))
   }


}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {

    // let row = sqlx::query!(
    //     r#"INSERT INTO course (id, teacher_id, name)
    //     VALUES ($1, $2, $3)
    //    RETURNING id, teacher_id, name, time"#,
    //     new_course.id,
    //     new_course.teacher_id,
    //     new_course.name,
    //
    // ).fetch_one(pool).await.unwrap();

    // let row = sqlx::query!(
    //     r#"INSERT INTO course (id, teacher_id, name)
    //     VALUES ($1, $2, $3)
    //    RETURNING id, teacher_id, name, time"#,
    //     new_course.id,
    //     new_course.teacher_id,
    //     new_course.name,
    //
    // ).fetch_one(pool).await?;

    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
       RETURNING id, teacher_id, name, description, format, structure, duration, price, language, level, time"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,

    )
        .fetch_one(pool)
        .await?;

    // Ok(Course {
    //     id: Some(row.id),
    //     teacher_id: row.teacher_id,
    //     time: Some(NaiveDateTime::from(row.time.unwrap())),
    //     name: row.name.clone()
    // })

    Ok(row)
}


pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM course WHERE teacher_id = $1 AND id = $2",
        teacher_id,
        id
    ).execute(pool).await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(pool: &PgPool, teacher_id: i32, id: i32, update_course: UpdateCourse) -> Result<Course, MyError> {

    // 先根据 teacher_id 和 id 进行查询，得到一条数据
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = $1 AND id = $2",
        teacher_id,
        id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err|  MyError::NotFound("Course Id not found".into()))?;


    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    let course_row = sqlx::query_as!(
        Course,
        r#"UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
       WHERE teacher_id = $9 and id = $10
       RETURNING id, teacher_id, name, description, format, structure, duration, price, language, level, time"#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id

    )
        .fetch_one(pool)
        .await;


    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

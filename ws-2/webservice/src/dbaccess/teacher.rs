use crate::errors::MyError;
use crate::models::course::Course;
use crate::models::teacher::CreateTeacher;
use crate::models::teacher::UpdateTeacher;
use crate::models::teacher::Teacher;
use chrono::NaiveDateTime;
use sqlx;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;


pub async fn post_new_teacher_db(pool: &PgPool, new_teacher: CreateTeacher) -> Result<Teacher, MyError> {
    let teacher = sqlx::query_as!(
        Teacher,
        r#"
            INSERT INTO teacher (name, picture_url, profile)
            VALUES ($1, $2, $3)
            RETURNING id, name, picture_url, profile
        "#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    ).fetch_one(pool).await?;

    Ok(teacher)
}

/// 新增课程
pub async fn get_all_teachers_db(
    pool: &PgPool,
) -> Result<Vec<Teacher>, MyError> {
    // 我们为 sqlx 的 error 类型实现了 From trait
    // 如果这里是 sqlx 报错，可以直接将错误类型转换为 MyError

    // 这里使用 ? 运算符取代 unwrap
    // 在函数中使用 ? 运算符，该运算符尝试从 Result 中获取值：
    // 成功，拿到 Ok 变体中的值
    // 失败：接收 Error，并终止函数执行，并把错误传播到调用该函数的函数

    let rows = sqlx::query!(
        r#"SELECT  id, name, picture_url, profile FROM teacher"#,
        )
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows.iter().map(|row| {
        Teacher {
            id: row.id,
            name: row.name.clone(),
            picture_url: row.picture_url.clone(),
            profile: row.profile.clone()
        }
    }).collect();

    Ok(teachers)

}

/// 根据课程 id 获取课程详情
pub async fn get_teacher_detail_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Teacher, MyError> {
   
    // 使用 query_as 进行查询，指定获取数据的类型，前提是：Course 要实现 sqlx::FromRow trait
    // ::<_, Course> 被称为是 turbofish 语法，用来指定具体的泛型参数类型
    
    let row = sqlx::query!(
        r#"SELECT * FROM teacher WHERE id = $1"#,
        teacher_id
    ).fetch_one(pool).await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile
    })
}

pub async fn update_teacher_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    // 先根据 teacher_id 和 course_id 查出数据
    let current_teacher_row = sqlx::query!(
        r#"SELECT * FROM teacher WHERE id = $1"#,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|row| Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile
    })
    .map_err(|err| {
        MyError::NotFound("Not Found Teacher".to_string())
    }).unwrap();

    // 组合数据
    let name = update_teacher.name;

    let picture_url = if let Some(picture_url) = update_teacher.picture_url {
        picture_url
    } else {
        current_teacher_row.picture_url
    };
    let profile = if let Some(profile) = update_teacher.profile {
        profile
    } else {
        current_teacher_row.profile
    };

    // 因为想获取更新后的数据，所以需要使用 fetch_one finalizer 接收获取到的结果
    let teacher =  sqlx::query_as!(
        Teacher,
        r#"
        UPDATE teacher SET name = $1, picture_url = $2, profile = $3 WHERE id = $4
       RETURNING id, name, picture_url, profile
        "#,
        name,
        picture_url,
        profile,
        teacher_id
    )
    .fetch_one(pool)
    .await?;

    Ok(teacher)

}



pub async fn delete_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<String, MyError>{
    
    println!("teacher id {}", teacher_id);

   let affect_rows = sqlx::query!(
        r#"
        DELETE FROM teacher WHERE id = $1
        "#,
        teacher_id
    )
    .execute(pool)
    .await
    .map_err(|err| {
        MyError::DBError("Unable Delete teacher".to_string())
    })?;

    Ok(format!("Deleted {} record", affect_rows.rows_affected()))
}

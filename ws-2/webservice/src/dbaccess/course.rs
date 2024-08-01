use crate::errors::MyError;
use crate::models::course::Course;
use crate::models::course::CreateCourse;
use crate::models::course::UpdateCourse;
use chrono::NaiveDateTime;
use sqlx;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;

/// 新增课程
pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    // 我们为 sqlx 的 error 类型实现了 From trait
    // 如果这里是 sqlx 报错，可以直接将错误类型转换为 MyError

    // 这里使用 ? 运算符取代 unwrap
    // 在函数中使用 ? 运算符，该运算符尝试从 Result 中获取值：
    // 成功，拿到 Ok 变体中的值
    // 失败：接收 Error，并终止函数执行，并把错误传播到调用该函数的函数

    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, teacher_id, name, description, format, structure, duration, price, language, level, time
        "#,
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

    Ok(row)
}

/// 根据课程 id 获取课程详情
pub async fn get_course_detail_db(
    pool: &PgPool,
    course_id: i32,
    teacher_id: i32,
) -> Result<Course, MyError> {
    // println!("get course detail");
    // println!("course_id {}", course_id);
    // println!("teacher id {}", teacher_id);

    // 使用 query 方法进行查询
    // let detail = sqlx::query("SELECT * FROM course WHERE id = $1 AND teacher_id = $2")
    // .bind(course_id)
    // .bind(teacher_id)
    // .fetch_one(pool)
    // .await?;

    // let course = Course {
    //     id: detail.get("id"),
    //     teacher_id: detail.get("teacher_id"),
    //     name: detail.get("name"),
    //     time: Some(NaiveDateTime::from(detail.get::<NaiveDateTime, _>("time"))),

    //     description: detail.get("description"),
    //     format: detail.get("format"),
    //     structure: detail.get("structure"),
    //     duration: detail.get("duration"),
    //     price:detail.get("price"),
    //     language: detail.get("language"),
    //     level:detail.get("level")
    // };

    // Ok(course)

    // 使用 map 方法进行类型映射
    // let detail = sqlx::query("SELECT * FROM course WHERE id = $1 AND teacher_id = $2")
    //     .bind(course_id)
    //     .bind(teacher_id)
    //     .map(|row: PgRow| Course {
    //         id: row.get("id"),
    //         teacher_id: row.get("teacher_id"),
    //         name: row.get("name"),
    //         time: Some(NaiveDateTime::from(row.get::<NaiveDateTime, _>("time"))),

    //         description: row.get("description"),
    //         format: row.get("format"),
    //         structure: row.get("structure"),
    //         duration: row.get("duration"),
    //         price: row.get("price"),
    //         language: row.get("language"),
    //         level: row.get("level"),
    //     })
    //     .fetch_one(pool)
    //     .await?;

    // Ok(detail)

    // 使用 query_as 进行查询，指定获取数据的类型，前提是：Course 要实现 sqlx::FromRow trait
    // ::<_, Course> 被称为是 turbofish 语法，用来指定具体的泛型参数类型
    let course =
        sqlx::query_as::<_, Course>("SELECT * FROM course WHERE id = $1 AND teacher_id = $2")
            .bind(course_id)
            .bind(teacher_id)
            .fetch_one(pool)
            .await?;

    Ok(course)
}

pub async fn update_course_detail_db(
    pool: &PgPool,
    course_id: i32,
    teacher_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    // 先根据 teacher_id 和 course_id 查出数据
    let current_course_row = sqlx::query!(
        r#"SELECT * FROM course WHERE id = $1 AND teacher_id = $2"#,
        course_id,
        teacher_id
    )
    .map(|row| Course {
        id: row.id,
        teacher_id: row.teacher_id,
        name: row.name,
        time: row.time,
        description: row.description,
        format: row.format,
        structure: row.structure,
        duration: row.duration,
        price: row.price,
        language: row.language,
        level: row.level,
    })
    .fetch_one(pool)
    .await
    .map_err(|err| MyError::NotFound("Course not found".to_string()))?;

    // 组合数据

    let name = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };

    let time = if let Some(time) = update_course.time {
        time
    } else {
        current_course_row.time.unwrap_or_default()
    };
    let description = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
 
    let language = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let level = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    // 更新数据
    // 使用 executor finalizer 获取影响的行数
    // 这里就是更新的行数
    // let affect_rows =  sqlx::query(
    //     r#"
    //     UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
    //    WHERE teacher_id = $9 and id = $10
    //    RETURNING id, teacher_id, name, description, format, structure, duration, price, language, level, time
    //     "#
    // )
    // .bind(name)
    // .bind(description)
    // .bind(format)
    // .bind(structure)
    // .bind(duration)
    // .bind(price)
    // .bind(language)
    // .bind(level)
    // .bind(teacher_id)
    // .bind(course_id)
    // .execute(pool)
    // .await?;

    // println!("affected Rows {}", affect_rows.rows_affected());

    // 因为想获取更新后的数据，所以需要使用 fetch_one finalizer 接收获取到的结果
    let course =  sqlx::query(
        r#"
        UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
       WHERE teacher_id = $9 and id = $10
       RETURNING id, teacher_id, name, description, format, structure, duration, price, language, level, time
        "#
    )
    .bind(name)
    .bind(description)
    .bind(format)
    .bind(structure)
    .bind(duration)
    .bind(price)
    .bind(language)
    .bind(level)
    .bind(teacher_id)
    .bind(course_id)
    .map(|row: PgRow| {
        Course {
            id: row.get("id"),
            teacher_id: row.get("teacher_id"),
            name: row.get("name"),
            time: Some(NaiveDateTime::from(row.get::<NaiveDateTime, _>("time"))),

            description: row.get("description"),
            format: row.get("format"),
            structure: row.get("structure"),
            duration: row.get("duration"),
            price: row.get("price"),
            language: row.get("language"),
            level: row.get("level"),
        }
    })
    .fetch_one(pool)
    .await?;

    Ok(course)

    // Ok(Course {
    //     id: 1,
    //     teacher_id: 1,
    //     name: "abc".to_string(),
    //     time: None,

    //     description: None,
    //     format: None,
    //     structure: None,
    //     duration: None,
    //     price: None,
    //     language: None,
    //     level: None,
    // })
}

pub async fn delete_course_db(
    pool: &PgPool,
    course_id: i32,
    teacher_id: i32,
) -> Result<String, MyError>{
    // 根据 course_id 和 teacher_id 删除课程
    // 因为是删除数据，所以使用 execute finalizer 获取受到影响的行数
    let affect_rows =  sqlx::query(
        r#"
        DELETE FROM course WHERE teacher_id = $1 AND id = $2
        "#
    )
    .bind(teacher_id)
    .bind(course_id)
    .execute(pool)
    .await?;

    println!("Delete {:?} rows", affect_rows);

    Ok(format!("Deleted {} record", affect_rows.rows_affected()))
}

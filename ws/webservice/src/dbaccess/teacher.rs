use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;
pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT id, name, picture_url, profile FROM teacher").fetch_all(pool).await?;

    let teachers: Vec<Teacher>= rows.iter().map(|r| Teacher {
        id: r.id,
        name: r.name.clone(),
        picture_url: r.picture_url.clone(),
        profile:r.nrofile.clone()
    });

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers)
    }

}

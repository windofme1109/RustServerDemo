use std::sync::Mutex;

use crate::state::AppState;
use actix_web::{http::StatusCode, web::{self, Json}, App, HttpResponse};
use chrono::Utc;
use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::dbaccess::course::*;
use actix_web::ResponseError;



pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    println!("Receive new course");
    // CreateCourse 实现的是 Try_From trait。所以这里使用 try_into 实现转换，使用 ? 运算符处理可能出错的情况
    post_new_course_db(&app_state.db, new_course.try_into()?).await.map(|course| HttpResponse::Ok().json(course))


    // let course = post_new_course_db(&app_state.db, new_course.into()).await;
    // HttpResponse::Ok().json(course)
    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| {course.teacher_id == new_course.teacher_id})
    //     .collect::<Vec<Course>>()
    //     .len();
    //
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };
    //
    // app_state.courses.lock().unwrap().push(new_course);
    //
    // HttpResponse::Ok().json("Course Add")



}


pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    // params: web::Path<(usize, )>
    params: web::Path<i32>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))

    // url params: //xxx/{teacher_id} 从 url 参数上获取
    // let teacher_id = i32::try_from(params.0).unwrap();

    // get_courses_for_teacher_db 返回值是 Result<Vec<Course>, MyError>
    // 如果正常返回，那么直接调用 map 进行迭代
    // 如果发生了错误，返回的是 MyError 类型，因为 MyError 实现了 ResponseError 这个 trait
    // 所以 Actix 会自动将其转换为错误类型对应的 http 响应
    // get_courses_for_teacher_db(&app_state.db, teacher_id)
    //     .await
    //     .map(|course| HttpResponse::Ok().json(course))

    // let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;
    //
    // HttpResponse::Ok().json(courses)

    // let teacher_id: usize = params.0;
    //
    // let filtered_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| {course.teacher_id == teacher_id})
    //     .collect::<Vec<Course>>();
    //
    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    //     HttpResponse::Ok().json("No courses found for teacher".to_string())
    // }


}


pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    // params: web::Path<(usize, usize)>
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {


    // let teacher_id = i32::try_from(params.0).unwrap();
    // let course_id = i32::try_from(params.1).unwrap();
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id).await.map(|course| HttpResponse::Ok().json(course))

    // let courses = get_course_details_db(&app_state.db, teacher_id, course_id).await;
    //
    // HttpResponse::Ok().json(courses)
    // let (teacher_id, course_id) = params.0;
    //
    // let selected_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.teacher_id ==teacher_id && x.id == Some(course_id))
    //     .ok_or("Course not found");
    //
    // if let Ok(course) = selected_courses {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     HttpResponse::Ok().json("No course detail found".to_string())
    // }

}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {

    let (teacher_id, course_id) = params.into_inner();

    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))

}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {

    let (teacher_id, course_id) = params.into_inner();

    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|resp| HttpResponse::Ok().json(resp))

}




#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use actix_web::cookie::time::format_description::parse;
    use sqlx::ColumnIndex;
    use sqlx::encode::IsNull::No;

    #[ignore]
    #[actix_rt::test]
    async fn post_course_test() {
        // let course = web::Json(Course {
        //     teacher_id: 1,
        //     name: "Test course".into(),
        //     id: None,
        //     time: None
        // });
        //
        //
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![])
        // });
        //
        // let resp = new_course(course, app_state).await;
        //
        // assert_eq!(resp.status(), StatusCode::OK)

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });


        let course = web::Json(
            CreateCourse {
                teacher_id: 1,
                name: "Test course".into(),
                // id: None,
                // id: Some(3), // 手动设置 id，防止新建课程失败
                // time: None,
                description: Some("This is a course".into()),
                format: None,
                structure: None,
                duration: None,
                price: None,
                language: Some("English".into()),
                level: Some("Beginner".into())
            }
        );

        let resp = post_new_course(course, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)

    }


    #[actix_rt::test]
    async fn get_all_courses_success() {
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![])
        // });
        //
        // let teacher_id:web::Path<usize> = web::Path::from(1);
        //
        // let resp = get_courses_for_teacher(app_state, teacher_id).await;
        //
        // assert_eq!(resp.status(), StatusCode::OK)
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let teacher_id: web::Path<i32> = web::Path::from((1));

        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_one_courses_success() {
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![])
        // });
        //
        // let params:web::Path<(usize, usize)> = web::Path::from((1, 1));
        //
        // let resp = get_course_detail(app_state, params).await;
        //
        // assert_eq!(resp.status(), StatusCode::OK)

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params:web::Path<(i32, i32)> = web::Path::from((1, 1));

        let resp = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }


    #[actix_rt::test]
    async fn get_one_courses_failure() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params:web::Path<(i32, i32)> = web::Path::from((1, 101));

        let resp = get_course_detail(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong ..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }


    #[actix_rt::test]
    async fn update_courses_success() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });
        
        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None
        };

        let params:web::Path<(i32, i32)> = web::Path::from((1, 2));

        let update_params = web::Json(update_course);

        let resp = update_course_details(app_state, update_params, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }


    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params:web::Path<(i32, i32)> = web::Path::from((1, 3));

        let resp = delete_course(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }



    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params:web::Path<(i32, i32)> = web::Path::from((1, 101));

        let resp = delete_course(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        };

    }
}
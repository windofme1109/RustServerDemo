use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm  {
    pub name: String,
    pub imageurl: String,
    pub profile: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: string,
    pub picture_url: String,
    pub profile: string,
}


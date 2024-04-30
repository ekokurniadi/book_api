use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody<T=Option<String>> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub body: ResponseBody<T>,
}

use crate::{
    connection,
    repository::{self, books::IBookRepository},
    shared::response::ResponseBody,
};

#[get("/books")]
pub async fn get_all() -> String {
    let mut db = connection::conn::config_db();
    let mut book_repo = repository::books::BookRepository::new(&mut db);

    let result = book_repo.get_all().await;

    match result {
        Ok(res) => {
            let response = ResponseBody {
                status: rocket::http::Status::Ok.code,
                message: "Sukses".to_string(),
                data: res,
            };

            serde_json::to_string(&response).unwrap()
        }
        Err(e) => {
            let response = ResponseBody {
                status: rocket::http::Status::InternalServerError.code,
                message: e.to_string(),
                data: Some("".to_string()),
            };

            serde_json::to_string(&response).unwrap()
        }
    }
}

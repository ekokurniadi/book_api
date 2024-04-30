use crate::models::book::Books;
use crate::schema::book as books;
use diesel::{prelude::*, result::Error};

pub trait IBookRepository {
    fn get_all(&mut self) -> impl std::future::Future<Output = Result<Vec<Books>, Error>> + Send;
}

pub struct BookRepository<'a> {
    db: &'a mut PgConnection,
}

impl<'a> BookRepository<'a> {
    pub fn new(db: &'a mut PgConnection) -> Self {
        BookRepository { db: db }
    }
}

impl<'a> IBookRepository for BookRepository<'a> {
    async fn get_all(&mut self) -> Result<Vec<Books>, Error> {
        use books::dsl::*;

        let data = book.select(Books::as_select()).load::<Books>(self.db);

        let val = match data {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        };

        val
    }
}

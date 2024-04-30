use diesel::prelude::*;
use crate::models::user::Users;
use crate::schema::users as user;

pub trait IUserRepository {
    fn get_all(&mut self) -> impl std::future::Future<Output = ()> + Send;
}

pub struct UsersRepository<'a> {
    db: &'a mut PgConnection,
}

impl<'a> UsersRepository<'a> {
    pub fn new(db: &'a mut PgConnection) -> Self {
        UsersRepository { db: db }
    }
}

impl<'a> IUserRepository for UsersRepository<'a> {
    async fn get_all(&mut self) {
        use user::dsl::*;

        let result = users
            .select(Users::as_select())
            .load::<Users>(self.db)
            .expect("Error loading");

        for r in result {
            println!("{:?}", r)
        }
    }
}

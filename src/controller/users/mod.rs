use crate::repository::users::{IUserRepository, UsersRepository};

pub trait IUserController {
    fn get_all(&mut self) -> impl std::future::Future<Output = ()> + Send;
}

pub struct UsersController<'a> {
    pub user_repo: &'a mut UsersRepository<'a>,
}

impl<'a> IUserController for UsersController<'a> {
    async fn get_all(&mut self) {
        self::UsersRepository::get_all(self.user_repo).await;
    }
}

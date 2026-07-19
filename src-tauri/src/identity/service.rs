use super::{
    models::User,
    repository::IdentityRepository,
    error::IdentityError,
};

pub struct IdentityService<R>
where
    R: IdentityRepository
{
    repository: R,
}

impl<R> IdentityService<R>
where
    R: IdentityRepository
{
    pub fn new(repository: R) -> Self {
        Self {
            repository
        }
    }
}

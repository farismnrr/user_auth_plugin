use actix_web::web;
use std::sync::Arc;
use crate::domains::user::usecases::user_usecase::UserUseCase;
use crate::domains::user::usecases::user_details_usecase::UserDetailsUseCase;
use crate::domains::user::repositories::user_repository::UserRepository;
use crate::domains::user::repositories::user_details_repository::UserDetailsRepository;
use crate::domains::tenant::repositories::user_tenant_repository::UserTenantRepository;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;

pub struct UserModule;

impl UserModule {
    pub fn configure(
        cfg: &mut web::ServiceConfig,
        db: &sea_orm::DatabaseConnection,
        cache: &Arc<RocksDbCache>,
    ) {
        let user_repo = Arc::new(UserRepository::new(Arc::new(db.clone()), cache.clone()));
        let user_details_repo = Arc::new(UserDetailsRepository::new(Arc::new(db.clone()), cache.clone()));
        let user_tenant_repo = Arc::new(UserTenantRepository::new(Arc::new(db.clone()), cache.clone()));

        let user_usecase = Arc::new(UserUseCase::new(
            user_repo.clone(),
            user_details_repo.clone(),
            user_tenant_repo.clone(),
        ));

        let user_details_usecase = Arc::new(UserDetailsUseCase::new(
            user_details_repo.clone(),
        ));

        crate::domains::user::routes::user_routes::configure_user_routes(cfg, user_usecase, user_details_usecase);
    }
}

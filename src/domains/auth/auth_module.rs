use actix_web::web;
use std::sync::Arc;
use crate::domains::auth::usecases::auth_usecase::AuthUseCase;
use crate::domains::user::repositories::user_repository::UserRepository;
use crate::domains::user::repositories::user_details_repository::UserDetailsRepository;
use crate::domains::user::repositories::user_session_repository::UserSessionRepository;
use crate::domains::user::repositories::user_activity_log_repository::UserActivityLogRepository;
use crate::domains::tenant::repositories::user_tenant_repository::UserTenantRepository;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;

pub struct AuthModule;

impl AuthModule {
    pub fn configure(
        cfg: &mut web::ServiceConfig,
        db: &sea_orm::DatabaseConnection,
        cache: &Arc<RocksDbCache>,
    ) {
        let user_repo = Arc::new(UserRepository::new(Arc::new(db.clone()), cache.clone()));
        let user_details_repo = Arc::new(UserDetailsRepository::new(Arc::new(db.clone()), cache.clone()));
        let user_session_repo = Arc::new(UserSessionRepository::new(Arc::new(db.clone())));
        let user_activity_log_repo = Arc::new(UserActivityLogRepository::new(Arc::new(db.clone())));
        let user_tenant_repo = Arc::new(UserTenantRepository::new(Arc::new(db.clone()), cache.clone()));

        let auth_usecase = Arc::new(AuthUseCase::new(
            user_repo.clone(),
            user_details_repo.clone(),
            user_tenant_repo.clone(),
            user_session_repo.clone(),
            user_activity_log_repo.clone(),
        ));

        crate::domains::auth::routes::auth_routes::configure_routes(cfg, auth_usecase);
    }
}

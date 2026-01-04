use actix_web::web;
use std::sync::Arc;
use crate::domains::tenant::usecases::tenant_usecase::TenantUseCase;
use crate::domains::tenant::repositories::tenant_repository::TenantRepository;

use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;

pub struct TenantModule;

impl TenantModule {
    pub fn configure(
        cfg: &mut web::ServiceConfig,
        db: &sea_orm::DatabaseConnection,
        cache: &Arc<RocksDbCache>,
    ) {
        let tenant_repo = Arc::new(TenantRepository::new(Arc::new(db.clone()), cache.clone()));


        let tenant_usecase = Arc::new(TenantUseCase::new(
            tenant_repo.clone(),
        ));

        crate::domains::tenant::routes::tenant_routes::configure_tenant_routes(cfg, tenant_usecase);
    }
}

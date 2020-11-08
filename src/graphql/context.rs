use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;

pub type DataPgPooledConnection = actix_web::web::Data::<r2d2::Pool<ConnectionManager<PgConnection>>>;
pub struct Context {
    pub connection: DataPgPooledConnection,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

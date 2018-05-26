use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// An alias to the type for a pool of Diesel SQLite connections.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    Pool::new(manager).expect("db pool")
}

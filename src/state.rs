use std::env;

use actix::{
    Addr,
    Actor,
    SyncContext,
    SyncArbiter,
};
use diesel::{
    pg::PgConnection,
    r2d2::{
        ConnectionManager,
        Pool,
        PoolError,
        PooledConnection,
    },
};

const DB_THREADS: usize = 4;

type PgPool =
    Pool<ConnectionManager<PgConnection>>;

type PgPooledConnection =
    PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(db_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}

struct Db(PgPool);

impl Actor for Db {
    type Context = SyncContext<Self>;
}

pub struct State(Addr<Db>);

impl State {
    pub fn init() -> State {
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let pool = init_pool(&db_url)
            .expect("Failed to create database pool");

        let addr = SyncArbiter::start(DB_THREADS, move ||
            Db(pool.clone())
        );

        State(addr)
    }
}




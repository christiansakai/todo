use std::env;
use std::ops::Deref;

use actix::{
    Addr,
    Actor,
    Handler,
    Message,
    SyncContext,
    SyncArbiter,
};
use actix_web::{error, Error};
use diesel::{
    pg::PgConnection,
    r2d2::{
        ConnectionManager,
        Pool,
        PoolError,
        PooledConnection,
    },
};

use crate::models::{
    Todo,
    AddTodo,
    RemoveTodo,
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

impl Db {
    fn get_conn(&self) -> Result<PgPooledConnection, Error> {
        self.0
            .get()
            .map_err(|e| error::ErrorInternalServerError(e))
    }
}

impl Actor for Db {
    type Context = SyncContext<Self>;
}

struct GetTodosMsg;

impl Message for GetTodosMsg {
    type Result = Result<Vec<Todo>, Error>;
}

impl Handler<GetTodosMsg> for Db {
    type Result = Result<Vec<Todo>, Error>;

    fn handle(&mut self, _: GetTodosMsg, _: &mut Self::Context) -> Self::Result {
        Todo::get_todos(self.get_conn()?.deref())
            .map_err(|_|
                error::ErrorInternalServerError(
                    "Failed to retrieve todos"
                )
            )
    }
}

struct GetTodoMsg(i32);

impl Message for GetTodoMsg {
    type Result = Result<Todo, Error>;
}

impl Handler<GetTodoMsg> for Db {
    type Result = Result<Todo, Error>;

    fn handle(&mut self, msg: GetTodoMsg, _: &mut Self::Context) -> Self::Result {
        Todo::get_todo(msg.0, self.get_conn()?.deref())
            .map_err(|_|
                error::ErrorInternalServerError(
                    "Failed to retrieve todo"
                )
            )
    }
}

struct AddTodoMsg {
    pub title: String,
    pub description: String,
}

impl Into<AddTodo> for AddTodoMsg {
    fn into(self) -> AddTodo {
        AddTodo {
            title: self.title,
            description: self.description,
        }
    }
}

impl Message for AddTodoMsg {
    type Result = Result<Todo, Error>;
}

impl Handler<AddTodoMsg> for Db {
    type Result = Result<Todo, Error>;

    fn handle(&mut self, msg: AddTodoMsg, _: &mut Self::Context) -> Self::Result {
        Todo::add_todo(msg.into(), self.get_conn()?.deref())
            .map_err(|_|
                error::ErrorInternalServerError(
                    "Failed to remove todo"
                )
            )
    }
}

struct RemoveTodoMsg(i32);

impl Message for RemoveTodoMsg {
    type Result = Result<usize, Error>;
}

impl Handler<RemoveTodoMsg> for Db {
    type Result = Result<usize, Error>;

    fn handle(&mut self, msg: RemoveTodoMsg, _: &mut Self::Context) -> Self::Result {
        Todo::remove_todo(msg.0, self.get_conn()?.deref())
            .map_err(|_|
                error::ErrorInternalServerError(
                    "Failed to remove todo"
                )
            )
    }
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

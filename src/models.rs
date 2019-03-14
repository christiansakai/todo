use diesel::{
    self,
    pg::PgConnection,
    prelude::*,
};

use crate::schema::todos;

#[derive(Queryable, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct AddTodo {
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct RemoveTodo {
    pub id: i32,
}

impl Todo {
    pub fn add_todo(todo: AddTodo, conn: &PgConnection) -> QueryResult<Todo> {
        diesel::insert_into(todos::table)
            .values(&todo)
            .get_result(conn)
    }

    pub fn remove_todo(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(todos::table.filter(todos::id.eq(id)))
            .execute(conn)
    }

    pub fn get_todos(conn: &PgConnection) -> QueryResult<Vec<Todo>> {
        todos::table.order(todos::id.desc())
            .load::<Todo>(conn)
    }

    pub fn get_todo(id: i32, conn: &PgConnection) -> QueryResult<Todo> {
        todos::table.filter(todos::id.eq(id))
            .get_result(conn)
    }
}

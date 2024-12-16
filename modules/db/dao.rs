use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::query_dsl::methods::FindDsl;
use diesel::Insertable;
use diesel::Identifiable;
use diesel::Associations;
use diesel::Queryable;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::pg::Pg;

pub trait GenericDao<T> {
    fn create(conn: &PgConnection, entity: &T) -> QueryResult<T>;
    fn read(conn: &PgConnection, id: i32) -> QueryResult<T>;
    fn update(conn: &PgConnection, entity: &T) -> QueryResult<T>;
    fn delete(conn: &PgConnection, id: i32) -> QueryResult<usize>;
    fn list(conn: &PgConnection) -> QueryResult<Vec<T>>;
}

pub struct GenericDaoImpl<T, U> {
    _marker: std::marker::PhantomData<(T, U)>,
}

impl<T, U> GenericDaoImpl<T, U>
where
    T: Queryable<U, Pg> + Identifiable + AsChangeset + Insertable<U> + Clone,
    U: diesel::Table,
    T::Id: diesel::ExpressionMethods + diesel::SelectableExpression<U>,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, U> GenericDao<T> for GenericDaoImpl<T, U>
where
    T: Queryable<U, Pg> + Identifiable + AsChangeset + Insertable<U> + Clone,
    U: diesel::Table,
    T::Id: diesel::ExpressionMethods + diesel::SelectableExpression<U>,
{
    fn create(conn: &PgConnection, entity: &T) -> QueryResult<T> {
        use diesel::insert_into;
        insert_into(U::table())
            .values(entity)
            .get_result(conn)
    }

    fn read(conn: &PgConnection, id: i32) -> QueryResult<T> {
        U::table().find(id).get_result(conn)
    }

    fn update(conn: &PgConnection, entity: &T) -> QueryResult<T> {
        use diesel::update;
        update(U::table().find(entity.id()))
            .set(entity)
            .get_result(conn)
    }

    fn delete(conn: &PgConnection, id: i32) -> QueryResult<usize> {
        use diesel::delete;
        delete(U::table().find(id)).execute(conn)
    }

    fn list(conn: &PgConnection) -> QueryResult<Vec<T>> {
        U::table().load(conn)
    }
}

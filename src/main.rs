use diesel::prelude::*;
diesel::table! {
    users {
        id -> Integer,
        hair_color -> Nullable<Text>,
    }
}

#[derive(Debug, diesel::Queryable, diesel::Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct User {
    #[diesel(embed)]
    hair_color: HairColor,
}

#[derive(Debug)]
struct HairColor(Option<String>);

impl<DB> diesel::expression::Selectable<DB> for HairColor
where
    DB: diesel::backend::Backend,
{
    type SelectExpression = diesel::helper_types::Concat<
        diesel::dsl::AssumeNotNull<users::columns::hair_color>,
        diesel::internal::derives::as_expression::Bound<diesel::sql_types::Text, &'static str>,
    >;

    fn construct_selection() -> Self::SelectExpression {
        users::hair_color.assume_not_null().concat("ish")
    }
}

impl<DB> Queryable<diesel::sql_types::Nullable<diesel::sql_types::Text>, DB> for HairColor
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    type Row = Option<String>;

    fn build(url_str: Self::Row) -> diesel::deserialize::Result<Self> {
        match url_str {
            None => Ok(HairColor(None)),
            Some(url_str) => Ok(HairColor(Some(url_str.into()))),
        }
    }
}

fn main() {
    let connection = &mut diesel::sqlite::SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query(
        "CREATE TABLE users (
             id INTEGER PRIMARY KEY,
             hair_color VARCHAR(255)
         )",
    )
    .execute(connection)
    .unwrap();

    dbg!(users::table
        .select(User::as_select())
        .load(connection)
        .unwrap());
}

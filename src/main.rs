use diesel::prelude::*;
use diesel::{Connection, NullableExpressionMethods, TextExpressionMethods};
diesel::table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

fn connection_no_data() -> diesel::sqlite::SqliteConnection {
    diesel::sqlite::SqliteConnection::establish(":memory:").unwrap()
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
        users::hair_color.assume_not_null().concat("hi")
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
    use diesel::insert_into;
    use diesel::prelude::*;

    let connection = &mut connection_no_data();
    diesel::sql_query(
        "CREATE TABLE users (
             id INTEGER PRIMARY KEY,
             name VARCHAR(255) NOT NULL,
             hair_color VARCHAR(255)
         )",
    )
    .execute(connection)
    .unwrap();

    insert_into(users::dsl::users)
        .values(&vec![
            (
                users::dsl::id.eq(1),
                users::dsl::name.eq("Sean"),
                users::dsl::hair_color.eq(Some("Green")),
            ),
            (
                users::dsl::id.eq(2),
                users::dsl::name.eq("Tess"),
                users::dsl::hair_color.eq(None),
            ),
        ])
        .execute(connection)
        .unwrap();

    let users = users::table
        .select(User::as_select())
        .load(connection)
        .unwrap();
    dbg!(users);
}

use diesel::{Connection, TextExpressionMethods};

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

// #[derive(Debug, diesel::Queryable, diesel::Selectable)]
// struct User {
//     id: i32,
//     name: String,
//     #[diesel(embed)]
//     hair_color: Option<HairColor>,
// }

#[derive(Debug)]
struct HairColor(Option<String>);

impl<DB> diesel::expression::Selectable<DB> for HairColor
where
    DB: diesel::backend::Backend,
{
    type SelectExpression = diesel::helper_types::Concat<
        users::columns::hair_color,
        diesel::internal::derives::as_expression::Bound<diesel::sql_types::Text, &'static str>,
    >;

    fn construct_selection() -> Self::SelectExpression {
        unimplemented!() //)users::hair_color.concat("withdraw/wallet/")
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

    // let users = users::table
    //     .select(User::as_select())
    //     .load(connection)
    //     .unwrap();
    // dbg!(users);

    let names = users::dsl::users
        .select(users::dsl::hair_color.concat("ish"))
        .load(connection);
    let expected_names = vec![Some("Greenish".to_string()), None];
    assert_eq!(Ok(expected_names), names);
}

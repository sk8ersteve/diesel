use super::schema::*;
use diesel::*;

#[diesel_test_helper::test]
fn limit() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Sean".to_string(), None::<String>)];
    let actual_data: Vec<_> = users
        .select((name, hair_color))
        .limit(1)
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);
}

#[cfg(any(feature = "sqlite", feature = "postgres"))]
#[diesel_test_helper::test]
fn offset() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Tess".to_string(), None::<String>)];
    let q = users.select((name, hair_color)).offset(1);
    let actual_data: Vec<_> = q.load(connection).unwrap();
    assert_eq!(expected_data, actual_data);
}

#[diesel_test_helper::test]
fn limit_offset() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess'), ('Ruby')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Ruby".to_string(), None::<String>)];
    let q = users.select((name, hair_color)).limit(1).offset(2);
    let actual_data: Vec<_> = q.load(connection).unwrap();
    assert_eq!(expected_data, actual_data);
}

#[diesel_test_helper::test]
fn boxed_limit() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Sean".to_string(), None::<String>)];
    let actual_data: Vec<_> = users
        .into_boxed()
        .select((name, hair_color))
        .limit(1)
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);

    let actual_data: Vec<_> = users
        .select((name, hair_color))
        .limit(1)
        .into_boxed()
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);
}

#[diesel_test_helper::test]
fn boxed_offset() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Tess".to_string(), None::<String>)];

    let actual_data: Vec<_> = users
        .select((name, hair_color))
        .into_boxed()
        .offset(1)
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    {
        let actual_data: Vec<_> = users
            .select((name, hair_color))
            .offset(1)
            .into_boxed()
            .load(connection)
            .unwrap();
        assert_eq!(expected_data, actual_data);
    }
}

#[diesel_test_helper::test]
fn boxed_limit_offset() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess'), ('Ruby')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![("Ruby".to_string(), None::<String>)];

    let actual_data: Vec<_> = users
        .into_boxed()
        .select((name, hair_color))
        .limit(1)
        .offset(2)
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);

    let actual_data: Vec<_> = users
        .select((name, hair_color))
        .limit(1)
        .offset(2)
        .into_boxed()
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);

    let actual_data: Vec<_> = users
        .select((name, hair_color))
        .limit(1)
        .into_boxed()
        .offset(2)
        .load(connection)
        .unwrap();
    assert_eq!(expected_data, actual_data);

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    {
        let actual_data: Vec<_> = users
            .select((name, hair_color))
            .offset(2)
            .into_boxed()
            .limit(1)
            .load(connection)
            .unwrap();
        assert_eq!(expected_data, actual_data);
    }
}

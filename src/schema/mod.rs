pub mod types;
use juniper::FieldResult;
use types::*;

pub struct Query;

pub struct Context {}
impl juniper::Context for Context {}

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field userTestGroups(&executor, id: String) -> FieldResult<UserTestGroups> {
        let result = UserTestGroups {
            user: id,
            id: String::from("123"),
            groups: vec![],
        };
        Ok(result)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field addTestGroup(&executor, new_test_group: NewTestGroup) -> FieldResult<UserTestGroups> {
        let result = UserTestGroups {
        user: new_test_group.user,
        id: String::from("123"),
        groups: vec![AB { test: new_test_group.group.test, group: new_test_group.group.group }],
        };
        Ok(result)
    }
});

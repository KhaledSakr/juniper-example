
#[derive(GraphQLEnum)]
pub enum Group {
    A,
    B,
}

#[derive(GraphQLObject)]
pub struct AB {
    pub test: String,
    pub group: Group,
}

#[derive(GraphQLInputObject)]
// AB and ABInput are exactly the same, find a better way to handle this
pub struct ABInput {
    pub test: String,
    pub group: Group,
}

#[derive(GraphQLObject)]
#[graphql(description="Association of a user to test groups")]
pub struct UserTestGroups {
    pub id: String,
    pub user: String,
    pub groups: Vec<AB>,
}

#[derive(GraphQLInputObject)]
#[graphql(description="New test group for a specific user")]
pub struct NewTestGroup {
    pub user: String,
    pub group: ABInput,
}

use app_core::StringListApp;
use async_graphql::{Context, Object, Schema, Subscription};
use tokio_stream::{Stream, StreamExt};

pub type AppSchema = Schema<Query, Mutation, Updates>;

pub fn schema(app: StringListApp) -> AppSchema {
    Schema::build(Query, Mutation, Updates).data(app).finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn strings(&self, ctx: &Context<'_>) -> Vec<String> {
        ctx.data_unchecked::<StringListApp>()
            .read()
            .as_ref()
            .clone()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_random_string(&self, ctx: &Context<'_>) -> String {
        ctx.data_unchecked::<StringListApp>().add_random_string()
    }
}

pub struct Updates;

#[Subscription]
impl Updates {
    async fn strings_changed(&self, ctx: &Context<'_>) -> impl Stream<Item = Vec<String>> {
        ctx.data_unchecked::<StringListApp>()
            .subscribe()
            .map(|snapshot| snapshot.as_ref().clone())
    }
}

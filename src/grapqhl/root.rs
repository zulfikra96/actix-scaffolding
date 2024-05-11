use actix_session::Session;
use actix_web::{web, HttpResponse};
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder,
};
use async_graphql_actix_web::GraphQLRequest;

use crate::DBPool;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, async_graphql::Error>;
pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn todo<'ctx>(&self, ctx: &Context<'ctx>, a: i32, b: i32) -> i32 {
        let _ctx = ctx.data::<String>();
        println!("ctx {:?}", _ctx);
        a + b
    }
}

pub async fn schema_handler() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    let schema: SchemaBuilder<Query, EmptyMutation, EmptySubscription> =
        Schema::build(Query, EmptyMutation, EmptySubscription);
    schema
}

pub async fn handler(
    req_gql: GraphQLRequest,
    conn: web::Data<DBPool>,
    session: Session,
) -> HttpResponse {
    let _session = session;
    let res = schema_handler().await.data(conn).finish();
    let res = res.execute(req_gql.into_inner()).await;
    println!("{:?}", res.data);
    HttpResponse::Ok().json(res)
}

pub async fn graphiql() -> HttpResponse {
    let source = GraphiQLSource::build().endpoint("/").finish();
    HttpResponse::Ok().body(source)
}

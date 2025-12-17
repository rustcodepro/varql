mod combine;
mod machine;
mod pad;
mod preparevariant;
mod query;
mod refreturn;
mod samread;
mod varstruct;
use crate::machine::Query;
use async_graphql::Schema;
use async_graphql::http::GraphiQLSource;
use axum::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;

/*
Gaurav Sablok
codeprog@icloud.com
*/

/*
Gaurav Sablok
codeprog@icloud.com
*/

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphiql").finish());
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(
        Query,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish();

    let app = Router::new().route("/graphiql", get(graphiql));
    println!("Server running on http://127.0.0.1:4000/graphql");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

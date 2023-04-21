use std::convert::Infallible;

use serde::Serialize;
use warp::{path, reply, Filter};

use crate::{db::ConnectionPool, api::gql};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct ApiRootResponse {
    version: String,
    commit_sha: String,
}

pub async fn run_server(_pool: &ConnectionPool) {
    // GET /
    let root = warp::get().and(path::end()).map(|| {
        let response = ApiRootResponse {
            version: VERSION.to_string(),
            commit_sha: option_env!("COMMIT_SHA").unwrap_or("undefined").to_string(),
        };

        reply::json(&response)
    });

    let schema = gql::build_schema(); 
    let filter = async_graphql_warp::graphql(schema).and_then(|(schema, request): (gql::ApiSchema, async_graphql::Request)| async move {
        // Execute query
        let resp = schema.execute(request).await;
    
        // Return result
        Ok::<_, Infallible>(async_graphql_warp::GraphQLResponse::from(resp))
    });

    // POST /graphql
    let graphql = warp::post()
        .and(warp::path("graphql"))
        .and(warp::path::end())
        .and(filter);

    let api = root.or(graphql);

    let server = warp::serve(api).run(([0, 0, 0, 0], 5000));

    println!("API server listening on 0.0.0.0:5000");

    server.await
}

// async fn api() -> warp::filters::BoxedFilter<()> {
//     path!("api").boxed()
// }

#[macro_use]
extern crate juniper;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate juniper_hyper;
extern crate mongodb;
// use mongodb::{Client, ThreadedClient};
// use mongodb::db::ThreadedDatabase;

mod schema;

use futures::future;
use hyper::{
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};
use std::sync::Arc;

fn main() {
    // let client = Client::connect("localhost", 27017)
    //   .expect("Failed to initialize standalone client.");
    // let coll = client.db("test").collection("ab");

    let addr = ([127, 0, 0, 1], 3000).into();

    let ctx = Arc::new(schema::Context {});
    let root_node = Arc::new(juniper::RootNode::new(schema::Query, schema::Mutation));

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = ctx.clone();
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}

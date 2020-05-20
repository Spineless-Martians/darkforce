use crate::schema::Schema;
use failure::Error;
use slog::Logger;

use crate::query::Query;
use hyper::{
    service::{
        make_service_fn,
        service_fn,
    },
    Body,
    Method,
    Response,
    Server,
    StatusCode,
};
use juniper::{
    EmptyMutation,
    EmptySubscription,
};
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "app/public/"]
struct Assets;

pub async fn start_web_server(logger: &Logger) -> Result<(), Error> {
    let root_node = Arc::new(Schema::new(
        Query {},
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));

    let addr = ([127, 0, 0, 1], 3000).into();
    let new_service = make_service_fn(move |_| {
        let root_node = root_node.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let root_node = root_node.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/playground") => {
                            juniper_hyper::playground("/graphql", None).await
                        }
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, Arc::new(()), req).await
                        }
                        _ => {
                            let asset = Assets::iter().find(|i| req.uri().path() == i);

                            let body = if asset.is_some() {
                                Body::from(Assets::get(&asset.unwrap()).unwrap())
                            } else {
                                Body::from(Assets::get("index.html").unwrap())
                            };

                            let mut response = Response::new(body);
                            *response.status_mut() = StatusCode::OK;
                            Ok(response)
                        }
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(new_service)
        .with_graceful_shutdown(shutdown_signal());
    info!(logger, "Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

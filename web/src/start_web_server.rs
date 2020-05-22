use crate::schema::Schema;
use failure::Error;
use slog::Logger;

use crate::query::Query;
use darkforce_shared::Settings;
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

pub async fn start_web_server(logger: &Logger, settings: &Settings) -> Result<(), Error> {
    for asset in Assets::iter() {
        debug!(logger, "Available static asset: {}", asset);
    }

    let root_node = Arc::new(Schema::new(
        Query {},
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));

    let new_service = make_service_fn(move |_| {
        let logger = logger.clone();
        let root_node = root_node.clone();

        async move {
            let logger = logger.clone();
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let root_node = root_node.clone();
                let logger = logger.clone();
                async move {
                    let logger = logger.new(o!("path" => req.uri().path().to_string(), "method" => req.method().to_string(), "request_id" => "ID"));
                    info!(logger, "Request received");
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/playground") => {
                            juniper_hyper::playground("/graphql", None).await
                        }
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, Arc::new(()), req).await
                        }
                        _ => {
                            let asset =
                                Assets::iter().find(|i| req.uri().path() == format!("/{}", i));
                            let body = if let Some(ass) = asset {
                                Body::from(Assets::get(&ass).unwrap())
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

    let server = Server::bind(&settings.web_address())
        .serve(new_service)
        .with_graceful_shutdown(shutdown_signal());
    info!(logger, "Listening on http://{}", settings.web_address());

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

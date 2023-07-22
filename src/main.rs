use axum::Router;
mod routes;


#[tokio::main]
async fn main() {
    println!("Axum server running...");
    // build our application with a single route
    let app = Router::new();
    let routes = routes::all::define_routes(app);

    // run it with hyper on localhost:8000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
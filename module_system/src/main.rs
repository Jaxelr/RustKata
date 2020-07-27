mod config;
mod routes;
mod models;

fn main() {
    routes::health_route::print_health_route();
    routes::user_route::print_user_route();
    config::print_config();

    println!("main");
}

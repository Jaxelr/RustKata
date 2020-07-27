use crate::models::user_model::print_user_model;

pub fn print_user_route() {
    print_user_model();
    
    super::health_route::print_health_route();
    
    println!("user_route");
}
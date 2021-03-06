use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum  Route {
    #[to = "/product/{id}"]
    ProductDetails(i32),
    #[to = "/"]
    HomePage,
}
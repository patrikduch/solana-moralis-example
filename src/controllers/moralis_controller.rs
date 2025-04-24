use actix_web::{HttpResponse, Responder};
use crate::dtos::moralis::responses::TokenPriceResponse;
use crate::handlers::queries::moralis::handle_get_sol_price;

pub struct MoralisController;

impl MoralisController {
    pub async fn get_sol_price() -> impl Responder {
        match handle_get_sol_price::handle().await {
            Ok(price) => HttpResponse::Ok().json(TokenPriceResponse { usd_price: price }),
            Err(err) => {
                println!("Error fetching SOL price: {:?}", err);
                HttpResponse::InternalServerError().body("Failed to fetch SOL price")
            }
        }
    }
}

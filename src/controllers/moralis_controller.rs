use actix_web::{get, web, HttpResponse, Responder};
use crate::dtos::moralis::responses::TokenPriceResponse;
use crate::handlers::queries::moralis::handle_get_sol_price;
use crate::handlers::queries::moralis::handle_get_token_price;

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

    pub async fn get_token_price(path: web::Path<String>) -> impl Responder {
        let token_address = path.into_inner();
        match handle_get_token_price::handle(&token_address).await {
            Ok(price) => HttpResponse::Ok().json(TokenPriceResponse { usd_price: price }),
            Err(err) => {
                println!("Error fetching token price for {}: {:?}", token_address, err);
                HttpResponse::InternalServerError().body("Failed to fetch token price")
            }
        }
    }
}

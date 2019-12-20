// use actix_web::{web, HttpResponse};

// pub fn append_resources(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/patents/{id}")
//             .route(
//                 "/generate_address",
//                 web::get().to_async(bitcoin::generate_address_handler),
//             )
//             .route(
//                 "/get_balance",
//                 web::get().to_async(bitcoin::get_balance_handler),
//             )
//             .route(
//                 "/get_utxo",
//                 web::get().to_async(bitcoin::get_utxo_handler))
//             .route(
//                 "/send_raw_transaction",
//                 web::post().to_async(bitcoin::send_raw_transaction_handler),
//             )
//             .route(
//                 "/sign",
//                 web::post().to(bitcoin::sign)
//             )
//     );

// }


// pub mod patents {
//     async fn get_patent (req: HttpRequest,

//     )
// }
use actix_web::web;

pub fn append_resources(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::resource("/patents").route(web::post().to(patents::add_patent)))
    .service(web::resource("/patents/{serialNumber}").route(web::get().to(patents::get_patent)));
}

pub mod patents {
    use crate::models::patent::Patent;
    use actix_web::{web, HttpResponse};

    pub async fn get_patent(
        path: web::Path<(String,)>,
        state: web::Data<crate::models::State>,
    ) -> Result<HttpResponse, failure::Error> {
        let serial_number = path.0.clone();
        let session = state.session.clone();
        let patent: Patent =
            web::block(move || Patent::get_by_serial_number(serial_number, session))
                .await
                .map_err(|err| format_err!("{:?}", err))?;
        Ok(HttpResponse::Ok().body(serde_json::to_string(&patent)?))
    }

    pub async fn add_patent(
        body: web::Json<Patent>,
        state: web::Data<crate::models::State>,
    ) -> Result<HttpResponse, failure::Error> {
        let patent: Patent = body.0;
        let session = state.session.clone();
        web::block(move || patent.insert(session))
            .await
            .map_err(|err| format_err!("{:?}", err))?;
        Ok(HttpResponse::Ok().into())
    }
}

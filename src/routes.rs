use actix_web::web::{ServiceConfig, post, scope, get};

use crate::{
    stream::index,
    handlers::{
    auth::{register::reg, log_in::log_in, log_out::log_out},
    chat::{show_channels::show_channels, get_messages::get_messages, create_channel::create_channel, create_message::create_message}, ws::start_connection}
};
    

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
scope("/api/v1")
            .service(
        scope("/auth")
                    .route("/reg", post().to(reg))
                    .route("/log_in", post().to(log_in))
                    .route("/log_out", get().to(log_out))
            )
            .service(
        scope("/chat")
                    .service(
                scope("/messages")
                            .route("/create", post().to(create_message))
                            .route("/get", post().to(get_messages))
                    )
                    .service(
                scope("/channels")
                            .route("/get", get().to(show_channels))
                            .route("/create", post().to(create_channel))
                    )
            )
            .service(
        scope("/stream")
                    .route("/start", get().to(index))
                    .service(start_connection::start_connection)
            )
    );
}
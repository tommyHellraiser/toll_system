mod life_services;

use actix_web::{web, App, HttpServer};
use actix_web::dev::ServerHandle;
use error_mapper::{create_new_error, TheResult};
use the_logger::{log_info, TheLogger};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::config::environment::Environment;

struct ApiData {
    api_stopper: Sender<bool>
}

pub(super) async fn start_api() -> TheResult<()> {

    let api_config = Environment::get_api_config().await;
    let api_ip = api_config.get_ip_addr();
    let api_port = api_config.get_port();

    let (stop_sender, stop_receiver) = tokio::sync::mpsc::channel::<bool>(5);

    log_info!(TheLogger::instance(), "Starting Api at: {}", chrono::Local::now());
    
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ApiData { api_stopper: stop_sender.clone() })).service(
            web::scope("/api")
                .service(
                    web::scope("/manage").configure(life_services::life_services)
                )
        )
    }).bind((api_ip, api_port)).map_err(|e| create_new_error!(e.to_string().as_str()))?.run();

    tokio::spawn(api_killer(api_server.handle(), stop_receiver));

    api_server.await.map_err(|e| create_new_error!(e.to_string()))
}

async fn api_killer(server_handler: ServerHandle, mut stop_receiver: Receiver<bool>) {
    
    'stop_loop: loop {
        if let Some(stop_method) = stop_receiver.recv().await {
            server_handler.stop(stop_method).await;
            break 'stop_loop;
        }    
    }
}

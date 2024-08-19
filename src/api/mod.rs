mod life_services;

use actix_web::{web, App, HttpServer};
use actix_web::dev::ServerHandle;
use error_mapper::{create_new_error, TheResult};
use the_logger::{log_info, TheLogger};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::config::environment::Environment;
use crate::{modules, DATETIME_FORMAT};

struct ApiData {
    api_stopper: Sender<bool>
}

pub(super) async fn start_api() -> TheResult<()> {

    let api_config = Environment::get_api_config().await;
    let api_ip = api_config.get_ip_addr();
    let api_port = api_config.get_port();
    let shutdown_timeout_seconds = api_config.get_shutdown_timeout_seconds();

    let (stop_sender, stop_receiver) = tokio::sync::mpsc::channel::<bool>(5);

    log_info!(
        TheLogger::instance(),
        "Starting Api at: {}",
        chrono::Local::now().format(DATETIME_FORMAT)
    );
    
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ApiData { api_stopper: stop_sender.clone() })).service(
            web::scope("/api")
                .service(
                    web::scope("/manage").configure(life_services::life_services)
                )
                .service(
                    web::scope("/blacklist").configure(modules::blacklist::services::blacklist_services)
                )
                .service(
                    web::scope("/booths").configure(modules::booths::services::booths_services)
                )
                .service(
                    web::scope("/clients").configure(modules::clients::services::clients_services)
                )
                .service(
                    web::scope("/clients_balances").configure(modules::clients_balances::services::client_balances_services)
                )
                .service(
                    web::scope("/discounts").configure(modules::discounts::services::discount_services)
                )
                .service(
                    web::scope("/penalties").configure(modules::penalties::services::penalties_services)
                )
                .service(
                    web::scope("/registered_vehicles").configure(modules::registered_vehicles::services::registered_vehicles_services)
                )
                .service(
                    web::scope("/transactions").configure(modules::transactions::services::transactions_services)
                )
                .service(
                    web::scope("/transit").configure(modules::transit::services::transit_services)
                )
                .service(
                    web::scope("/transit_logs").configure(modules::transit_logs::services::transit_logs_services)
                )
                .service(
                    web::scope("/transit_rates").configure(modules::transit_rates::services::transit_rates_services)
                )
                .service(
                    web::scope("/vehicle_types").configure(modules::vehicle_types::services::vehicle_types_services)
                )
                .service(
                    web::scope("/violation_logs").configure(modules::violation_logs::services::violation_logs_services)
                )
        ).service(
                web::scope("/internal")
                    .service(
                    web::scope("/booths").configure(modules::booths::services::booths_internal_services)
                )
                    .service(
                        web::scope("/discounts").configure(modules::discounts::services::discounts_internal_services)
                    )
                    .service(
                        web::scope("/penalties").configure(modules::penalties::services::penalties_internal_services)
                    )
                    .service(
                        web::scope("/transit_rates").configure(modules::transit_rates::services::transit_rates_internal_services)
                    )
            )
    }).bind((api_ip, api_port))
        .map_err(|e| create_new_error!(e.to_string().as_str()))?
        .shutdown_timeout(shutdown_timeout_seconds as u64)
        .run();

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

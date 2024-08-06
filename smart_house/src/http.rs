use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::devices::{SmartHouse, SmartSocket, SmartThermometer};

#[derive(Deserialize)]
struct AddDevice {
    room_name: String,
    device_type: DeviceType,
    device_name: String,
}

#[derive(Deserialize)]
struct AddRoom {
    room_name: String,
}

#[derive(Deserialize)]
enum DeviceType {
    Socket,
    Thermo,
}

pub async fn run_http_server(house: Arc<Mutex<SmartHouse>>) -> std::io::Result<()> {
    println!("Starting http server...");
    HttpServer::new(move || {
        let data = Data::new(house.clone());
        App::new()
            .app_data(data)
            .service(hello)
            .service(add_room)
            .service(add_device)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello(data: web::Data<Arc<Mutex<SmartHouse>>>) -> impl Responder {
    let house = data.lock().unwrap();
    HttpResponse::Ok().body({
        let mut report = "".to_string();
        for room in house.get_rooms() {
            report.push_str(room);
            report.push_str(": ");
            for device in house.devices(room.to_string()).unwrap() {
                report.push_str(device.0);
                report.push(' ');
            }
            report.push('\n')
        }
        report
    })
}

#[put("room/{room_name}")]
async fn add_room(
    data: web::Data<Arc<Mutex<SmartHouse>>>,
    info: web::Path<AddRoom>,
) -> impl Responder {
    println!("adding new room {}", info.room_name);
    match data
        .clone()
        .lock()
        .unwrap()
        .add_room(info.room_name.clone())
    {
        Ok(_) => HttpResponse::Created().body("room created"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[put("device/{room_name}/{device_type}/{device_name}")]
async fn add_device(
    data: web::Data<Arc<Mutex<SmartHouse>>>,
    info: web::Path<AddDevice>,
) -> impl Responder {
    println!(
        "adding new device {} into room {}",
        info.device_name, info.room_name
    );
    let device = match info.device_type {
        DeviceType::Socket => Box::new(SmartSocket::new(
            info.room_name.clone(),
            info.device_name.clone(),
        )) as _,
        DeviceType::Thermo => Box::new(SmartThermometer::new(
            info.room_name.clone(),
            info.device_name.clone(),
        )) as _,
    };
    match data
        .clone()
        .lock()
        .unwrap()
        .add_device(info.room_name.clone(), device)
    {
        Ok(_) => HttpResponse::Created().body("device created"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

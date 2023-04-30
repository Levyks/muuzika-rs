use test::TestRequest;
use actix_web::{http::header::ContentType, test, web, App};
use actix_web::http::header::Accept;
use actix_web::http::StatusCode;

use muuzika::state::AppState;
use muuzika::app::config;
use muuzika::dtos::responses::CreateOrJoinRoomResponse;

macro_rules! print_test_start {
    ($test_name:expr) => {
        println!("\n--------\n{}:\n", $test_name);
    };
}

#[actix_web::test]
async fn test_create_room_endpoint() {
    print_test_start!("test_create_room_endpoint");

    const LEADER_USERNAME: &str = "Leader";

    let app = test::init_service(App::new()
        .app_data(web::Data::new(AppState::new()))
        .configure(config)
    ).await;

    let payload = format!(r#"{{"username": "{}"}}"#, LEADER_USERNAME);

    let req = TestRequest::post().uri("/rooms")
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("Response: {:?}", resp);

    assert_eq!(resp.status(), StatusCode::OK);

    let body: CreateOrJoinRoomResponse = test::read_body_json(resp).await;
    println!("Body: {:?}", body);

    assert_eq!(body.room.code.len(), 4);
    assert_eq!(body.room.leader_username, LEADER_USERNAME);
    assert_eq!(body.room.players.len(), 1);
    assert_eq!(body.room.players[0].username, LEADER_USERNAME);
    assert_eq!(body.room.players[0].score, 0);
    assert_eq!(body.room.players[0].is_connected, false);
}

#[actix_web::test]
async fn test_join_room_endpoint() {
    print_test_start!("test_join_room_endpoint");

    const LEADER_USERNAME: &str = "Leader";
    const PLAYER_USERNAME: &str = "Player";

    let app = test::init_service(App::new()
        .app_data(web::Data::new(AppState::new()))
        .configure(config)
    ).await;

    let create_payload = format!(r#"{{"username": "{}"}}"#, LEADER_USERNAME);

    let create_req = TestRequest::post().uri("/rooms")
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_payload(create_payload)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), StatusCode::OK);

    let create_body: CreateOrJoinRoomResponse = test::read_body_json(create_resp).await;

    let join_payload = format!(r#"{{"username": "{}"}}"#, PLAYER_USERNAME);
    let join_uri = format!("/rooms/{}", create_body.room.code);

    let join_req = TestRequest::post().uri(&join_uri)
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_payload(join_payload)
        .to_request();

    let join_resp = test::call_service(&app, join_req).await;
    println!("Response: {:?}", join_resp);
    assert_eq!(join_resp.status(), StatusCode::OK);

    let join_body: CreateOrJoinRoomResponse = test::read_body_json(join_resp).await;
    println!("Body: {:?}", join_body);

    assert_eq!(join_body.room.code, create_body.room.code);
    assert_eq!(join_body.room.leader_username, LEADER_USERNAME);
    assert_eq!(join_body.room.players.len(), 2);

    let player = join_body.room.players.iter().find(|p| p.username == PLAYER_USERNAME);

    assert!(player.is_some());

    let player = player.unwrap();

    assert_eq!(player.score, 0);
    assert_eq!(player.is_connected, false);
}
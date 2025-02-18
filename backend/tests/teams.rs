use actix_web::http::header;
use actix_web::middleware::from_fn;
use actix_web::web::Data;
use actix_web::{test, App};
use chrono::Duration;
use hack4krak_backend::models::entities::{teams, users};
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::env::Config;
use hack4krak_backend::utils::jwt::encode_jwt;
use hack4krak_backend::{middlewares, routes};
use sea_orm::{DatabaseBackend, MockDatabase};
use utoipa::gen::serde_json::json;
use utoipa_actix_web::scope;

#[actix_web::test]
async fn create_team_user_already_belongs_to_team() {
    Config::load_test_config();

    let uuid = uuid::Uuid::new_v4();
    let team_id = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: Some(team_id),
            permissions: None,
            is_leader: false,
            password: None,
        }]])
        .append_query_results([vec![teams::Model {
            id: team_id,
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
        }]])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(
                scope("/teams/manage")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::teams::management::config),
            ),
    )
    .await;

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let create_team_payload = json!({
        "team_name": "team1".to_string(),
    });

    let request = test::TestRequest::post()
        .uri("/teams/manage/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn create_duplicate_team() {
    Config::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: None,
            permissions: None,
            is_leader: false,
            password: None,
        }]])
        .append_query_results([vec![teams::Model {
            id: Default::default(),
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
        }]])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(
                scope("/teams/manage")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::teams::management::config),
            ),
    )
    .await;

    let create_team_payload = json!({
        "team_name": "Dziengiel".to_string(),
    });

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/manage/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn create_team_success() {
    Config::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let example_user = users::Model {
        id: uuid,
        username: "Salieri".to_string(),
        email: "example@gmail.com".to_string(),
        created_at: Default::default(),
        team: None,
        permissions: None,
        is_leader: false,
        password: None,
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![Vec::<teams::Model>::new()])
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![vec![example_user]])
        .append_query_results(vec![Vec::<teams::Model>::new()])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(
                scope("/teams/manage")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::teams::management::config),
            ),
    )
    .await;

    let create_team_payload = json!({
        "team_name": "Dziengiel".to_string(),
    });

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/manage/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}

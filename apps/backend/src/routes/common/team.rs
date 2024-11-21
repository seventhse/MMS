use std::sync::Arc;

use crate::common::{handle_response_by_service, ApiResponse, Empty};
use actix_web::{delete, get, post, put, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use service::{
    common::{
        team_service::{CreateTeamDto, UpdateTeamDto},
        team_user_service::{JoinTeamDto, LeftTeamDto},
        user_service::UpdateUserDto,
    },
    sea_orm::{sqlx::types::Uuid, TryIntoModel},
    Service,
    _entities::sea_orm_active_enums::TeamUserRoles,
};

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/team")
            .service(get_teams)
            .service(get_team)
            .service(get_users)
            .service(join_team)
            .service(left_team)
            .service(create_team)
            .service(update_team)
            .service(delete_team),
    );
}

#[get("/list")]
async fn get_teams(service: web::Data<Arc<Service>>) -> impl Responder {
    let res = service.team_service.find_project_all().await;
    handle_response_by_service(res)
}

#[get("/detail/{id}")]
async fn get_team(service: web::Data<Arc<Service>>, id: web::Path<Uuid>) -> impl Responder {
    let res = service.team_service.find_by_id(id.into_inner()).await;
    handle_response_by_service(res)
}

#[get("/users/{id}")]
async fn get_users(service: web::Data<Arc<Service>>, id: web::Path<Uuid>) -> impl Responder {
    let res = service
        .team_user_service
        .find_users_by_team(id.into_inner())
        .await;
    handle_response_by_service(res)
}

#[post("/create")]
async fn create_team(
    service: web::Data<Arc<Service>>,
    form: web::Json<CreateTeamDto>,
    token: BearerAuth,
) -> impl Responder {
    let form = form.into_inner();
    let token_str = token.token();

    let user_info = service
        .auth_service
        .get_user_info_by_token(token_str)
        .await
        .unwrap();

    log::info!("Create team form data: {:#?}", form);
    let res = service.team_service.create_team(form).await;

    match res {
        Ok(team) => {
            let team = team.try_into_model().unwrap();

            let payload = JoinTeamDto {
                team_id: team.team_id,
                user_id: user_info.user_id,
                role: TeamUserRoles::Owner,
            };

            if user_info.default_team_id.is_none() {
                service
                    .user_service
                    .update_user_by_id(
                        user_info.user_id,
                        UpdateUserDto {
                            username: None,
                            display_name: None,
                            email: None,
                            avatar: None,
                            default_team_id: Some(team.team_id),
                        },
                    )
                    .await
                    .unwrap();
            }

            if let Err(e) = service.team_user_service.join_team(payload).await {
                return ApiResponse::<Empty>::bad_request(Some(&e.to_string()));
            }

            ApiResponse::<Empty>::ok(Some("Create team successful!"))
        }
        Err(e) => ApiResponse::<Empty>::bad_request(Some(&e.to_string())),
    }
}

#[post("/left-team")]
async fn left_team(
    service: web::Data<Arc<Service>>,
    payload: web::Json<LeftTeamDto>,
    token: BearerAuth,
) -> impl Responder {
    let token_str = token.token();
    let user_id = service
        .auth_service
        .get_user_id_by_token(token_str)
        .await
        .unwrap();

    let payload = payload.into_inner();

    if payload.user_id == user_id {
        return ApiResponse::<()>::bad_request(Some("Cannot remove yourself from team"));
    }

    let role = service
        .team_user_service
        .get_user_role_by_team(payload.team_id, user_id)
        .await;

    match role {
        Ok(role) => {
            if !role.can_remove_user_by_team() {
                return ApiResponse::<()>::forbidden(Some(
                    "Only Owner or Admin can remove user in team.",
                ));
            }
        }
        Err(e) => return ApiResponse::<()>::bad_request(Some(&e.to_string())),
    }

    let res = service.team_user_service.left_team(payload).await;

    handle_response_by_service(res)
}

#[post("/join-team")]
async fn join_team(
    service: web::Data<Arc<Service>>,
    payload: web::Json<JoinTeamDto>,
) -> impl Responder {
    let res = service
        .team_user_service
        .join_team(payload.into_inner())
        .await;

    handle_response_by_service(res)
}

#[put("/update/{id}")]
async fn update_team(
    service: web::Data<Arc<Service>>,
    id: web::Path<Uuid>,
    form: web::Json<UpdateTeamDto>,
    token: BearerAuth,
) -> impl Responder {
    let token_str = token.token();
    let user_id = service
        .auth_service
        .get_user_id_by_token(token_str)
        .await
        .unwrap();

    let team_id = id.into_inner();
    let role = service
        .team_user_service
        .get_user_role_by_team(team_id, user_id)
        .await;

    let res = service
        .team_service
        .update_team(team_id, form.into_inner())
        .await;

    match role {
        Ok(role) => {
            if !role.can_update_team() {
                return ApiResponse::<()>::forbidden(Some("Only Owner or admin can delete team"));
            }
        }
        Err(e) => return ApiResponse::<()>::bad_request(Some(&e.to_string())),
    }

    handle_response_by_service(res)
}

#[delete("/delete/{id}")]
async fn delete_team(
    service: web::Data<Arc<Service>>,
    id: web::Path<Uuid>,
    token: BearerAuth,
) -> impl Responder {
    let token_str = token.token();
    let user_id = service
        .auth_service
        .get_user_id_by_token(token_str)
        .await
        .unwrap();

    let team_id = id.into_inner();
    let role = service
        .team_user_service
        .get_user_role_by_team(team_id, user_id)
        .await;

    match role {
        Ok(role) => {
            if !role.can_remove_team() {
                return ApiResponse::<()>::forbidden(Some("Only Owner can delete team"));
            }
        }
        Err(e) => return ApiResponse::<()>::bad_request(Some(&e.to_string())),
    }

    if let Err(e) = service
        .team_user_service
        .clean_relation_by_team(team_id.clone())
        .await
    {
        return ApiResponse::<()>::bad_request(Some(&e.to_string()));
    }

    let res = service.team_service.delete_team(team_id).await;

    handle_response_by_service(res)
}

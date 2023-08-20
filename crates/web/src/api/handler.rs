// // login handler
// pub async fn login_handler(
//     Extension(db): Extension<Database>,
//     Json(payload): Json<LoginRequest>,
// ) -> Result<Json<LoginResponse>, StatusCode> {
//     let user = db
//         .get_user_by_username(&payload.username)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .ok_or(StatusCode::NOT_FOUND)?;

//     if user.password != payload.password {
//         return Err(StatusCode::UNAUTHORIZED);
//     }

//     let token = db
//         .create_token(&user.username)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(LoginResponse { token }))
// }

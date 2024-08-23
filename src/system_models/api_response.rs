use crate::repository::models;
use crate::system_models::errors::AppError;
use crate::system_models::scenario_status::EScenarioStatus;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
	pub status: EScenarioStatus,
	pub result: String,
	pub payload: Option<serde_json::Value>,
}

impl ApiResponse {
	fn new(status: EScenarioStatus, result: String, payload: Option<serde_json::Value>) -> Self {
		return Self {
			status,
			result,
			payload,
		};
	}

	fn scenario_success(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SCENARIO_SUCCESS, result, payload);
	}

	pub fn unauthorized(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::UNAUTHORIZED, result, payload);
	}

	fn scenario_fail(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SCENARIO_FAIL, result, payload);
	}

	pub fn system_error(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SYSTEM_ERROR, result, payload);
	}

	//  *********************************
	//  *                               *
	//  *       Scenario Success        *
	//  *                               *
	//  *********************************

	pub fn user_registered(promocode: String) -> Result<Self, AppError> {
		let upper = promocode.to_uppercase();
		return Ok(Self::scenario_success(
			format!("Новый пользователь успешно зарегистрирован. Промокод: {upper}"),
			None,
		));
	}

	pub fn promo_valid() -> Result<Self, AppError> {
		return Ok(Self::scenario_success(
			String::from("Промокод корректен"),
			None,
		));
	}

	pub fn promo_activated() -> Result<Self, AppError> {
		return Ok(Self::scenario_success(
			String::from("Промокод успешно активирован"),
			None,
		));
	}

	pub fn user_list(users: Vec<models::RegisteredUser>) -> Result<Self, AppError> {
		let payload = serde_json::json!(users);
		return Ok(Self::scenario_success(
			String::from("Список пользователей"),
			Some(payload),
		));
	}
}

impl IntoResponse for ApiResponse {
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}

impl From<AppError> for ApiResponse {
	fn from(err: AppError) -> Self {
		match err {
			AppError::ScenarioError(result, payload) => {
				ApiResponse::scenario_fail(result, payload.map(|p| serde_json::json!(p)))
			}
			AppError::SystemError(result) => ApiResponse::system_error(result, None),
		}
	}
}

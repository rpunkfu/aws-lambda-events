use crate::custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// `AppSyncResolverTemplate` represents the requests from AppSync to Lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncResolverTemplate<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    pub operation: AppSyncOperation,
    #[serde(bound = "")]
    pub payload: Option<T1>,
}

/// `AppSyncIamIdentity` contains information about the caller authed via IAM.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncIamIdentity {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: Option<String>,
    #[serde(rename = "sourceIp")]
    pub source_ip: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub username: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userArn")]
    pub user_arn: Option<String>,
}

/// `AppSyncCognitoIdentity` contains information about the caller authed via Cognito.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncCognitoIdentity<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sub: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub username: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub claims: HashMap<String, T1>,
    #[serde(rename = "sourceIp")]
    pub source_ip: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "defaultAuthStrategy")]
    pub default_auth_strategy: Option<String>,
}

pub type AppSyncOperation = String;

/// `AppSyncLambdaAuthorizerRequest` contains an authorization request from AppSync.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncLambdaAuthorizerRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "authorizationToken")]
    pub authorization_token: Option<String>,
    #[serde(rename = "requestContext")]
    pub request_context: AppSyncLambdaAuthorizerRequestContext,
}

/// `AppSyncLambdaAuthorizerRequestContext` contains the parameters of the AppSync invocation which triggered
/// this authorization request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncLambdaAuthorizerRequestContext<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "queryString")]
    pub query_string: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub variables: HashMap<String, T1>,
}

/// `AppSyncLambdaAuthorizerResponse` represents the expected format of an authorization response to AppSync.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncLambdaAuthorizerResponse<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    #[serde(rename = "resolverContext")]
    pub resolver_context: HashMap<String, T1>,
    #[serde(rename = "deniedFields")]
    pub denied_fields: Option<Vec<String>>,
    #[serde(rename = "ttlOverride")]
    pub ttl_override: Option<i64>,
}

use crate::custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EcrScanEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub time: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub resources: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub account: Option<String>,
    pub detail: EcrScanEventDetailType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EcrScanEventDetailType {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "scan-status")]
    pub scan_status: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "repository-name")]
    pub repository_name: Option<String>,
    #[serde(rename = "finding-severity-counts")]
    pub finding_severity_counts: EcrScanEventFindingSeverityCounts,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "image-digest")]
    pub image_digest: Option<String>,
    #[serde(rename = "image-tags")]
    pub image_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EcrScanEventFindingSeverityCounts {
    #[serde(rename = "CRITICAL")]
    pub critical: i64,
    #[serde(rename = "HIGH")]
    pub high: i64,
    #[serde(rename = "MEDIUM")]
    pub medium: i64,
    #[serde(rename = "LOW")]
    pub low: i64,
    #[serde(rename = "INFORMATIONAL")]
    pub informational: i64,
    #[serde(rename = "UNDEFINED")]
    pub undefined: i64,
}

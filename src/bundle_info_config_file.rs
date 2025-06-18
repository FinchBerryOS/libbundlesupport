use std::collections::HashMap;

use serde::Deserialize;

use crate::entitlements::EntitlementType;

#[derive(Debug, Clone, Deserialize)]
pub struct BundleInfoConfigFile {
    pub name: String,
    pub identifier: String,
    pub entry_point: String,
    pub metadata: HashMap<String, String>,

    pub icons: Icons,
    pub platforms: Vec<String>,
    pub minimum_system_version: String,
    pub device_family: Vec<String>,

    pub entitlements: Vec<EntitlementType>,

    pub url_schemes: Vec<UrlScheme>,

    pub app_services: AppServices,

    pub security: Security,

    pub fibyos: Fibyos,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Icons {
    #[serde(rename = "icon_16")]
    pub icon_16: String,
    #[serde(rename = "icon_32")]
    pub icon_32: String,
    #[serde(rename = "icon_128")]
    pub icon_128: String,
    pub launch_screen: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UrlScheme {
    pub scheme: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppServices {
    pub background_modes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Security {
    pub app_sandbox: bool,
    pub app_transport_security: AppTransportSecurity,
    pub code_signature: CodeSignature,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppTransportSecurity {
    pub allows_insecure_http: bool,
    pub exception_domains: HashMap<String, ExceptionDomain>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExceptionDomain {
    pub includes_subdomains: bool,
    pub allows_insecure_http: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CodeSignature {
    pub team_id: String,
    pub entitlements_file: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fibyos {
    #[serde(rename = "document_types")]
    pub document_types: Vec<DocumentType>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DocumentType {
    pub name: String,
    pub extensions: Vec<String>,
    pub icon_file: String,
}
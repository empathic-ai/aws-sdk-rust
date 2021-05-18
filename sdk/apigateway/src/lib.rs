#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
/// <fullname>Amazon API Gateway</fullname>
/// <p>Amazon API Gateway helps developers deliver robust, secure, and scalable mobile and web application back ends. API Gateway allows developers to securely connect mobile and web applications to APIs that run on AWS Lambda, Amazon EC2, or other publicly addressable web services that are hosted outside of AWS.</p>
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_json_errors;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
pub mod input;
mod instant_epoch;
mod json_deser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("apigateway", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;

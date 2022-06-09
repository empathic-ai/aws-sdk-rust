#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>IoT Analytics allows you to collect large amounts of device data, process messages, and store them.
//! You can then query the data and run sophisticated analytics on it.  IoT Analytics enables advanced
//! data exploration through integration with Jupyter Notebooks and data visualization through integration
//! with Amazon QuickSight.</p>
//!
//! <p>Traditional analytics and business intelligence tools are designed to process structured data. IoT data
//! often comes from devices that record noisy processes (such as temperature, motion, or sound). As a result
//! the data from these devices can have significant gaps, corrupted messages, and false readings that must be
//! cleaned up before analysis can occur. Also, IoT data is often only meaningful in the context of other data
//! from external sources. </p>
//!
//! <p>IoT Analytics automates the steps required to analyze data from IoT devices. IoT Analytics
//! filters, transforms, and enriches IoT data before storing it in a time-series data store for analysis. You
//! can set up the service to collect only the data you need from your devices, apply mathematical transforms
//! to process the data, and enrich the data with device-specific metadata such as device type and location
//! before storing it. Then, you can analyze your data by running queries using the built-in SQL query engine,
//! or perform more complex analytics and machine learning inference. IoT Analytics includes pre-built models
//! for common IoT use cases so you can answer questions like which devices are about to fail or which customers
//! are at risk of abandoning their wearable devices.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("iotanalytics", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;

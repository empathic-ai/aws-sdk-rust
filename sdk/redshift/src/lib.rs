#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Redshift</fullname>
//! <p>
//! <b>Overview</b>
//! </p>
//! <p>This is an interface reference for Amazon Redshift. It contains documentation for one of
//! the programming or command line interfaces you can use to manage Amazon Redshift clusters.
//! Note that Amazon Redshift is asynchronous, which means that some interfaces may require
//! techniques, such as polling or asynchronous callback handlers, to determine when a
//! command has been applied. In this reference, the parameter descriptions indicate whether
//! a change is applied immediately, on the next instance reboot, or during the next
//! maintenance window. For a summary of the Amazon Redshift cluster management interfaces, go to
//! <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/using-aws-sdk.html">Using the
//! Amazon Redshift Management Interfaces</a>.</p>
//! <p>Amazon Redshift manages all the work of setting up, operating, and scaling a data
//! warehouse: provisioning capacity, monitoring and backing up the cluster, and applying
//! patches and upgrades to the Amazon Redshift engine. You can focus on using your data to
//! acquire new insights for your business and customers.</p>
//! <p>If you are a first-time user of Amazon Redshift, we recommend that you begin by reading
//! the <a href="https://docs.aws.amazon.com/redshift/latest/gsg/getting-started.html">Amazon Redshift Getting Started Guide</a>.</p>
//!
//! <p>If you are a database developer, the <a href="https://docs.aws.amazon.com/redshift/latest/dg/welcome.html">Amazon Redshift Database Developer Guide</a> explains how to design,
//! build, query, and maintain the databases that make up your data warehouse. </p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`].
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

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
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
pub use aws_smithy_types::DateTime;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("redshift", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
pub use client::Client;

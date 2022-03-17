#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the Routing Control (Recovery Cluster) API Reference Guide for Amazon Route 53 Application Recovery Controller.</p>
//! <p>With Amazon Route 53 Application Recovery Controller, you can use routing control with extreme reliability to
//! recover applications by rerouting traffic across
//! Availability Zones or AWS Regions. Routing controls are simple on/off switches hosted
//! on a highly available cluster in Application Recovery Controller. A cluster provides a set of five redundant Regional endpoints against which you
//! can run API calls to get or update the state of routing controls. To implement failover, you set
//! one routing control on and another one off, to reroute traffic from one Availability Zone or Amazon Web Services Region
//! to another. </p>
//! <p>
//! <i>Be aware that you must specify the Regional endpoints for a cluster when you work with API cluster operations
//! to get or update routing control states in Application Recovery Controller.</i> In addition, you must specify the US West (Oregon) Region
//! for Application Recovery Controller API calls. For example, use the parameter <code>region us-west-2</code> with AWS CLI commands.
//! For more information, see
//! <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.update.api.html">
//! Get and update routing control states using the API</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p>
//! <p>This API guide includes information about the API operations for how to get and update routing control states
//! in Application Recovery Controller. You also must set up the structures to support routing controls: clusters and control panels.</p>
//! <p>For more information about working with routing control in Application Recovery Controller, see the following:</p>
//! <ul>
//! <li>
//! <p>To create clusters, routing controls, and control panels by using the control plane API
//! for routing control, see the <a href="https://docs.aws.amazon.com/recovery-cluster/latest/api/">Recovery Control Configuration API Reference Guide for Amazon Route 53 Application Recovery Controller</a>.</p>
//! </li>
//! <li>
//! <p>Learn about the components in recovery control configuration, including clusters,
//! routing controls, and control panels. For more information, see
//! <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/introduction-components.html#introduction-components-routing">
//! Recovery control components</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p>
//! </li>
//! <li>
//! <p>Application Recovery Controller also provides readiness checks that run continually to help make sure that your
//! applications are scaled and ready to handle failover traffic. For more information about
//! the related API actions, see the <a href="https://docs.aws.amazon.com/recovery-readiness/latest/api/">Recovery Readiness API Reference Guide for Amazon Route 53 Application Recovery Controller</a>.</p>
//! </li>
//! <li>
//! <p>For more information about creating resilient applications and preparing for
//! recovery readiness with Application Recovery Controller, see the <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/">Amazon Route 53 Application Recovery Controller Developer Guide</a>.</p>
//! </li>
//! </ul>
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
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("route53recoverycluster", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;

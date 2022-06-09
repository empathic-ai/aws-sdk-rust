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
//! <fullname>AWS OpsWorks CM</fullname>
//! <p>AWS OpsWorks for configuration management (CM) is a service that runs and manages
//! configuration management servers. You can use AWS OpsWorks CM to create and manage AWS
//! OpsWorks for Chef Automate and AWS OpsWorks for Puppet Enterprise servers, and add or remove
//! nodes for the servers to manage.</p>
//! <p>
//! <b>Glossary of terms</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <b>Server</b>: A configuration management server that can be
//! highly-available. The configuration management server runs on
//! an Amazon Elastic Compute Cloud (EC2) instance, and may use various other AWS services, such as Amazon Relational
//! Database Service (RDS) and Elastic Load Balancing. A server is a generic abstraction over the configuration
//! manager that you want to use, much like Amazon RDS. In AWS OpsWorks CM, you do not start
//! or stop servers. After you create servers, they continue to run until they are deleted.</p>
//! </li>
//! <li>
//! <p>
//! <b>Engine</b>: The engine is the specific configuration manager
//! that you want to use. Valid values in this release include <code>ChefAutomate</code> and <code>Puppet</code>.</p>
//! </li>
//! <li>
//! <p>
//! <b>Backup</b>: This
//! is an application-level backup of the data that the configuration manager
//! stores. AWS OpsWorks CM
//! creates an S3 bucket for backups when you launch the first
//! server. A backup maintains a snapshot of a server's configuration-related
//! attributes at the time the backup starts.</p>
//! </li>
//! <li>
//! <p>
//! <b>Events</b>:
//! Events are always related to a server. Events are written
//! during server creation, when health checks run, when backups
//! are created, when system maintenance is performed, etc. When you delete a server, the server's events are
//! also deleted.</p>
//! </li>
//! <li>
//! <p>
//! <b>Account attributes</b>:
//! Every account has attributes that are assigned in the AWS OpsWorks CM
//! database. These attributes store information about configuration limits (servers,
//! backups, etc.) and your customer account.
//! </p>
//! </li>
//! </ul>
//! <p>
//! <b>Endpoints</b>
//! </p>
//! <p>AWS OpsWorks CM supports the following endpoints, all HTTPS. You must connect to one of the following endpoints. Your servers
//! can only be accessed or managed within the endpoint in which they are created.</p>
//! <ul>
//! <li>
//! <p>opsworks-cm.us-east-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.us-east-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.us-west-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.us-west-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.ap-northeast-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.ap-southeast-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.ap-southeast-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.eu-central-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks-cm.eu-west-1.amazonaws.com</p>
//! </li>
//! </ul>
//! <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/opsworks-service.html">AWS OpsWorks endpoints and quotas</a> in the AWS General Reference.</p>
//! <p>
//! <b>Throttling limits</b>
//! </p>
//! <p>All API operations allow for five requests per second with a burst of 10 requests per second.</p>
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
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("opsworkscm", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;

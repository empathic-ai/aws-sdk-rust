// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::types::_virtual_machine::VirtualMachine;

pub use crate::types::_virtual_machine_details::VirtualMachineDetails;

pub use crate::types::_vmware_tag::VmwareTag;

pub use crate::types::_hypervisor::Hypervisor;

pub use crate::types::_hypervisor_state::HypervisorState;

pub use crate::types::_tag::Tag;

pub use crate::types::_hypervisor_details::HypervisorDetails;

pub use crate::types::_sync_metadata_status::SyncMetadataStatus;

pub use crate::types::_vmware_to_aws_tag_mapping::VmwareToAwsTagMapping;

pub use crate::types::_gateway::Gateway;

pub use crate::types::_gateway_type::GatewayType;

pub use crate::types::_gateway_details::GatewayDetails;

pub use crate::types::_maintenance_start_time::MaintenanceStartTime;

pub use crate::types::_bandwidth_rate_limit_interval::BandwidthRateLimitInterval;

mod _bandwidth_rate_limit_interval;

mod _gateway;

mod _gateway_details;

mod _gateway_type;

mod _hypervisor;

mod _hypervisor_details;

mod _hypervisor_state;

mod _maintenance_start_time;

mod _sync_metadata_status;

mod _tag;

mod _virtual_machine;

mod _virtual_machine_details;

mod _vmware_tag;

mod _vmware_to_aws_tag_mapping;

/// Builders
pub mod builders;

/// Error types that AWS Backup Gateway can respond with.
pub mod error;

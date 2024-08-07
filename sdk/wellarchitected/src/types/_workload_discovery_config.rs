// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Discovery configuration associated to the workload.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct WorkloadDiscoveryConfig {
    /// <p>Discovery integration status in respect to Trusted Advisor for the workload.</p>
    #[doc(hidden)]
    pub trusted_advisor_integration_status:
        std::option::Option<crate::types::TrustedAdvisorIntegrationStatus>,
}
impl WorkloadDiscoveryConfig {
    /// <p>Discovery integration status in respect to Trusted Advisor for the workload.</p>
    pub fn trusted_advisor_integration_status(
        &self,
    ) -> std::option::Option<&crate::types::TrustedAdvisorIntegrationStatus> {
        self.trusted_advisor_integration_status.as_ref()
    }
}
impl WorkloadDiscoveryConfig {
    /// Creates a new builder-style object to manufacture [`WorkloadDiscoveryConfig`](crate::types::WorkloadDiscoveryConfig).
    pub fn builder() -> crate::types::builders::WorkloadDiscoveryConfigBuilder {
        crate::types::builders::WorkloadDiscoveryConfigBuilder::default()
    }
}

/// A builder for [`WorkloadDiscoveryConfig`](crate::types::WorkloadDiscoveryConfig).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct WorkloadDiscoveryConfigBuilder {
    pub(crate) trusted_advisor_integration_status:
        std::option::Option<crate::types::TrustedAdvisorIntegrationStatus>,
}
impl WorkloadDiscoveryConfigBuilder {
    /// <p>Discovery integration status in respect to Trusted Advisor for the workload.</p>
    pub fn trusted_advisor_integration_status(
        mut self,
        input: crate::types::TrustedAdvisorIntegrationStatus,
    ) -> Self {
        self.trusted_advisor_integration_status = Some(input);
        self
    }
    /// <p>Discovery integration status in respect to Trusted Advisor for the workload.</p>
    pub fn set_trusted_advisor_integration_status(
        mut self,
        input: std::option::Option<crate::types::TrustedAdvisorIntegrationStatus>,
    ) -> Self {
        self.trusted_advisor_integration_status = input;
        self
    }
    /// Consumes the builder and constructs a [`WorkloadDiscoveryConfig`](crate::types::WorkloadDiscoveryConfig).
    pub fn build(self) -> crate::types::WorkloadDiscoveryConfig {
        crate::types::WorkloadDiscoveryConfig {
            trusted_advisor_integration_status: self.trusted_advisor_integration_status,
        }
    }
}

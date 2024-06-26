// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The list of information about logs to be enabled for the specified broker.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PendingLogs {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    #[doc(hidden)]
    pub audit: bool,
    /// <p>Enables general logging.</p>
    #[doc(hidden)]
    pub general: bool,
}
impl PendingLogs {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    pub fn audit(&self) -> bool {
        self.audit
    }
    /// <p>Enables general logging.</p>
    pub fn general(&self) -> bool {
        self.general
    }
}
impl PendingLogs {
    /// Creates a new builder-style object to manufacture [`PendingLogs`](crate::types::PendingLogs).
    pub fn builder() -> crate::types::builders::PendingLogsBuilder {
        crate::types::builders::PendingLogsBuilder::default()
    }
}

/// A builder for [`PendingLogs`](crate::types::PendingLogs).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PendingLogsBuilder {
    pub(crate) audit: std::option::Option<bool>,
    pub(crate) general: std::option::Option<bool>,
}
impl PendingLogsBuilder {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    pub fn audit(mut self, input: bool) -> Self {
        self.audit = Some(input);
        self
    }
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    pub fn set_audit(mut self, input: std::option::Option<bool>) -> Self {
        self.audit = input;
        self
    }
    /// <p>Enables general logging.</p>
    pub fn general(mut self, input: bool) -> Self {
        self.general = Some(input);
        self
    }
    /// <p>Enables general logging.</p>
    pub fn set_general(mut self, input: std::option::Option<bool>) -> Self {
        self.general = input;
        self
    }
    /// Consumes the builder and constructs a [`PendingLogs`](crate::types::PendingLogs).
    pub fn build(self) -> crate::types::PendingLogs {
        crate::types::PendingLogs {
            audit: self.audit.unwrap_or_default(),
            general: self.general.unwrap_or_default(),
        }
    }
}

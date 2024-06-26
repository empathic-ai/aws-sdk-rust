// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEventSourcesConfig`](crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigFluentBuilder::send) it.
    /// - On success, responds with [`DescribeEventSourcesConfigOutput`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigOutput) with field(s):
    ///   - [`event_sources(Option<EventSourcesConfig>)`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigOutput::event_sources): <p>Lists the event sources in the configuration.</p>
    /// - On failure, responds with [`SdkError<DescribeEventSourcesConfigError>`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigError)
    pub fn describe_event_sources_config(&self) -> crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigFluentBuilder{
        crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigFluentBuilder::new(self.handle.clone())
    }
}

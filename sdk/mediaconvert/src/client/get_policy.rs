// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPolicy`](crate::operation::get_policy::builders::GetPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_policy::builders::GetPolicyFluentBuilder::send) it.
    /// - On success, responds with [`GetPolicyOutput`](crate::operation::get_policy::GetPolicyOutput) with field(s):
    ///   - [`policy(Option<Policy>)`](crate::operation::get_policy::GetPolicyOutput::policy): A policy configures behavior that you allow or disallow for your account. For information about MediaConvert policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
    /// - On failure, responds with [`SdkError<GetPolicyError>`](crate::operation::get_policy::GetPolicyError)
    pub fn get_policy(&self) -> crate::operation::get_policy::builders::GetPolicyFluentBuilder {
        crate::operation::get_policy::builders::GetPolicyFluentBuilder::new(self.handle.clone())
    }
}

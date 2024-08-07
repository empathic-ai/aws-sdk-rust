// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRegistrationCode`](crate::operation::delete_registration_code::builders::DeleteRegistrationCodeFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::delete_registration_code::builders::DeleteRegistrationCodeFluentBuilder::send) it.
    /// - On success, responds with [`DeleteRegistrationCodeOutput`](crate::operation::delete_registration_code::DeleteRegistrationCodeOutput)
    /// - On failure, responds with [`SdkError<DeleteRegistrationCodeError>`](crate::operation::delete_registration_code::DeleteRegistrationCodeError)
    pub fn delete_registration_code(
        &self,
    ) -> crate::operation::delete_registration_code::builders::DeleteRegistrationCodeFluentBuilder
    {
        crate::operation::delete_registration_code::builders::DeleteRegistrationCodeFluentBuilder::new(self.handle.clone())
    }
}

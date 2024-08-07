// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccountSettings`](crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder::send) it.
    /// - On success, responds with [`GetAccountSettingsOutput`](crate::operation::get_account_settings::GetAccountSettingsOutput) with field(s):
    ///   - [`account_settings(Option<AccountSettings>)`](crate::operation::get_account_settings::GetAccountSettingsOutput::account_settings): <p>The account settings.</p>
    /// - On failure, responds with [`SdkError<GetAccountSettingsError>`](crate::operation::get_account_settings::GetAccountSettingsError)
    pub fn get_account_settings(
        &self,
    ) -> crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder {
        crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

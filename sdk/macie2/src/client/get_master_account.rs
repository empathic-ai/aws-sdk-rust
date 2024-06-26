// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMasterAccount`](crate::operation::get_master_account::builders::GetMasterAccountFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_master_account::builders::GetMasterAccountFluentBuilder::send) it.
    /// - On success, responds with [`GetMasterAccountOutput`](crate::operation::get_master_account::GetMasterAccountOutput) with field(s):
    ///   - [`master(Option<Invitation>)`](crate::operation::get_master_account::GetMasterAccountOutput::master): <p>(Deprecated) The Amazon Web Services account ID for the administrator account. If the accounts are associated by a Macie membership invitation, this object also provides details about the invitation that was sent to establish the relationship between the accounts.</p>
    /// - On failure, responds with [`SdkError<GetMasterAccountError>`](crate::operation::get_master_account::GetMasterAccountError)
    pub fn get_master_account(
        &self,
    ) -> crate::operation::get_master_account::builders::GetMasterAccountFluentBuilder {
        crate::operation::get_master_account::builders::GetMasterAccountFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

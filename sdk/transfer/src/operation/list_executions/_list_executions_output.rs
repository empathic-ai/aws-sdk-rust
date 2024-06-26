// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListExecutionsOutput {
    /// <p> <code>ListExecutions</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional executions.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A unique identifier for the workflow.</p>
    #[doc(hidden)]
    pub workflow_id: std::option::Option<std::string::String>,
    /// <p>Returns the details for each execution.</p>
    /// <ul>
    /// <li> <p> <b>NextToken</b>: returned from a call to several APIs, you can use pass it to a subsequent command to continue listing additional executions.</p> </li>
    /// <li> <p> <b>StartTime</b>: timestamp indicating when the execution began.</p> </li>
    /// <li> <p> <b>Executions</b>: details of the execution, including the execution ID, initial file location, and Service metadata.</p> </li>
    /// <li> <p> <b>Status</b>: one of the following values: <code>IN_PROGRESS</code>, <code>COMPLETED</code>, <code>EXCEPTION</code>, <code>HANDLING_EXEPTION</code>. </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub executions: std::option::Option<std::vec::Vec<crate::types::ListedExecution>>,
    _request_id: Option<String>,
}
impl ListExecutionsOutput {
    /// <p> <code>ListExecutions</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional executions.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(&self) -> std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
    /// <p>Returns the details for each execution.</p>
    /// <ul>
    /// <li> <p> <b>NextToken</b>: returned from a call to several APIs, you can use pass it to a subsequent command to continue listing additional executions.</p> </li>
    /// <li> <p> <b>StartTime</b>: timestamp indicating when the execution began.</p> </li>
    /// <li> <p> <b>Executions</b>: details of the execution, including the execution ID, initial file location, and Service metadata.</p> </li>
    /// <li> <p> <b>Status</b>: one of the following values: <code>IN_PROGRESS</code>, <code>COMPLETED</code>, <code>EXCEPTION</code>, <code>HANDLING_EXEPTION</code>. </p> </li>
    /// </ul>
    pub fn executions(&self) -> std::option::Option<&[crate::types::ListedExecution]> {
        self.executions.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListExecutionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListExecutionsOutput {
    /// Creates a new builder-style object to manufacture [`ListExecutionsOutput`](crate::operation::list_executions::ListExecutionsOutput).
    pub fn builder() -> crate::operation::list_executions::builders::ListExecutionsOutputBuilder {
        crate::operation::list_executions::builders::ListExecutionsOutputBuilder::default()
    }
}

/// A builder for [`ListExecutionsOutput`](crate::operation::list_executions::ListExecutionsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListExecutionsOutputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) workflow_id: std::option::Option<std::string::String>,
    pub(crate) executions: std::option::Option<std::vec::Vec<crate::types::ListedExecution>>,
    _request_id: Option<String>,
}
impl ListExecutionsOutputBuilder {
    /// <p> <code>ListExecutions</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional executions.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p> <code>ListExecutions</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional executions.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.workflow_id = Some(input.into());
        self
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn set_workflow_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// Appends an item to `executions`.
    ///
    /// To override the contents of this collection use [`set_executions`](Self::set_executions).
    ///
    /// <p>Returns the details for each execution.</p>
    /// <ul>
    /// <li> <p> <b>NextToken</b>: returned from a call to several APIs, you can use pass it to a subsequent command to continue listing additional executions.</p> </li>
    /// <li> <p> <b>StartTime</b>: timestamp indicating when the execution began.</p> </li>
    /// <li> <p> <b>Executions</b>: details of the execution, including the execution ID, initial file location, and Service metadata.</p> </li>
    /// <li> <p> <b>Status</b>: one of the following values: <code>IN_PROGRESS</code>, <code>COMPLETED</code>, <code>EXCEPTION</code>, <code>HANDLING_EXEPTION</code>. </p> </li>
    /// </ul>
    pub fn executions(mut self, input: crate::types::ListedExecution) -> Self {
        let mut v = self.executions.unwrap_or_default();
        v.push(input);
        self.executions = Some(v);
        self
    }
    /// <p>Returns the details for each execution.</p>
    /// <ul>
    /// <li> <p> <b>NextToken</b>: returned from a call to several APIs, you can use pass it to a subsequent command to continue listing additional executions.</p> </li>
    /// <li> <p> <b>StartTime</b>: timestamp indicating when the execution began.</p> </li>
    /// <li> <p> <b>Executions</b>: details of the execution, including the execution ID, initial file location, and Service metadata.</p> </li>
    /// <li> <p> <b>Status</b>: one of the following values: <code>IN_PROGRESS</code>, <code>COMPLETED</code>, <code>EXCEPTION</code>, <code>HANDLING_EXEPTION</code>. </p> </li>
    /// </ul>
    pub fn set_executions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ListedExecution>>,
    ) -> Self {
        self.executions = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListExecutionsOutput`](crate::operation::list_executions::ListExecutionsOutput).
    pub fn build(self) -> crate::operation::list_executions::ListExecutionsOutput {
        crate::operation::list_executions::ListExecutionsOutput {
            next_token: self.next_token,
            workflow_id: self.workflow_id,
            executions: self.executions,
            _request_id: self._request_id,
        }
    }
}

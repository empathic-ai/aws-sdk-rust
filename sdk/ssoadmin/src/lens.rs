// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_account_assignment_creation_status_output_next_token(
    input: &crate::operation::list_account_assignment_creation_status::ListAccountAssignmentCreationStatusOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_account_assignment_deletion_status_output_next_token(
    input: &crate::operation::list_account_assignment_deletion_status::ListAccountAssignmentDeletionStatusOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_account_assignments_output_next_token(
    input: &crate::operation::list_account_assignments::ListAccountAssignmentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_accounts_for_provisioned_permission_set_output_next_token(
    input: &crate::operation::list_accounts_for_provisioned_permission_set::ListAccountsForProvisionedPermissionSetOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_customer_managed_policy_references_in_permission_set_output_next_token(
    input: &crate::operation::list_customer_managed_policy_references_in_permission_set::ListCustomerManagedPolicyReferencesInPermissionSetOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_instances_output_next_token(
    input: &crate::operation::list_instances::ListInstancesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_managed_policies_in_permission_set_output_next_token(
    input: &crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_permission_set_provisioning_status_output_next_token(
    input: &crate::operation::list_permission_set_provisioning_status::ListPermissionSetProvisioningStatusOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_permission_sets_output_next_token(
    input: &crate::operation::list_permission_sets::ListPermissionSetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_permission_sets_provisioned_to_account_output_next_token(
    input: &crate::operation::list_permission_sets_provisioned_to_account::ListPermissionSetsProvisionedToAccountOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_tags_for_resource_output_next_token(
    input: &crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_account_assignment_creation_status_output_account_assignments_creation_status(
    input: crate::operation::list_account_assignment_creation_status::ListAccountAssignmentCreationStatusOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AccountAssignmentOperationStatusMetadata>> {
    let input = match input.account_assignments_creation_status {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_account_assignment_deletion_status_output_account_assignments_deletion_status(
    input: crate::operation::list_account_assignment_deletion_status::ListAccountAssignmentDeletionStatusOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AccountAssignmentOperationStatusMetadata>> {
    let input = match input.account_assignments_deletion_status {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_account_assignments_output_account_assignments(
    input: crate::operation::list_account_assignments::ListAccountAssignmentsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AccountAssignment>> {
    let input = match input.account_assignments {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_accounts_for_provisioned_permission_set_output_account_ids(
    input: crate::operation::list_accounts_for_provisioned_permission_set::ListAccountsForProvisionedPermissionSetOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.account_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_customer_managed_policy_references_in_permission_set_output_customer_managed_policy_references(
    input: crate::operation::list_customer_managed_policy_references_in_permission_set::ListCustomerManagedPolicyReferencesInPermissionSetOutput,
) -> std::option::Option<std::vec::Vec<crate::types::CustomerManagedPolicyReference>> {
    let input = match input.customer_managed_policy_references {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_instances_output_instances(
    input: crate::operation::list_instances::ListInstancesOutput,
) -> std::option::Option<std::vec::Vec<crate::types::InstanceMetadata>> {
    let input = match input.instances {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_managed_policies_in_permission_set_output_attached_managed_policies(
    input: crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AttachedManagedPolicy>> {
    let input = match input.attached_managed_policies {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_permission_set_provisioning_status_output_permission_sets_provisioning_status(
    input: crate::operation::list_permission_set_provisioning_status::ListPermissionSetProvisioningStatusOutput,
) -> std::option::Option<std::vec::Vec<crate::types::PermissionSetProvisioningStatusMetadata>> {
    let input = match input.permission_sets_provisioning_status {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_permission_sets_output_permission_sets(
    input: crate::operation::list_permission_sets::ListPermissionSetsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.permission_sets {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_permission_sets_provisioned_to_account_output_permission_sets(
    input: crate::operation::list_permission_sets_provisioned_to_account::ListPermissionSetsProvisionedToAccountOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.permission_sets {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_tags_for_resource_output_tags(
    input: crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Tag>> {
    let input = match input.tags {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

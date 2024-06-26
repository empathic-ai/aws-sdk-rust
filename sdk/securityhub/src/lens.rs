// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_action_targets_output_next_token(
    input: &crate::operation::describe_action_targets::DescribeActionTargetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_describe_products_output_next_token(
    input: &crate::operation::describe_products::DescribeProductsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_describe_standards_output_next_token(
    input: &crate::operation::describe_standards::DescribeStandardsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_describe_standards_controls_output_next_token(
    input: &crate::operation::describe_standards_controls::DescribeStandardsControlsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_get_enabled_standards_output_next_token(
    input: &crate::operation::get_enabled_standards::GetEnabledStandardsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_get_findings_output_next_token(
    input: &crate::operation::get_findings::GetFindingsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_get_insights_output_next_token(
    input: &crate::operation::get_insights::GetInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_enabled_products_for_import_output_next_token(
    input: &crate::operation::list_enabled_products_for_import::ListEnabledProductsForImportOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_finding_aggregators_output_next_token(
    input: &crate::operation::list_finding_aggregators::ListFindingAggregatorsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_invitations_output_next_token(
    input: &crate::operation::list_invitations::ListInvitationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_members_output_next_token(
    input: &crate::operation::list_members::ListMembersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_organization_admin_accounts_output_next_token(
    input: &crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_security_control_definitions_output_next_token(
    input: &crate::operation::list_security_control_definitions::ListSecurityControlDefinitionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_standards_control_associations_output_next_token(
    input: &crate::operation::list_standards_control_associations::ListStandardsControlAssociationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_action_targets_output_action_targets(
    input: crate::operation::describe_action_targets::DescribeActionTargetsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ActionTarget>> {
    let input = match input.action_targets {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_products_output_products(
    input: crate::operation::describe_products::DescribeProductsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Product>> {
    let input = match input.products {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_standards_output_standards(
    input: crate::operation::describe_standards::DescribeStandardsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Standard>> {
    let input = match input.standards {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_standards_controls_output_controls(
    input: crate::operation::describe_standards_controls::DescribeStandardsControlsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::StandardsControl>> {
    let input = match input.controls {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_get_enabled_standards_output_standards_subscriptions(
    input: crate::operation::get_enabled_standards::GetEnabledStandardsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::StandardsSubscription>> {
    let input = match input.standards_subscriptions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_get_findings_output_findings(
    input: crate::operation::get_findings::GetFindingsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AwsSecurityFinding>> {
    let input = match input.findings {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_get_insights_output_insights(
    input: crate::operation::get_insights::GetInsightsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Insight>> {
    let input = match input.insights {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_enabled_products_for_import_output_product_subscriptions(
    input: crate::operation::list_enabled_products_for_import::ListEnabledProductsForImportOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.product_subscriptions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_finding_aggregators_output_finding_aggregators(
    input: crate::operation::list_finding_aggregators::ListFindingAggregatorsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::FindingAggregator>> {
    let input = match input.finding_aggregators {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_invitations_output_invitations(
    input: crate::operation::list_invitations::ListInvitationsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Invitation>> {
    let input = match input.invitations {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_members_output_members(
    input: crate::operation::list_members::ListMembersOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Member>> {
    let input = match input.members {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_organization_admin_accounts_output_admin_accounts(
    input: crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AdminAccount>> {
    let input = match input.admin_accounts {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_security_control_definitions_output_security_control_definitions(
    input: crate::operation::list_security_control_definitions::ListSecurityControlDefinitionsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::SecurityControlDefinition>> {
    let input = match input.security_control_definitions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_standards_control_associations_output_standards_control_association_summaries(
    input: crate::operation::list_standards_control_associations::ListStandardsControlAssociationsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::StandardsControlAssociationSummary>> {
    let input = match input.standards_control_association_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

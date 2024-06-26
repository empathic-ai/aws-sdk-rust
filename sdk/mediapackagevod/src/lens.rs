// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_assets_output_next_token(
    input: &crate::operation::list_assets::ListAssetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_packaging_configurations_output_next_token(
    input: &crate::operation::list_packaging_configurations::ListPackagingConfigurationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_packaging_groups_output_next_token(
    input: &crate::operation::list_packaging_groups::ListPackagingGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_assets_output_assets(
    input: crate::operation::list_assets::ListAssetsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AssetShallow>> {
    let input = match input.assets {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_packaging_configurations_output_packaging_configurations(
    input: crate::operation::list_packaging_configurations::ListPackagingConfigurationsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::PackagingConfiguration>> {
    let input = match input.packaging_configurations {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_packaging_groups_output_packaging_groups(
    input: crate::operation::list_packaging_groups::ListPackagingGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::PackagingGroup>> {
    let input = match input.packaging_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

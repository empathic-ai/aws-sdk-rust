// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_client_devices_associated_with_core_device_output_next_token(
    input: &crate::operation::list_client_devices_associated_with_core_device::ListClientDevicesAssociatedWithCoreDeviceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_component_versions_output_next_token(
    input: &crate::operation::list_component_versions::ListComponentVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_components_output_next_token(
    input: &crate::operation::list_components::ListComponentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_core_devices_output_next_token(
    input: &crate::operation::list_core_devices::ListCoreDevicesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_deployments_output_next_token(
    input: &crate::operation::list_deployments::ListDeploymentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_effective_deployments_output_next_token(
    input: &crate::operation::list_effective_deployments::ListEffectiveDeploymentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_installed_components_output_next_token(
    input: &crate::operation::list_installed_components::ListInstalledComponentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_client_devices_associated_with_core_device_output_associated_client_devices(
    input: crate::operation::list_client_devices_associated_with_core_device::ListClientDevicesAssociatedWithCoreDeviceOutput,
) -> std::option::Option<std::vec::Vec<crate::types::AssociatedClientDevice>> {
    let input = match input.associated_client_devices {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_component_versions_output_component_versions(
    input: crate::operation::list_component_versions::ListComponentVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ComponentVersionListItem>> {
    let input = match input.component_versions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_components_output_components(
    input: crate::operation::list_components::ListComponentsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Component>> {
    let input = match input.components {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_core_devices_output_core_devices(
    input: crate::operation::list_core_devices::ListCoreDevicesOutput,
) -> std::option::Option<std::vec::Vec<crate::types::CoreDevice>> {
    let input = match input.core_devices {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_deployments_output_deployments(
    input: crate::operation::list_deployments::ListDeploymentsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Deployment>> {
    let input = match input.deployments {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_effective_deployments_output_effective_deployments(
    input: crate::operation::list_effective_deployments::ListEffectiveDeploymentsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::EffectiveDeployment>> {
    let input = match input.effective_deployments {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_installed_components_output_installed_components(
    input: crate::operation::list_installed_components::ListInstalledComponentsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::InstalledComponent>> {
    let input = match input.installed_components {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

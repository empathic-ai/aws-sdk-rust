// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_enabled_controls_output_next_token(
    input: &crate::operation::list_enabled_controls::ListEnabledControlsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_enabled_controls_output_enabled_controls(
    input: crate::operation::list_enabled_controls::ListEnabledControlsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::EnabledControlSummary>> {
    let input = match input.enabled_controls {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

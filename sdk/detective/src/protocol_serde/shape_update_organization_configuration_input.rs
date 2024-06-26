// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_organization_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_organization_configuration::UpdateOrganizationConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.auto_enable {
        object.key("AutoEnable").boolean(input.auto_enable);
    }
    if let Some(var_1) = &input.graph_arn {
        object.key("GraphArn").string(var_1.as_str());
    }
    Ok(())
}

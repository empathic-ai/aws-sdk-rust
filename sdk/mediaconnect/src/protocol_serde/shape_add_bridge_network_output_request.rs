// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_bridge_network_output_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AddBridgeNetworkOutputRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ip_address {
        object.key("ipAddress").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.network_name {
        object.key("networkName").string(var_3.as_str());
    }
    {
        object.key("port").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.port).into()),
        );
    }
    if let Some(var_4) = &input.protocol {
        object.key("protocol").string(var_4.as_str());
    }
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    Ok(())
}

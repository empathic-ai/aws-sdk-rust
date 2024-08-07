// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_bridge_network_output_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateBridgeNetworkOutputRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ip_address {
        object.key("ipAddress").string(var_1.as_str());
    }
    if let Some(var_2) = &input.network_name {
        object.key("networkName").string(var_2.as_str());
    }
    if input.port != 0 {
        object.key("port").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.port).into()),
        );
    }
    if let Some(var_3) = &input.protocol {
        object.key("protocol").string(var_3.as_str());
    }
    if input.ttl != 0 {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    Ok(())
}

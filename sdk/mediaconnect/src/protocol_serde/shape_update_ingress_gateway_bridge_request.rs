// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_ingress_gateway_bridge_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateIngressGatewayBridgeRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_bitrate != 0 {
        object.key("maxBitrate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_bitrate).into()),
        );
    }
    if input.max_outputs != 0 {
        object.key("maxOutputs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_outputs).into()),
        );
    }
    Ok(())
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bucket_criteria_additional_properties(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::BucketCriteriaAdditionalProperties,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.eq {
        let mut array_2 = object.key("eq").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if input.gt != 0 {
        object.key("gt").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.gt).into()),
        );
    }
    if input.gte != 0 {
        object.key("gte").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.gte).into()),
        );
    }
    if input.lt != 0 {
        object.key("lt").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.lt).into()),
        );
    }
    if input.lte != 0 {
        object.key("lte").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.lte).into()),
        );
    }
    if let Some(var_4) = &input.neq {
        let mut array_5 = object.key("neq").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.prefix {
        object.key("prefix").string(var_7.as_str());
    }
    Ok(())
}

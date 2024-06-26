// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_cases_output_next_token(
    input: &crate::operation::describe_cases::DescribeCasesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_describe_communications_output_next_token(
    input: &crate::operation::describe_communications::DescribeCommunicationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_cases_output_cases(
    input: crate::operation::describe_cases::DescribeCasesOutput,
) -> std::option::Option<std::vec::Vec<crate::types::CaseDetails>> {
    let input = match input.cases {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_describe_communications_output_communications(
    input: crate::operation::describe_communications::DescribeCommunicationsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Communication>> {
    let input = match input.communications {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

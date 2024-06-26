// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_human_loops_output_next_token(
    input: &crate::operation::list_human_loops::ListHumanLoopsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_human_loops_output_human_loop_summaries(
    input: crate::operation::list_human_loops::ListHumanLoopsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::HumanLoopSummary>> {
    let input = match input.human_loop_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

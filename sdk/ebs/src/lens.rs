// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_changed_blocks_output_next_token(
    input: &crate::operation::list_changed_blocks::ListChangedBlocksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_snapshot_blocks_output_next_token(
    input: &crate::operation::list_snapshot_blocks::ListSnapshotBlocksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

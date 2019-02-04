use crate::{
    nucleus::ribosome::{api::ZomeApiResult, Runtime},
    workflows::remove_link::remove_link_workflow,
};
use futures::executor::block_on;
use holochain_core_types::{entry::Entry, error::HolochainError, link::link_add::LinkAdd};
use holochain_wasm_utils::api_serialization::link_entries::LinkEntriesArgs;
use std::convert::TryFrom;
use wasmi::{RuntimeArgs, RuntimeValue};

/// ZomeApiFunction::GetLinks function code
/// args: [0] encoded MemoryAllocation as u64
/// Expected complex argument: GetLinksArgs
/// Returns an HcApiReturnCode as I64
pub fn invoke_remove_link(runtime: &mut Runtime, args: &RuntimeArgs) -> ZomeApiResult {
    // deserialize args
    let args_str = runtime.load_json_string_from_args(&args);
    let input = match LinkEntriesArgs::try_from(args_str.clone()) {
        Ok(entry_input) => entry_input,
        // Exit on error
        Err(_) => {
            runtime.context.log(format!(
                "err/zome: invoke_link_entries failed to deserialize LinkEntriesArgs: {:?}",
                args_str
            ));
            return ribosome_error_code!(ArgumentDeserializationFailed);
        }
    };

    let link = input.to_link();
    let link_add = LinkAdd::from_link(&link);
    let entry = Entry::LinkRemove(link_add);

    // Wait for future to be resolved
    let result: Result<(), HolochainError> =
        block_on(remove_link_workflow(&entry, &runtime.context)).map(|_| ());

    runtime.store_result(result)
}
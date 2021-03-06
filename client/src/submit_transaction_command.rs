use crate::{client_proxy::ClientProxy, commands::*};

pub struct SubmitTransactionFromDiskCommand {}

impl Command for SubmitTransactionFromDiskCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["submit", "submitb", "s", "sb"]
    }
    fn get_description(&self) -> &'static str {
        "Load a RawTransaction from file and submit to the network"
    }
    fn get_params_help(&self) -> &'static str {
        "\n\t<signer_account_address>|<signer_account_ref_id> <path_to_raw_transaction> Suffix 'b' is for blocking. "
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!(
                "Invalid number of arguments for submitting transaction, got {}",
                params.len()
            );
            return;
        }
        let is_blocking = blocking_cmd(&params[0]);
        match client.submit_transaction_from_disk(params, is_blocking) {
            Ok(index_and_seq) => {
                if is_blocking {
                    println!("Finished transaction!");
                } else {
                    println!("Transaction submitted to validator");
                }
                println!(
                    "To query for transaction status, run: query txn_acc_seq {} {} \
                     <fetch_events=true|false>",
                    index_and_seq.account_index, index_and_seq.sequence_number
                );
            }
            Err(e) => report_error("Failed to perform transaction", e),
        }
    }
}

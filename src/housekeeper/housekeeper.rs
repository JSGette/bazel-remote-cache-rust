use re_grpc::remote_execution::ActionResult as ActionResult;
use re_grpc::remote_execution::Digest as Digest;
use std::collections::HashMap;
use std::vec::Vec;

struct Configuration {
    ac_path: String,
    cas_path: String
}

impl Configuration {
    fn new_configuration(ac_path: String, cas_path: String) -> Configuration {
        Configuration {
            ac_path,
            cas_path
        }
    }
}

fn read_ac(ac_path: String) -> Vec<ActionResult> {
    Vec::new()
}

fn read_cas(cas_path: String) -> Vec<Digest> {
    Vec::new()
}

fn link_blobs_to_action_result(actions_results: Vec<ActionResult>, 
    blobs: Vec<Digest>) -> HashMap<ActionResult, Vec<Digest>> {
        HashMap::new()
}

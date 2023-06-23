use log::info;
use tfhe::shortint::{gen_keys, parameters, ClientKey, ServerKey};

pub fn generate_keys() -> (ClientKey, ServerKey) {
    info!("Generating keys");
    gen_keys(parameters::PARAM_MESSAGE_4_CARRY_0)
}

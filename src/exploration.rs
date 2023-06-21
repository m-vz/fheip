use log::info;
use tfhe::boolean::prelude::BinaryBooleanGates;
use tfhe::prelude::{FheDecrypt, FheEncrypt};
use tfhe::shortint::parameters;
use tfhe::{boolean, integer, shortint, ConfigBuilder};

#[allow(unused)]
pub fn high_level() {
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = tfhe::generate_keys(config);

    tfhe::set_server_key(server_key);

    let (a_plain, b_plain) = (27, 128);
    let (a, b) = (
        tfhe::FheUint8::encrypt(a_plain, &client_key),
        tfhe::FheUint8::encrypt(b_plain, &client_key),
    );
    let result = a + b; // this is an FHE operation, + is overloaded
    let result_plain: u8 = result.decrypt(&client_key);

    info!("result = {}", result_plain);
    assert_eq!(result_plain, a_plain + b_plain);
}

#[allow(unused)]
pub fn boolean() {
    let (client_key, server_key) = boolean::gen_keys();

    // execute circuit `if ((NOT b) NAND (a AND b)) then (NOT b) else (a AND b)`
    let (a_plain, b_plain) = (true, false);
    let (a, b) = (client_key.encrypt(a_plain), client_key.encrypt(b_plain));
    let not_b = server_key.not(&b);
    let a_and_b = server_key.and(&a, &b);
    let condition = server_key.nand(&not_b, &a_and_b);
    let result = server_key.mux(&condition, &not_b, &a_and_b);
    let result_plain = client_key.decrypt(&result);

    info!("result = {}", result_plain);
    assert!(result_plain);
}

#[allow(unused)]
pub fn shortint() {
    let (client_key, server_key) = shortint::gen_keys(parameters::PARAM_MESSAGE_2_CARRY_2);
    let modulus = client_key.parameters.message_modulus.0;

    let (a_plain, b_plain) = (1, 2);
    let (a, b) = (client_key.encrypt(a_plain), client_key.encrypt(b_plain));
    let result = server_key.unchecked_add(&a, &b);
    let result_plain = client_key.decrypt(&result);

    info!("modulus = {}, result = {}", modulus, result_plain);
    assert_eq!(result_plain, (a_plain + b_plain) % modulus as u64);
}

#[allow(unused)]
pub fn integer() {
    let (client_key, server_key) = integer::gen_keys_radix(&parameters::PARAM_MESSAGE_2_CARRY_2, 8);

    let (a_plain, b_plain) = (2382, 29374);
    let (mut a, mut b) = (client_key.encrypt(a_plain), client_key.encrypt(b_plain));
    let max = server_key.smart_max_parallelized(&mut a, &mut b);
    let max_plain: u64 = client_key.decrypt(&max);

    info!("result = {}", max_plain);
    assert_eq!(max_plain, a_plain.max(b_plain));
}

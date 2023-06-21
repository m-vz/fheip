use log::info;
use tfhe::prelude::FheDecrypt;
use tfhe::prelude::FheEncrypt;
use tfhe::ConfigBuilder;

pub fn high_level_example() {
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = tfhe::generate_keys(config);

    tfhe::set_server_key(server_key);

    let (a, b) = (27, 128);
    let (a_cipher, b_cipher) = (
        tfhe::FheUint8::encrypt(a, &client_key),
        tfhe::FheUint8::encrypt(b, &client_key),
    );
    let result_cipher = a_cipher + b_cipher; // this is an FHE operation, + is overloaded
    let result: u8 = result_cipher.decrypt(&client_key);

    info!("result = {}", result);
    assert_eq!(result, a + b);
}

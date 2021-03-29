fn main() {
    let init_subject_number = 7;
    let card_public_key = 7573546;
    let door_public_key = 17786549;
    const MOD: usize = 20201227;

    let mut loop_size = 0;
    let mut value = 1;
    while value != card_public_key {
        loop_size += 1;
        value = (value * init_subject_number) % MOD;
    }

    let card_loop_size = loop_size;
    let mut value = 1;
    for _ in 0..card_loop_size {
        value = (value * door_public_key) % MOD;
    }
    let encryption_key = value;

    println!(
        "Encryption key: {}, card_loop_size: {}",
        encryption_key, card_loop_size
    );
}

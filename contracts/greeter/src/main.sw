contract;

abi Greeter {
    #[storage(read, write)]
    fn set_value(new_value: u64);

    #[storage(read)]
    fn get_value() -> u64;
}

struct ValueSet {
    value: u64,
}

storage {
    last_value: u64 = 0,
}

impl Greeter for Contract {
    #[storage(read, write)]
    fn set_value(new_value: u64) {
        storage.last_value.write(new_value);
        log(ValueSet { value: new_value });
    }

    #[storage(read)]
    fn get_value() -> u64 {
        storage.last_value.read()
    }
}
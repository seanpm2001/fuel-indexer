extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[no_mangle]
fn ff_log_data(_inp: ()) {}

#[no_mangle]
fn ff_put_object(_inp: ()) {}

#[no_mangle]
fn ff_put_many_to_many_record(_inp: ()) {}

#[no_mangle]
fn ff_early_exit(_inp: ()) {}

#[indexer(manifest = "packages/fuel-indexer-tests/trybuild/simple_wasm_unsupported.yaml")]
mod indexer {
    fn function_one(event: SomeEvent) {
        let SomeEvent { id, account } = event;

        assert_eq!(id, 9);
        assert_eq!(account, Bits256([48u8; 32]));
    }
}

fn main() {
    // We're not actually testing the serialization of the event from the ABI JSON here, 
    // we're just testing that this compiles.
}
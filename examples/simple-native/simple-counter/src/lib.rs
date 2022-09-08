extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(
    abi = "examples/simple-native/contracts/counter/out/debug/counter-abi.json",
    namespace = "counter",
    schema = "../schema/counter.graphql"
)]
mod simple_native {
    fn count_handler(event: CountEvent) {
        // Using the Count entity from the GraphQL schema
        let count = Count {
            id: event.id,
            timestamp: event.timestamp,
            count: event.count,
        };

        count.save()
    }
}
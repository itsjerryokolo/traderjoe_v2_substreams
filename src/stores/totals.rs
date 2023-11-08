#[substreams::handlers::store]
pub fn store_pairs(i: traderjoe_v2::Pairs, o: StoreSetProto<traderjoe_v2::Pair>) {
    for pair in i.pairs {
        o.set(0, generate_key("Pair", &pair.address), &pair);
    }
}
#[substreams::handlers::store]
pub fn store_totals(pair: traderjoe_v2::Pairs, swap: traderjoe_v2::Swaps) {
    for s in swap.swaps {}
}

// -------------------
//  Map Pool Entities
// -------------------
pub fn pools_created_pool_entity_changes(tables: &mut Tables, pools: &Pools) {
    for pool in &pools.pools {
        create_pool(tables, pool);
    }
}

fn create_pool(tables: &mut Tables, pool: &Pool) {
    let bigint0 = BigInt::zero();
    let bigdecimal0 = BigDecimal::zero();
    tables
        .create_row("Pool", format!("0x{}", &pool.address))
        .set(
            "createdAtTimestamp",
            BigInt::from(pool.created_at_timestamp),
        )
        .set("createdAtBlockNumber", pool.created_at_block_number)
        .set(
            "token0",
            format!("0x{}", pool.token0.as_ref().unwrap().address),
        )
        .set(
            "token1",
            format!("0x{}", pool.token1.as_ref().unwrap().address),
        )
        .set_bigint("feeTier", &pool.fee_tier)
        .set("liquidity", &bigint0)
        .set("sqrtPrice", &bigint0)
        .set("feeGrowthGlobal0X128", &bigint0)
        .set("feeGrowthGlobal1X128", &bigint0)
        .set("token0Price", &bigdecimal0)
        .set("token1Price", &bigdecimal0)
        .set("tick", &bigint0)
        .set("observationIndex", &bigint0)
        .set("volumeToken0", &bigdecimal0)
        .set("volumeToken1", &bigdecimal0)
        .set("volumeUSD", &bigdecimal0)
        .set("untrackedVolumeUSD", &bigdecimal0)
        .set("feesUSD", &bigdecimal0)
        .set("txCount", &bigint0)
        .set("collectedFeesToken0", &bigdecimal0)
        .set("collectedFeesToken1", &bigdecimal0)
        .set("collectedFeesUSD", &bigdecimal0)
        .set("totalValueLockedToken0", &bigdecimal0)
        .set("totalValueLockedToken1", &bigdecimal0)
        .set("totalValueLockedETH", &bigdecimal0)
        .set("totalValueLockedUSD", &bigdecimal0)
        .set("totalValueLockedUSDUntracked", &bigdecimal0)
        .set("totalValueLockedETHUntracked", &bigdecimal0)
        .set("liquidityProviderCount", &bigint0);
}

fn create_pool_windows_entity(
    tables: &mut Tables,
    table_name: &str,
    time_id: i64,
    pool_time_id: &String,
    pool_addr: &str,
) {
    let row = tables
        .update_row(table_name, pool_time_id)
        .set("pool", format!("0x{}", pool_addr))
        .set("liquidity", BigInt::zero())
        .set("sqrtPrice", BigInt::zero())
        .set("token0Price", BigDecimal::zero())
        .set("token1Price", BigDecimal::zero())
        .set("tick", BigInt::zero())
        .set("feeGrowthGlobal0X128", BigInt::zero())
        .set("feeGrowthGlobal1X128", BigInt::zero())
        .set("totalValueLockedUSD", BigDecimal::zero())
        .set("volumeToken0", BigDecimal::zero())
        .set("volumeToken1", BigDecimal::zero())
        .set("volumeUSD", BigDecimal::zero())
        .set("feesUSD", BigDecimal::zero())
        .set("txCount", BigInt::zero())
        .set("open", BigDecimal::zero())
        .set("high", BigDecimal::zero())
        .set("low", BigDecimal::zero())
        .set("close", BigDecimal::zero());

    match table_name {
        "PoolDayData" => {
            row.set("date", (time_id * 86400) as i32);
        }
        "PoolHourData" => {
            row.set("periodStartUnix", (time_id * 3600) as i32);
        }
        _ => {}
    }
}

pub fn liquidities_pool_entity_change(
    tables: &mut Tables,
    pool_liquidities_store_deltas: &Deltas<DeltaBigInt>,
) {
    for delta in pool_liquidities_store_deltas
        .iter()
        .key_first_segment_eq("pool")
    {
        let pool_address = key::segment_at(&delta.key, 1);
        tables
            .update_row("Pool", &format!("0x{pool_address}"))
            .set("liquidity", &delta.new_value);
    }
}

pub fn total_value_locked_pool_entity_change(
    tables: &mut Tables,
    derived_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    for delta in derived_tvl_deltas
        .iter()
        .operation_eq(Operation::Create)
        .key_first_segment_eq("pool")
        .key_last_segment_in([
            "totalValueLockedUSD",
            "totalValueLockedETH",
            "totalValueLockedUSDUntracked",
            "totalValueLockedETHUntracked",
        ])
    {
        let pool_address = key::segment_at(&delta.key, 1);
        tables
            .update_row("Pool", &format!("0x{pool_address}"))
            .set(key::last_segment(&delta.key), &delta.new_value);
    }
}

pub fn total_value_locked_by_token_pool_entity_change(
    tables: &mut Tables,
    token_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    for delta in token_tvl_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        // TODO: maybe change the field name on the key itself??
        let field_name = match key::last_segment(&delta.key) {
            "token0" => "totalValueLockedToken0",
            "token1" => "totalValueLockedToken1",
            _ => continue,
        };
        tables
            .update_row("Pool", &format!("0x{pool_address}"))
            .set(field_name, &delta.new_value);
    }
}

pub fn tx_count_pool_entity_change(tables: &mut Tables, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        tables
            .update_row("Pool", &format!("0x{pool_address}"))
            .set("txCount", &delta.new_value);
    }
}

pub fn swap_volume_pool_entity_change(
    tables: &mut Tables,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
) {
    for delta in swaps_volume_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        // TODO: maybe change the field name on the key itself??
        let field_name = match key::last_segment(&delta.key) {
            "volumeToken0" => "volumeToken0",
            "volumeToken1" => "volumeToken1",
            "volumeUSD" => "volumeUSD",
            "volumeUntrackedUSD" => "untrackedVolumeUSD",
            "feesUSD" => "feesUSD",
            "liquidityProviderCount" => "liquidityProviderCount",
            _ => continue,
        };

        if field_name == "liquidityProviderCount" {
            tables
                .update_row("Pool", &format!("0x{pool_address}"))
                .set("liquidityProviderCount", &delta.new_value.to_bigint());
            continue;
        } else {
            tables
                .update_row("Pool", &format!("0x{pool_address}"))
                .set(field_name, &delta.new_value);
        }
    }
}

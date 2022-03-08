// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use fasthash::murmur3::hash32_with_seed;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeResult},
    pop_arg,
    values::Value,
};
use std::collections::VecDeque;

pub fn native_nbits(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    //debug_assert!(ty_args.is_empty());
    //debug_assert!(args.len() == 1);

    let n = pop_arg!(args, u64) as usize;

    let cost = native_gas(context.cost_table(), NativeCostIndex::SHA2_256, 1);

    let m = get_m(n, 0.01);

    NativeResult::map_partial_vm_result_one(cost, Ok(move_vm_types::values::Value::u64(m as u64)))
}

pub fn native_num_of_hashfuncs(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    //debug_assert!(ty_args.is_empty());
    //debug_assert!(args.len() == 1);

    let m = pop_arg!(args, u64) as usize;
    let n = pop_arg!(args, u64) as usize;

    let cost = native_gas(context.cost_table(), NativeCostIndex::SHA2_256, 1);

    let k = get_k(m, n);

    NativeResult::map_partial_vm_result_one(cost, Ok(move_vm_types::values::Value::u64(k as u64)))
}

pub fn native_hash(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    //debug_assert!(ty_args.is_empty());
    //debug_assert!(args.len() == 1);

    let i = pop_arg!(args, u64);
    let data = pop_arg!(args, Vec<u8>);

    let cost = native_gas(context.cost_table(), NativeCostIndex::SHA2_256, 1);

    let hash = hash32_with_seed(data, i as u32);

    NativeResult::map_partial_vm_result_one(
        cost,
        Ok(move_vm_types::values::Value::u64(hash as u64)),
    )
}
pub fn get_m(n: usize, p: f64) -> usize {
    let numerator = n as f64 * p.ln();
    let denominator = (1.0_f64 / 2.0_f64.powf(2.0_f64.ln())).ln();
    (numerator / denominator).ceil() as usize
}

#[inline(always)]
pub fn get_k(m: usize, n: usize) -> usize {
    ((m as f64 / n as f64) * 2.0_f64.ln()).round() as usize
}

// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use failure::Error;

use crypto::hash::HashType;
use storage::*;
use storage::tests_common::TmpStorage;
use tezos_context::channel::ContextAction;

#[test]
fn context_get_values_by_block_hash() -> Result<(), Error> {
    let tmp_storage = TmpStorage::create("__ctx_storage_get_by_block_hash")?;

    let str_block_hash_1 = "BKyQ9EofHrgaZKENioHyP4FZNsTmiSEcVmcghgzCC9cGhE7oCET";
    let block_hash_1 = HashType::BlockHash.string_to_bytes(str_block_hash_1)?;
    let str_block_hash_2 = "BLaf78njreWdt2WigJjM9e3ecEdVKm5ehahUfYBKvcWvZ8vfTcJ";
    let block_hash_2 = HashType::BlockHash.string_to_bytes(str_block_hash_2)?;
    let value_1_0 = ContextRecordValue { action: ContextAction::Set { key: vec!("hello".to_string(), "this".to_string(), "is".to_string(), "dog".to_string()), value: vec![10, 200], operation_hash: None, block_hash: Some(str_block_hash_1.into()), context_hash: None, value_as_json: None, start_time: 0.0, end_time: 0.0 } };
    let value_1_1 = ContextRecordValue { action: ContextAction::Set { key: vec!("hello".to_string(), "world".to_string()), value: vec![11, 200], operation_hash: None, block_hash: Some(str_block_hash_1.into()), context_hash: None, value_as_json: None, start_time: 0.0, end_time: 0.0 } };
    let value_2_0 = ContextRecordValue { action: ContextAction::Set { key: vec!("nice".to_string(), "to meet you".to_string()), value: vec![20, 200], operation_hash: None, block_hash: Some(str_block_hash_2.into()), context_hash: None, value_as_json: None, start_time: 0.0, end_time: 0.0 } };

    let mut storage = ContextStorage::new(tmp_storage.storage());
    storage.put(&block_hash_1, &value_1_0)?;
    storage.put(&block_hash_2, &value_2_0)?;
    storage.put(&block_hash_1, &value_1_1)?;

    // block hash 1
    let values = storage.get_by_block_hash(&block_hash_1)?;
    assert_eq!(2, values.len(), "Was expecting vector of {} elements but instead found {}", 2, values.len());
    if let ContextAction::Set { value, .. } = &values[0].action {
        assert_eq!(&vec![10, 200], value);
    } else {
        panic!("Was expecting ContextAction::Set");
    }
    if let ContextAction::Set { value, .. } = &values[1].action {
        assert_eq!(&vec![11, 200], value);
    } else {
        panic!("Was expecting ContextAction::Set");
    }
    // block hash 2
    let values = storage.get_by_block_hash(&block_hash_2)?;
    assert_eq!(1, values.len(), "Was expecting vector of {} elements but instead found {}", 1, values.len());
    if let ContextAction::Set { value, .. } = &values[0].action {
        assert_eq!(&vec![20, 200], value);
    } else {
        panic!("Was expecting ContextAction::Set");
    }

    Ok(())
}
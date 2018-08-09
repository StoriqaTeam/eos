//! Review smart contract.
//!
//! A simple smart contract implementaion that affords to create, read and update
//! reviews hash on EOS blockchain

#![no_std]
#![deny(
    missing_docs,
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

extern crate eos;

mod models;

use eos::bindings::action::*;
use eos::bindings::console::*;
use eos::bindings::db::*;

use models::{ReadReviewAction, Review};

const TABLE_NAME: u64 = 1;

/// Init review smart contract
#[no_mangle]
pub extern "C" fn init() {
    print_str("Deployed");
}

/// Do some stuff with review smart contract
#[no_mangle]
pub extern "C" fn apply(receiver: u64, _code: u64, action: u64) {
    match name_to_str(action).as_ref() {
        "review.add" => {
            if let Ok(review) = read_action::<Review>() {
                review_add(receiver, review);
            } else {
                print_str("Failed to deserialize data for `review.add` action\n");
            }
        }
        "review.read" => {
            if let Ok(ReadReviewAction { id }) = read_action::<ReadReviewAction>() {
                review_read(receiver, id);
            } else {
                print_str("Failed to deserialize data for `review.read` action\n");
            }
        }
        "review.upd" => {
            if let Ok(review) = read_action::<Review>() {
                review_update(receiver, review);
            } else {
                print_str("Failed to deserialize data for `review.upd` action\n");
            }
        }
        _ => print_str("No such action\n"),
    }
}

fn review_add(receiver: u64, review: Review) {
    print_str("Received action `review.add` for id: ");
    print_u64(review.id);
    print_str("\n");
    db_store(receiver, TABLE_NAME, receiver, review.id, &review);
}

fn review_update(receiver: u64, mut review: Review) {
    print_str("Received action `review.update` for id: ");
    print_u64(review.id);
    print_str("\n");
    db_update(receiver, receiver, receiver, TABLE_NAME, review.id, &mut review);
}

fn review_read(receiver: u64, id: u64) {
    print_str("Received action `review.read` for id: ");
    print_u64(id);
    print_str("\n");
    if let Ok(review) = db_read::<Review>(receiver, receiver, TABLE_NAME, id) {
        print_str("Found review with id: ");
        print_u64(review.id);
        print_str("\n");
        print_str("byte1: ");
        print_u64(review.byte1);
        print_str("\n");
        print_str("byte2: ");
        print_u64(review.byte2);
        print_str("\n");
        print_str("byte3: ");
        print_u64(review.byte3);
        print_str("\n");
        print_str("byte4: ");
        print_u64(review.byte4);
        print_str("\n");
    } else {
        print_str("Unable to read data from db\n");
    }
}

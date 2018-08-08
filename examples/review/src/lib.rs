//! Review smart contract.
//!
//! A simple smart contract implementaion that affords to create, read and update
//! reviews hash on EOS blockchain
//!

#![deny(warnings)]
extern crate eos;

const TABLE_NAME: u64 = 1;

use eos::console::*;
use eos::db::*;

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

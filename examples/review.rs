//! Review smart contract.
//!
//! A simple smart contract implementaion that affords to create, read and update
//! reviews hash on EOS blockchain
//!
//! You can test this out by running:
//!
//!     cargo run --example review
//!

#![deny(warnings)]

extern crate alloc;
extern crate eos_rust;

const TABLE_NAME: u64 = 1;

fn review_add(receiver: u64, review: Review) {
    eos_rust::print_str("Received action `review.add` for id: ");
    eos_rust::print_u64(review.id);
    eos_rust::print_str("\n");
    eos_rust::db_store(receiver, TABLE_NAME, receiver, review.id, &review);
}

fn review_update(receiver: u64, mut review: Review) {
    eos_rust::print_str("Received action `review.update` for id: ");
    eos_rust::print_u64(review.id);
    eos_rust::print_str("\n");
    eos_rust::db_update(receiver, receiver, receiver, TABLE_NAME, review.id, &mut review);
}

fn review_read(receiver: u64, id: u64) {
    eos_rust::print_str("Received action `review.read` for id: ");
    eos_rust::print_u64(id);
    eos_rust::print_str("\n");
    if let Ok(review) = eos_rust::db_read::<Review>(receiver, receiver, TABLE_NAME, id) {
        eos_rust::print_str("Found review with id: ");
        eos_rust::print_u64(review.id);
        eos_rust::print_str("\n");
        eos_rust::print_str("byte1: ");
        eos_rust::print_u64(review.byte1);
        eos_rust::print_str("\n");
        eos_rust::print_str("byte2: ");
        eos_rust::print_u64(review.byte2);
        eos_rust::print_str("\n");
        eos_rust::print_str("byte3: ");
        eos_rust::print_u64(review.byte3);
        eos_rust::print_str("\n");
        eos_rust::print_str("byte4: ");
        eos_rust::print_u64(review.byte4);
        eos_rust::print_str("\n");
    } else {
        eos_rust::print_str("Unable to read data from db\n");
    }
}

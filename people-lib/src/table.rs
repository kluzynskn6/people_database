extern crate database_lib as dbl;
// Table operations
// create
//  Take user without id, put into table, new id gets generated
// read
//  Take id and get user
// update
//  Take partially filled user with id and put into database, overriding filled in fields
// delete
//  Take id and remove from database
//

/**
 *
 * Represents a join table
 *
 * A join table is where two things from different tables get related.
 * For example, a User Department join table would have User keys and Database keys. Each entry
 * would relate one user to one department. Note that there may be more than one entry for each
 * user or department
 *
 */

pub trait JoinTable {
    type KeyA;
    type KeyB;

    fn lookup_a(&self, key: Self::KeyA) -> Self::KeyB;
    fn lookup_b(&self, key: Self::KeyB) -> Self::KeyA;
}

/**
 * A table used for testing
 * It does not actually store anything anywhere
 */

pub struct TestTable {

}


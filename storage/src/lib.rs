// Nearssage
// Copyright (C) 2023 Oscar

pub mod entry;
pub mod object;

pub use entry::*;
pub use object::*;

use nearssage_commons::*;
use nearssage_schema::*;

use std::mem::transmute;

use anyhow::{Context, Result};
use async_trait::async_trait;
use atomic_refcell::AtomicRefCell;
use either::*;
use paste::paste;
use rclite::Arc;
use redb::*;
use tokio::sync::OnceCell;
use tokio::task::JoinSet;

pub static DB: OnceCell<AtomicRefCell<Arc<Database>>> = OnceCell::const_new();

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn object() -> Result<()> {
        // Create a temporal file to store the DB
        let tmpfile = tempfile::NamedTempFile::new()?;

        // Create a DB
        DB.set(AtomicRefCell::new(Arc::new(Database::create(
            tmpfile.path(),
        )?)))?;

        // Generate keys
        let keys: SVec<UserID> = (0..32).map(|_| rand::random::<UserID>()).collect();

        // Insert data
        let mut inserts: JoinSet<Result<()>> = JoinSet::new();
        for key in &keys {
            inserts.spawn(Preferences::insert(*key, Preference::default()));
        }
        while inserts.join_next().await.is_some() {}

        // Check whether all the data has been inserted
        assert_eq!(
            keys.len(),
            DB.get()
                .context("Cannot get DB handle")?
                .borrow()
                .begin_read()?
                .open_table(Preferences::TABLE_DEFINITION)?
                .len()? as usize
        );

        // Insert unique value
        let (unique_id, unique_value) = (
            rand::random::<UserID>(),
            Preference::new(Some(Gender::Female), None, LocationRange::default()),
        );
        Preferences::insert(unique_id, unique_value.clone()).await?;

        // Get the unique value
        let retrieved_unique_value = Preferences::get(unique_id)
            .await?
            .context("Unique value not found!")?;

        // Check whether the retrieved value is the unique value
        assert!(
            unique_value == retrieved_unique_value,
            "Unique value and retrieved value aren't the same!"
        );

        // Search the unique value's id by itself
        let found_id = Preferences::search(retrieved_unique_value, true)?
            .unwrap_left()
            .context("Unique value's id not found!")?;

        // Check whether the unique value's id and the id found are the same
        assert_eq!(unique_id, found_id);

        Ok(())
    }
}

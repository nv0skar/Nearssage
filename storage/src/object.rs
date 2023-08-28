// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

macro_rules! object {
    ($name:ident, $key_type:ty, $value_type:ty) => {
        paste! {
            pub struct [< $name:camel >];

            impl<'a> Object<'a> for $name {
                type Key = $key_type;
                type Value = $value_type;

                const TABLE_DEFINITION: TableDefinition<'a, &'static [u8], &'static [u8]> =
                    TableDefinition::new(stringify!([< $name:lower >]));
            }
        }
    };
}

object!(Usernames, UserID, Username);
object!(Reaches, UserID, Reach);
object!(Statuses, UserID, Status);
object!(Profiles, UserID, Profile);
object!(Preferences, UserID, Preference);
object!(Sessions, UserID, Session);
object!(Devices, DeviceID, Device);
object!(Passwords, UserID, Password);

#[async_trait]
pub trait Object<'a> {
    type Key: Key;
    type Value: Value;

    const TABLE_DEFINITION: TableDefinition<'a, &'static [u8], &'static [u8]>;

    /// Gets value by id
    async fn get(id: Self::Key) -> Result<Option<Self::Value>> {
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_read()?;
        let table = txn.open_table(Self::TABLE_DEFINITION)?;
        let value = table.get(id.as_slice())?;
        Ok(match value {
            Some(value) => Some(Self::Value::decode(value.value()).await?),
            None => None,
        })
    }

    /// Inserts value by id
    async fn insert(id: Self::Key, value: Self::Value) -> Result<()> {
        let value = value.encode().await?;
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_write()?;
        {
            let mut table = txn.open_table(Self::TABLE_DEFINITION)?;
            table.insert(id.as_slice(), value.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Updates value by id
    async fn update(id: Self::Key, value: Self::Value) -> Result<()> {
        let value = value.encode().await?;
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_write()?;
        {
            let mut table = txn.open_table(Self::TABLE_DEFINITION)?;
            table.get(id.as_slice())?.context("Key doesn't exist!")?;
            table.insert(id.as_slice(), value.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Removes value by id
    async fn remove(id: Self::Key) -> Result<()> {
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_write()?;
        {
            let mut table = txn.open_table(Self::TABLE_DEFINITION)?;
            table.remove(id.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Check if id exists
    async fn exists(id: Self::Key) -> Result<bool> {
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_read()?;
        let table = txn.open_table(Self::TABLE_DEFINITION)?;
        let exists = table.get(id.as_slice())?.is_some();
        Ok(exists)
    }

    /// Searches ids with the matching value
    fn search(
        predicate: Self::Value,
        unique: bool,
    ) -> Result<Either<Option<Self::Key>, SVec<Self::Key>>> {
        let db = DB.get().context("Cannot get DB handle!")?.borrow();
        let txn = db.begin_read()?;
        let table = txn.open_table(Self::TABLE_DEFINITION)?;
        let mut search: JoinSet<Result<Option<Self::Key>>> = JoinSet::new();
        for entry in table.iter()? {
            let (id, value) = entry?;
            let predicate = predicate.clone();

            let id =
                unsafe { transmute::<AccessGuard<'_, &[u8]>, AccessGuard<'static, &[u8]>>(id) };
            let value =
                unsafe { transmute::<AccessGuard<'_, &[u8]>, AccessGuard<'static, &[u8]>>(value) };

            search.spawn(async move {
                if Self::Value::decode(&value.value()).await?.eq(&predicate) {
                    Ok(Some(Self::Key::into_key(&id.value())?))
                } else {
                    Ok(None)
                }
            });
        }
        match unique {
            true => {
                while let Some(result) = futures::executor::block_on(search.join_next()) {
                    if let Some(key) = result?? {
                        search.abort_all();
                        return Ok(Left(Some(key)));
                    }
                }
                Ok(Left(None))
            }
            false => {
                let mut found = SVec::new();
                while let Some(result) = futures::executor::block_on(search.join_next()) {
                    if let Some(key) = result?? {
                        found.push(key);
                    }
                }
                Ok(Right(found))
            }
        }
    }
}

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TodoStatus {
    New,
    Started,
    Complete,
}
impl Default for TodoStatus {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    /// _key is required to identify the document
    _key: String,
    title: String,
    timestamp: i64,
    status: TodoStatus,
}
impl Todo {
    pub fn new(title: &str) -> Self {
        // timestamp in millisec
        let now = time::OffsetDateTime::now_utc().timestamp() * 1000;
        Todo {
            _key: String::new(),
            title: title.to_owned(),
            timestamp: now,
            status: TodoStatus::New,
        }
    }
    pub fn back_date(&mut self, date: &time::OffsetDateTime) {
        let now = date.to_offset(time::offset!(+0)).timestamp() * 1000;
        self.timestamp = now;
    }
}

pub trait List<E: Serialize> {
    /// Lists elements of Self up to limit.
    /// Returns anything for Error of type E which can be Serialized.
    fn list(limit: u64) -> Result<Vec<Self>, E>
    where
        Self: Sized + Serialize;
}
pub trait Fetch<E: Serialize> {
    /// Fetch Self by key.
    /// Returns anything for Error of type E which can be Serialized.
    fn fetch(key: &str) -> Result<Self, E>
    where
        Self: Sized + Serialize;
}
pub trait Create<I: DeserializeOwned, E: Serialize> {
    /// Create Self based on I input, which mush be convertible to Self with into().
    /// Returns anything for Error of type E which can be Serialized.
    fn create(data: I) -> Result<Self, E>
    where
        Self: Sized + Serialize,
        I: Into<Self>;
}
pub trait Update<I: DeserializeOwned, E: Serialize> {
    /// Update Self based on I input, which mush be convertible to Self with into().
    /// Returns anything for Error of type E which can be Serialized.
    /// Note: To be able to implement this, the type Self should have some key, or other unique identifier.
    fn update(data: I) -> Result<Self, E>
    where
        Self: Sized + Serialize;
}
pub trait Replace<I: DeserializeOwned, E: Serialize> {
    /// Replace Self based on I input, which mush be convertible to Self with into().
    /// Returns anything for Error of type E which can be Serialized.
    /// Note: To be able to implement this, the type Self should have some key, or other unique identifier.
    fn replace(data: I) -> Result<Self, E>
    where
        Self: Sized + Serialize,
        I: Into<Self>;
}
pub trait Delete<E: Serialize> {
    /// Update Self based on I input, which mush be convertible to Self with into().
    /// Returns anything for Error of type E which can be Serialized.
    /// Note: To be able to implement this, the type Self should have some key, or other unique identifier.
    fn delete(key: &str) -> Result<Self, E>
    where
        Self: Sized + Serialize;
}

// TODO: move these to it's own file
use serde_json::{json, Value};
use sled::Config;

impl List<Value> for Todo {
    fn list(limit: u64) -> Result<Vec<Self>, Value> {
        // TODO: use a named tree instead
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            let collection = t.iter();

            let mut res = vec![];
            for item in collection {
                if let Ok(item) = &item {
                    if let Ok(doc) = serde_cbor::from_slice::<Todo>(&item.1) {
                        // Since ret.len() is usize, this may fail on a larger than 64bit target architecture, let's worry about it when this code needs to run on such a machine.
                        if (res.len() as u64) < limit {
                            res.push(doc);
                        } else {
                            return Ok(res);
                        }
                    }
                }
            }
            Ok(res)
        } else {
            Err(json!("Could not open database."))
        }
    }
}

impl Fetch<Value> for Todo {
    fn fetch(key: &str) -> Result<Self, Value> {
        // TODO: use a named tree instead.
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            // TODO: get rid of these unwraps.
            let encoded_stored = t.get(key).unwrap().unwrap();
            let decoded: Todo = serde_cbor::from_slice(&encoded_stored).unwrap();
            Ok(decoded)
        } else {
            Err(json!("Could not open database."))
        }
    }
}

impl Create<Todo, Value> for Todo {
    fn create(data: Todo) -> Result<Self, Value> {
        // TODO: use a named tree instead.
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            // TODO: get rid of these unwraps.
            // Find the last entry.
            let last = t.iter().next_back().unwrap().unwrap();
            let idx_vec: Vec<u8> = last.0.to_vec();
            let idx: u64 = String::from_utf8(idx_vec)
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap()
                + 1;
            let new_key = idx.to_string();

            let data = Todo {
                _key: new_key.clone(),
                ..data
            };

            let encoded = serde_cbor::to_vec(&data).unwrap();
            if t.insert(new_key.as_bytes(), encoded).is_ok() {
                Ok(data)
            } else {
                Err(json!("Could not write new document to the database."))
            }
        } else {
            Err(json!("Could not open the database."))
        }
    }
}

impl Update<Value, Value> for Todo {
    fn update(data: Value) -> Result<Self, Value> {
        // TODO: use a named tree instead.
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            // TODO: get rid of these unwraps.
            if let Some(key) = data["_key"].as_str() {
                let encoded_stored = t.get(key).unwrap().unwrap();
                let mut decoded_val: Value = serde_cbor::from_slice(&encoded_stored).unwrap();
                // Patch the data.
                json_patch::merge(&mut decoded_val, &data);
                // Do not let _key change.
                *decoded_val.get_mut("_key").unwrap() = json!(key.clone());

                let decoded: Todo = serde_json::from_value(decoded_val).unwrap();
                let encoded = serde_cbor::to_vec(&decoded).unwrap();
                if t.insert(key.as_bytes(), encoded).is_ok() {
                    Ok(decoded)
                } else {
                    Err(json!("Could not write new document to the database."))
                }
            } else {
                Err(json!("Input document doesn't have a _key."))
            }
        } else {
            Err(json!("Could not open the database."))
        }
    }
}

impl Replace<Todo, Value> for Todo {
    fn replace(data: Todo) -> Result<Self, Value> {
        // TODO: use a named tree instead.
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            // TODO: get rid of these unwraps.
            let key: String = data._key.clone();

            let encoded = serde_cbor::to_vec(&data).unwrap();
            if t.insert(key.as_bytes(), encoded).is_ok() {
                Ok(data)
            } else {
                Err(json!("Could not write new document to the database."))
            }
        } else {
            Err(json!("Could not open the database."))
        }
    }
}

impl Delete<Value> for Todo {
    fn delete(key: &str) -> Result<Self, Value> {
        // TODO: use a named tree instead.
        let config = Config::new().temporary(true);
        if let Ok(t) = config.open() {
            // TODO: get rid of these unwraps.
            let encoded_stored = t.remove(key).unwrap().unwrap();
            let decoded: Todo = serde_cbor::from_slice(&encoded_stored).unwrap();
            Ok(decoded)
        } else {
            Err(json!("Could not open database."))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

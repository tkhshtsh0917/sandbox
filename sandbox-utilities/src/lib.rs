use anyhow::{Context as _, Result};
use serde::{Deserialize, Deserializer};
use serde_json::from_str;

#[derive(Deserialize)]
pub struct OuterFormat {
    #[serde(
        rename(deserialize = "inners"),
        deserialize_with = "deserialize_max_age"
    )]
    pub max_age: u8,
}

impl OuterFormat {
    pub fn new(max_age: u8) -> Self {
        OuterFormat { max_age }
    }
}

#[derive(Deserialize)]
pub struct InnerFormat<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> InnerFormat<'a> {
    pub fn new<T: Into<String>>(name: &'a str, age: u8) -> Self {
        InnerFormat { name, age }
    }
}

#[derive(Deserialize)]
struct Wrapper<'a>(#[serde(borrow = "'a")] InnerFormat<'a>);

fn deserialize_max_age<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let vector = Vec::deserialize(deserializer)?;

    let parsed = vector
        .into_iter()
        .map(|Wrapper(item)| item)
        .map(|item| item.age)
        .max()
        .expect("Parse error...");

    Ok(parsed)
}

#[forbid(unsafe_code)]
pub fn read_json() -> Result<OuterFormat> {
    let json = r#"
    {
        "inners": [
            {
                "name": "John",
                "age": 10
            },
            {
                "name": "Paul",
                "age": 15
            },
            {
                "name": "George",
                "age": 20
            },
            {
                "name": "Ringo",
                "age": 25
            }
        ]
    }
    "#;

    from_str(json).with_context(|| "Failed to reading JSON data...")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_json() {
        let expected = OuterFormat::new(25);
        let actual = read_json().unwrap();

        assert_eq!(expected.max_age, actual.max_age);
    }
}

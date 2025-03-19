use std::path::PathBuf;
use crate::{GameClass, PropertyValue, ClassProperty};

// Implement conversions from strings to PropertyValue
impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::String(value.to_string())
    }
}

// Implement conversions from numbers to PropertyValue
impl From<i64> for PropertyValue {
    fn from(value: i64) -> Self {
        PropertyValue::Number(value)
    }
}

impl From<i32> for PropertyValue {
    fn from(value: i32) -> Self {
        PropertyValue::Number(value as i64)
    }
}

// Implement conversions from arrays to PropertyValue
impl From<Vec<String>> for PropertyValue {
    fn from(value: Vec<String>) -> Self {
        PropertyValue::Array(value)
    }
}

impl<'a> From<&'a [String]> for PropertyValue {
    fn from(value: &'a [String]) -> Self {
        PropertyValue::Array(value.to_vec())
    }
}

impl<'a> From<&'a [&str]> for PropertyValue {
    fn from(value: &'a [&str]) -> Self {
        PropertyValue::Array(value.iter().map(|s| s.to_string()).collect())
    }
}

// Builder implementation for GameClass
pub struct GameClassBuilder {
    name: String,
    parent: Option<String>,
    file_path: PathBuf,
    container_class: Option<String>,
    properties: Vec<ClassProperty>,
}

impl GameClassBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            file_path: PathBuf::new(),
            container_class: None,
            properties: Vec::new(),
        }
    }
    
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    
    pub fn file_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.file_path = path.into();
        self
    }
    
    pub fn container_class(mut self, container: impl Into<String>) -> Self {
        self.container_class = Some(container.into());
        self
    }
    
    pub fn add_property(mut self, name: impl Into<String>, value: impl Into<PropertyValue>) -> Self {
        self.properties.push(ClassProperty {
            name: name.into(),
            value: value.into(),
        });
        self
    }
    
    pub fn build(self) -> GameClass {
        GameClass {
            name: self.name,
            parent: self.parent,
            file_path: self.file_path,
            container_class: self.container_class,
            properties: self.properties,
        }
    }
} 
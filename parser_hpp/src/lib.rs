use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use hemtt_config::{Config, parse, Property, Class, Value, Array, Item};
use hemtt_preprocessor::Processor;
use hemtt_workspace::{reporting::{Codes, Processed, Code, Diagnostic, Severity}, LayerType, Workspace, WorkspacePath};
use serde::{Serialize, Deserialize};
use tempfile::NamedTempFile;

mod parser;
pub use parser::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HppClass {
    pub name: String,
    pub parent: Option<String>,
    pub properties: Vec<HppProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HppProperty {
    pub name: String,
    pub value: HppValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HppValue {
    String(String),
    Array(Vec<String>),
    Number(i64),
    Class(HppClass),
}

pub struct HppParser {
    config: Config,
}

/// Parse an HPP file and return a vector of classes.
/// 
/// # Arguments
/// 
/// * `file_path` - Path to the HPP file to parse
/// 
/// # Returns
/// 
/// * `Result<Vec<HppClass>, Codes>` - List of classes found in the file or error
pub fn parse_file(file_path: &std::path::Path) -> Result<Vec<HppClass>, Codes> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|_| vec![])?;
    
    let parser = HppParser::new(&content)?;
    Ok(parser.parse_classes())
}

impl HppParser {
    pub fn new(content: &str) -> Result<Self, Codes> {
        // Create a temporary workspace with the content
        let temp_file = NamedTempFile::new().map_err(|_| vec![])?;
        fs::write(temp_file.path(), content).map_err(|_| vec![])?;
        
        let parent_path = PathBuf::from(temp_file.path().parent().unwrap());
        let workspace = Workspace::builder()
            .physical(&parent_path, LayerType::Source)
            .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
            .map_err(|_| vec![])?;
            
        let path = workspace.join(temp_file.path().file_name().unwrap().to_str().unwrap()).map_err(|_| vec![])?;
        let processed = match Processor::run(&path) {
            Ok(processed) => processed,
            Err((_, e)) => {
                #[derive(Debug)]
                struct ProcessorError(hemtt_preprocessor::Error);
                impl Code for ProcessorError {
                    fn message(&self) -> String { self.0.to_string() }
                    fn severity(&self) -> Severity { Severity::Error }
                    fn diagnostic(&self) -> Option<Diagnostic> { None }
                    fn ident(&self) -> &'static str { "processor_error" }
                }
                return Err(vec![Arc::new(ProcessorError(e))]);
            }
        };
        let report = parse(None, &processed)?;
        
        Ok(Self {
            config: report.into_config(),
        })
    }

    /// Parse all classes and return them as a flat list
    pub fn parse_classes(&self) -> Vec<HppClass> {
        let mut classes = Vec::new();
        println!("\n=== Starting class parsing ===");
        self.extract_classes(&self.config, &mut classes);
        println!("\n=== Final class list ===");
        for class in &classes {
            println!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        classes
    }

    fn extract_classes(&self, config: &Config, classes: &mut Vec<HppClass>) {
        for property in config.0.iter() {
            match property {
                Property::Class(class) => {
                    match class {
                        Class::Local { name, parent, properties, .. } => {
                            println!("Processing Local class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                            
                            // First process any forward declarations in this class
                            for prop in properties {
                                if let Property::Class(Class::External { name, .. }) = prop {
                                    println!("  Found forward declaration: {}", name.as_str());
                                    classes.push(HppClass {
                                        name: name.as_str().to_string(),
                                        parent: None,
                                        properties: Vec::new(),
                                    });
                                }
                            }

                            // Add the class itself
                            let mut hpp_class = HppClass {
                                name: name.as_str().to_string(),
                                parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                properties: Vec::new(),
                            };

                            // Extract properties from the class
                            for prop in properties {
                                match prop {
                                    Property::Entry { name, value, .. } => {
                                        println!("  Adding property: {}", name.as_str());
                                        hpp_class.properties.push(HppProperty {
                                            name: name.as_str().to_string(),
                                            value: self.convert_value(value),
                                        });
                                    }
                                    Property::Class(nested_class) => {
                                        if let Class::Local { name, parent, properties, .. } = nested_class {
                                            println!("  Processing nested class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                            let mut nested_hpp_class = HppClass {
                                                name: name.as_str().to_string(),
                                                parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                                properties: Vec::new(),
                                            };

                                            // Extract properties from the nested class
                                            for prop in properties {
                                                match prop {
                                                    Property::Entry { name, value, .. } => {
                                                        nested_hpp_class.properties.push(HppProperty {
                                                            name: name.as_str().to_string(),
                                                            value: self.convert_value(value),
                                                        });
                                                    }
                                                    Property::Class(inner_class) => {
                                                        if let Class::Local { name, parent, properties, .. } = inner_class {
                                                            let inner_hpp_class = HppClass {
                                                                name: name.as_str().to_string(),
                                                                parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                                                properties: properties.iter().filter_map(|p| {
                                                                    if let Property::Entry { name, value, .. } = p {
                                                                        Some(HppProperty {
                                                                            name: name.as_str().to_string(),
                                                                            value: self.convert_value(value),
                                                                        })
                                                                    } else {
                                                                        None
                                                                    }
                                                                }).collect(),
                                                            };
                                                            nested_hpp_class.properties.push(HppProperty {
                                                                name: name.as_str().to_string(),
                                                                value: HppValue::Class(inner_hpp_class.clone()),
                                                            });
                                                            
                                                            // If the inner class has a parent, also add it as a top-level class
                                                            if inner_hpp_class.parent.is_some() {
                                                                classes.push(inner_hpp_class);
                                                            }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }

                                            // If the nested class has a parent, add it as a top-level class
                                            if nested_hpp_class.parent.is_some() {
                                                classes.push(nested_hpp_class.clone());
                                            }

                                            // Also add it as a property of the parent class
                                            hpp_class.properties.push(HppProperty {
                                                name: name.as_str().to_string(),
                                                value: HppValue::Class(nested_hpp_class),
                                            });
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            classes.push(hpp_class);
                        }
                        Class::External { name, .. } => {
                            println!("Processing External class (forward declaration): {}", name.as_str());
                            // Handle forward declarations
                            classes.push(HppClass {
                                name: name.as_str().to_string(),
                                parent: None,
                                properties: Vec::new(),
                            });
                        }
                        Class::Root { properties, .. } => {
                            println!("Processing Root class with {} properties", properties.len());
                            
                            // First process any forward declarations in the root
                            for prop in properties {
                                if let Property::Class(Class::External { name, .. }) = prop {
                                    println!("  Found forward declaration in root: {}", name.as_str());
                                    classes.push(HppClass {
                                        name: name.as_str().to_string(),
                                        parent: None,
                                        properties: Vec::new(),
                                    });
                                }
                            }

                            // Then process regular classes
                            for prop in properties {
                                if let Property::Class(nested_class) = prop {
                                    if let Class::Local { name, parent, properties, .. } = nested_class {
                                        println!("  Found local class in root: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                        let mut hpp_class = HppClass {
                                            name: name.as_str().to_string(),
                                            parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                            properties: Vec::new(),
                                        };

                                        // Extract properties from the class
                                        for prop in properties {
                                            match prop {
                                                Property::Entry { name, value, .. } => {
                                                    println!("    Adding property: {}", name.as_str());
                                                    hpp_class.properties.push(HppProperty {
                                                        name: name.as_str().to_string(),
                                                        value: self.convert_value(value),
                                                    });
                                                }
                                                Property::Class(inner_class) => {
                                                    if let Class::Local { name, parent, properties, .. } = inner_class {
                                                        println!("    Processing nested class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                                        let mut inner_hpp_class = HppClass {
                                                            name: name.as_str().to_string(),
                                                            parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                                            properties: Vec::new(),
                                                        };

                                                        // Extract properties from the inner class
                                                        for prop in properties {
                                                            if let Property::Entry { name, value, .. } = prop {
                                                                inner_hpp_class.properties.push(HppProperty {
                                                                    name: name.as_str().to_string(),
                                                                    value: self.convert_value(value),
                                                                });
                                                            }
                                                        }

                                                        // If the inner class has a parent, add it as a top-level class
                                                        if inner_hpp_class.parent.is_some() {
                                                            classes.push(inner_hpp_class.clone());
                                                        }

                                                        // Also add it as a property of the parent class
                                                        hpp_class.properties.push(HppProperty {
                                                            name: name.as_str().to_string(),
                                                            value: HppValue::Class(inner_hpp_class),
                                                        });
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }

                                        // If the class has a parent, add it as a top-level class
                                        if hpp_class.parent.is_some() {
                                            classes.push(hpp_class.clone());
                                        }

                                        // Also add it as a property of the root class
                                        classes.push(HppClass {
                                            name: "CfgWeapons".to_string(),
                                            parent: None,
                                            properties: vec![HppProperty {
                                                name: name.as_str().to_string(),
                                                value: HppValue::Class(hpp_class),
                                            }],
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn convert_value(&self, value: &Value) -> HppValue {
        match value {
            Value::Str(s) => HppValue::String(s.value().to_string()),
            Value::Number(n) => {
                match n {
                    hemtt_config::Number::Int32 { value, .. } => HppValue::Number(*value as i64),
                    hemtt_config::Number::Int64 { value, .. } => HppValue::Number(*value),
                    hemtt_config::Number::Float32 { value, .. } => HppValue::Number(*value as i64),
                }
            }
            Value::Array(arr) => {
                let mut values = Vec::new();
                for item in arr.items() {
                    match item {
                        Item::Str(s) => values.push(s.value().to_string()),
                        Item::Macro((macro_name, item, _)) => {
                            let count = macro_name.value()
                                .strip_prefix("LIST_")
                                .and_then(|n| n.parse::<usize>().ok())
                                .unwrap_or(1);
                            for _ in 0..count {
                                values.push(item.value().to_string());
                            }
                        }
                        Item::Eval { expression, .. } => values.push(expression.value().to_string()),
                        _ => {}
                    }
                }
                HppValue::Array(values)
            }
            _ => HppValue::String(String::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_class_parsing() {
        let content = r#"
            class BaseMan {
                displayName = "Unarmed";
                uniform[] = {"uniform1", "uniform2"};
                items[] = {"item1", "item2"};
            };
        "#;

        let parser = HppParser::new(content).unwrap();
        let classes = parser.parse_classes();

        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "BaseMan");
        assert_eq!(classes[0].properties.len(), 3);
    }

    #[test]
    fn test_inheritance() {
        let content = r#"
            class BaseMan {
                displayName = "Base";
            };
            class Rifleman : BaseMan {
                displayName = "Rifleman";
            };
        "#;

        let parser = HppParser::new(content).unwrap();
        let classes = parser.parse_classes();

        assert_eq!(classes.len(), 2);
        assert_eq!(classes[1].parent.as_deref(), Some("BaseMan"));
    }

    #[test]
    fn test_array_with_list_macro() {
        let content = r#"
            class Test {
                uniform[] = {
                    LIST_2("usp_g3c_kp_mx_aor2"),
                    "usp_g3c_rs_kp_mx_aor2",
                    "usp_g3c_rs2_kp_mx_aor2"
                };
            };
        "#;
        let parser = HppParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let test_class = &classes[0];
        assert_eq!(test_class.name, "Test");
        
        let uniform_prop = test_class.properties.iter().find(|p| p.name == "uniform").unwrap();
        if let HppValue::Array(uniforms) = &uniform_prop.value {
            assert!(uniforms.contains(&"usp_g3c_kp_mx_aor2".to_string()));
            assert!(uniforms.contains(&"usp_g3c_rs_kp_mx_aor2".to_string()));
            assert!(uniforms.contains(&"usp_g3c_rs2_kp_mx_aor2".to_string()));
            assert_eq!(uniforms.len(), 4); // Should have 4 items because LIST_2 expands to 2
        } else {
            panic!("Expected uniform to be an array");
        }
    }
} 
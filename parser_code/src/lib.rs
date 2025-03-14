use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use hemtt_config::{Config, parse, Property, Class, Value, Array, Item};
use hemtt_preprocessor::Processor;
use hemtt_workspace::{reporting::{Codes, Processed, Code, Diagnostic, Severity}, LayerType, Workspace, WorkspacePath};
use serde::{Serialize, Deserialize};
use tempfile::NamedTempFile;
use log::debug;

mod parser;
pub use parser::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeClass {
    pub name: String,
    pub parent: Option<String>,
    pub properties: Vec<CodeProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeProperty {
    pub name: String,
    pub value: CodeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeValue {
    String(String),
    Array(Vec<String>),
    Number(i64),
    Class(CodeClass),
}

pub struct CodeParser {
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
/// * `Result<Vec<CodeClass>, Codes>` - List of classes found in the file or error
pub fn parse_file(file_path: &std::path::Path) -> Result<Vec<CodeClass>, Codes> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|_| vec![])?;
    
    let parser = CodeParser::new(&content)?;
    Ok(parser.parse_classes())
}

impl CodeParser {
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
    pub fn parse_classes(&self) -> Vec<CodeClass> {
        let mut classes = Vec::new();
        debug!("\n=== Starting class parsing ===");
        self.extract_classes(&self.config, &mut classes);
        debug!("\n=== Final class list ===");
        for class in &classes {
            debug!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        classes
    }

    fn extract_classes(&self, config: &Config, classes: &mut Vec<CodeClass>) {
        for property in config.0.iter() {
            if let Property::Class(class) = property {
                match class {
                    Class::Local { name, parent, properties, .. } => {
                        debug!("Processing Local class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                        
                        // Process forward declarations
                        self.process_forward_declarations(properties, classes);
                        
                        // Create and add the class
                        let code_class = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, false);
                        classes.push(code_class);
                    },
                    Class::External { name, .. } => {
                        debug!("Processing External class (forward declaration): {}", name.as_str());
                        // Handle forward declarations
                        classes.push(CodeClass {
                            name: name.as_str().to_string(),
                            parent: None,
                            properties: Vec::new(),
                        });
                    },
                    Class::Root { properties, .. } => {
                        debug!("Processing Root class with {} properties", properties.len());
                        
                        // Process forward declarations in root
                        self.process_forward_declarations(properties, classes);
                        
                        // Process regular classes in root
                        for prop in properties {
                            if let Property::Class(nested_class) = prop {
                                if let Class::Local { name, parent, properties, .. } = nested_class {
                                    debug!("  Found local class in root: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                    
                                    let code_class = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, true);
                                    
                                    // Also add it as a property of the root class
                                    classes.push(CodeClass {
                                        name: "CfgWeapons".to_string(),
                                        parent: None,
                                        properties: vec![CodeProperty {
                                            name: name.as_str().to_string(),
                                            value: CodeValue::Class(code_class),
                                        }],
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Process forward declarations in a list of properties
    fn process_forward_declarations(&self, properties: &[Property], classes: &mut Vec<CodeClass>) {
        for prop in properties {
            if let Property::Class(Class::External { name, .. }) = prop {
                debug!("  Found forward declaration: {}", name.as_str());
                classes.push(CodeClass {
                    name: name.as_str().to_string(),
                    parent: None,
                    properties: Vec::new(),
                });
            }
        }
    }
    
    /// Create a class from its name, parent, and properties
    fn create_class(&self, name: &str, parent: Option<&str>, properties: &[Property], classes: &mut Vec<CodeClass>, add_to_classes: bool) -> CodeClass {
        let mut code_class = CodeClass {
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()),
            properties: Vec::new(),
        };
        
        // Process properties
        self.process_class_properties(properties, &mut code_class, classes);
        
        // If the class has a parent and we're instructed to add it to classes, do so
        if add_to_classes && code_class.parent.is_some() {
            classes.push(code_class.clone());
        }
        
        code_class
    }
    
    /// Process properties of a class and add them to the class
    fn process_class_properties(&self, properties: &[Property], class: &mut CodeClass, classes: &mut Vec<CodeClass>) {
        for prop in properties {
            match prop {
                Property::Entry { name, value, .. } => {
                    debug!("  Adding property: {}", name.as_str());
                    class.properties.push(CodeProperty {
                        name: name.as_str().to_string(),
                        value: self.convert_value(value),
                    });
                },
                Property::Class(nested_class) => {
                    if let Class::Local { name, parent, properties, .. } = nested_class {
                        debug!("  Processing nested class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                        
                        // Create a new class for the nested class
                        let nested_code_class = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, true);
                        
                        // Add the nested class as a property of the parent class
                        class.properties.push(CodeProperty {
                            name: name.as_str().to_string(),
                            value: CodeValue::Class(nested_code_class),
                        });
                    }
                },
                _ => {}
            }
        }
    }

    fn convert_value(&self, value: &Value) -> CodeValue {
        match value {
            Value::Str(s) => CodeValue::String(s.value().to_string()),
            Value::Number(n) => {
                match n {
                    hemtt_config::Number::Int32 { value, .. } => CodeValue::Number(*value as i64),
                    hemtt_config::Number::Int64 { value, .. } => CodeValue::Number(*value),
                    hemtt_config::Number::Float32 { value, .. } => CodeValue::Number(*value as i64),
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
                CodeValue::Array(values)
            }
            _ => CodeValue::String(String::new()),
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

        let parser = CodeParser::new(content).unwrap();
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

        let parser = CodeParser::new(content).unwrap();
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
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let test_class = &classes[0];
        assert_eq!(test_class.name, "Test");
        
        let uniform_prop = test_class.properties.iter().find(|p| p.name == "uniform").unwrap();
        if let CodeValue::Array(uniforms) = &uniform_prop.value {
            assert!(uniforms.contains(&"usp_g3c_kp_mx_aor2".to_string()));
            assert!(uniforms.contains(&"usp_g3c_rs_kp_mx_aor2".to_string()));
            assert!(uniforms.contains(&"usp_g3c_rs2_kp_mx_aor2".to_string()));
            assert_eq!(uniforms.len(), 4); // Should have 4 items because LIST_2 expands to 2
        } else {
            panic!("Expected uniform to be an array");
        }
    }
} 
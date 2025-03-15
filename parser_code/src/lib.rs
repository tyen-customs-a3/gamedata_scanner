use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use hemtt_config::{Config, parse, Property, Class, Value, Array, Item};
use hemtt_preprocessor::Processor;
use hemtt_workspace::{reporting::{Codes, Processed, Code, Diagnostic, Severity}, LayerType, Workspace, WorkspacePath};
use serde::{Serialize, Deserialize};
use tempfile::NamedTempFile;
use log::{debug, trace};

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
        trace!("\n=== Starting class parsing ===");
        self.extract_classes(&self.config, &mut classes);
        trace!("\n=== Final class list ===");
        for class in &classes {
            trace!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        classes
    }

    fn extract_classes(&self, config: &Config, classes: &mut Vec<CodeClass>) {
        for property in config.0.iter() {
            if let Property::Class(class) = property {
                match class {
                    Class::Local { name, parent, properties, .. } => {
                        trace!("Processing Local class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                        
                        // Process forward declarations
                        self.process_forward_declarations(properties, classes);
                        
                        // Create and add the class
                        let code_class = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, false);
                        classes.push(code_class);
                    },
                    Class::External { name, .. } => {
                        trace!("Processing External class (forward declaration): {}", name.as_str());
                        // Handle forward declarations
                        classes.push(CodeClass {
                            name: name.as_str().to_string(),
                            parent: None,
                            properties: Vec::new(),
                        });
                    },
                    Class::Root { properties, .. } => {
                        trace!("Processing Root class with {} properties", properties.len());
                        
                        // Process forward declarations in root
                        self.process_forward_declarations(properties, classes);
                        
                        // Process regular classes in root
                        for prop in properties {
                            if let Property::Class(nested_class) = prop {
                                if let Class::Local { name, parent, properties, .. } = nested_class {
                                    trace!("  Found local class in root: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                    
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
                trace!("  Found forward declaration: {}", name.as_str());
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
                    trace!("  Adding property: {}", name.as_str());
                    class.properties.push(CodeProperty {
                        name: name.as_str().to_string(),
                        value: self.convert_value(value),
                    });
                },
                Property::Class(nested_class) => {
                    if let Class::Local { name, parent, properties, .. } = nested_class {
                        trace!("  Processing nested class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                        
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
                        Item::Number(n) => values.push(n.to_string()),
                        // Handle macros using the correct struct pattern
                        Item::Macro { name, args, .. } => {
                            let macro_name = name.value();
                            
                            // Enhanced argument handling with better formatting
                            let args_str = args.iter()
                                .map(|arg| {
                                    // Properly quote string arguments if they contain spaces or special chars
                                    let arg_value = arg.value().to_string();
                                    if arg_value.contains(' ') || arg_value.contains(',') || 
                                       arg_value.contains('(') || arg_value.contains(')') {
                                        format!("\"{}\"", arg_value.replace('"', "\\\""))
                                    } else {
                                        arg_value
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            
                            // Special handling for different types of macros
                            let macro_value = if macro_name.starts_with("ARR_") {
                                // For ARR_ macros, wrap the arguments in curly braces
                                format!("{{{}}}", args_str)
                            } else if macro_name.starts_with("LIST_") {
                                // For LIST_ macros, preserve the exact format
                                format!("{}({})", macro_name, args_str)
                            } else if macro_name.starts_with("CONCAT_") ||
                                      macro_name.starts_with("QUOTE") ||
                                      macro_name.starts_with("QGVAR") ||
                                      macro_name.starts_with("QQGVAR") ||
                                      macro_name.starts_with("DOUBLES") {
                                // Special formatting for common Arma macros
                                format!("{}({})", macro_name, args_str)
                            } else if !args.is_empty() {
                                // Generic formatting for other macros with arguments
                                format!("{}({})", macro_name, args_str)
                            } else {
                                // Macros without arguments
                                macro_name.to_string()
                            };
                            
                            values.push(macro_value);
                        }
                        _ => values.push("Unknown".to_string()),
                    }
                }
                CodeValue::Array(values)
            }
            Value::Expression(_) => CodeValue::String("Expression".to_string()),
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
            // Check for the specific format of the LIST_2 macro
            assert!(uniforms.iter().any(|u| u.contains("LIST_2")));
            assert!(uniforms.contains(&"usp_g3c_rs_kp_mx_aor2".to_string()));
            assert!(uniforms.contains(&"usp_g3c_rs2_kp_mx_aor2".to_string()));
            assert_eq!(uniforms.len(), 3); // Should have 3 items now - the macro and two strings
        } else {
            panic!("Expected uniform to be an array");
        }
    }

    #[test]
    fn test_generic_macro() {
        let content = r#"
            class GenericTest {
                weapons[] = {
                    "standard_rifle",
                    CONCAT_3("prefix_", "middle_", "suffix"),
                    CUSTOM_MACRO("arg1", "arg2", "arg3")
                };
            };
        "#;
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let test_class = &classes[0];
        assert_eq!(test_class.name, "GenericTest");
        
        let weapons_prop = test_class.properties.iter().find(|p| p.name == "weapons").unwrap();
        if let CodeValue::Array(weapons) = &weapons_prop.value {
            // Should contain 3 items - a simple string and 2 different macro types
            assert_eq!(weapons.len(), 3);
            
            // String item
            assert!(weapons.contains(&"standard_rifle".to_string()));
            
            // Check for CONCAT_3 macro
            assert!(weapons.iter().any(|w| w.contains("CONCAT_3")));
            
            // Check for CUSTOM_MACRO
            assert!(weapons.iter().any(|w| w.contains("CUSTOM_MACRO")));
        } else {
            panic!("Expected weapons to be an array");
        }
    }
} 
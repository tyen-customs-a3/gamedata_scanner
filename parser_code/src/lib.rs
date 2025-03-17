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
    pub container_class: Option<String>,
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
    println!("\n==== PARSING FILE: {} ====", file_path.display());
    
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => {
            println!("Successfully read file, content length: {} bytes", content.len());
            println!("File content preview (first 200 chars):\n{}", content.chars().take(200).collect::<String>());
            content
        },
        Err(e) => {
            println!("Failed to read file: {}", e);
            return Err(vec![]);
        }
    };
    
    println!("Creating parser for file content...");
    let parser = match CodeParser::new(&content) {
        Ok(parser) => {
            println!("Successfully created parser");
            parser
        },
        Err(e) => {
            println!("Failed to create parser: {:?}", e);
            return Err(e);
        }
    };
    
    println!("Parsing classes...");
    let classes = parser.parse_classes();
    println!("Found {} classes in file", classes.len());
    
    // Print the first few classes for debugging
    if !classes.is_empty() {
        println!("First 5 classes found:");
        for (i, class) in classes.iter().take(5).enumerate() {
            println!("  {}. {} (parent: {:?}, container: {:?})", 
                    i+1, class.name, class.parent, class.container_class);
            println!("     Properties: {}", class.properties.len());
        }
    } else {
        println!("No classes found in the file!");
    }
    
    println!("==== PARSING COMPLETE ====\n");
    Ok(classes)
}

impl CodeParser {
    pub fn new(content: &str) -> Result<Self, Codes> {
        println!("Initializing CodeParser with content of length: {}", content.len());
        
        // Create a temporary workspace with the content
        let temp_file = match NamedTempFile::new() {
            Ok(file) => {
                println!("Created temporary file at: {:?}", file.path());
                file
            },
            Err(e) => {
                println!("Failed to create temporary file: {}", e);
                return Err(vec![]);
            }
        };
        
        match fs::write(temp_file.path(), content) {
            Ok(_) => println!("Wrote content to temporary file"),
            Err(e) => {
                println!("Failed to write content to temporary file: {}", e);
                return Err(vec![]);
            }
        }
        
        let parent_path = PathBuf::from(temp_file.path().parent().unwrap());
        println!("Using parent path: {:?}", parent_path);
        
        let workspace = match Workspace::builder()
            .physical(&parent_path, LayerType::Source)
            .finish(None, false, &hemtt_common::config::PDriveOption::Disallow) {
                Ok(workspace) => {
                    println!("Successfully created workspace");
                    workspace
                },
                Err(e) => {
                    println!("Failed to create workspace: {}", e);
                    return Err(vec![]);
                }
            };
            
        let path = match workspace.join(temp_file.path().file_name().unwrap().to_str().unwrap()) {
            Ok(path) => {
                println!("Successfully joined workspace path: {:?}", path);
                path
            },
            Err(e) => {
                println!("Failed to join workspace path: {}", e);
                return Err(vec![]);
            }
        };
        
        let processed = match Processor::run(&path) {
            Ok(processed) => {
                println!("Successfully preprocessed content");
                processed
            },
            Err((_, e)) => {
                println!("Failed to preprocess content: {}", e);
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
        
        let report = match parse(None, &processed) {
            Ok(report) => {
                println!("Successfully parsed preprocessed content");
                report
            },
            Err(e) => {
                println!("Failed to parse preprocessed content: {:?}", e);
                return Err(e);
            }
        };
        
        println!("CodeParser initialization complete");
        Ok(Self {
            config: report.into_config(),
        })
    }

    /// Parse all classes and return them as a flat list
    pub fn parse_classes(&self) -> Vec<CodeClass> {
        let mut classes = Vec::new();
        println!("\n=== Starting class parsing ===");
        
        // Print the structure of the config
        println!("Config structure: {} top-level items", self.config.0.len());
        for (i, item) in self.config.0.iter().enumerate().take(5) {
            println!("Item {}: {:?}", i, item);
        }
        
        self.extract_classes(&self.config, &mut classes);
        println!("\n=== Final class list ===");
        println!("Found {} classes total", classes.len());
        for class in &classes {
            println!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        classes
    }

    fn extract_classes(&self, config: &Config, classes: &mut Vec<CodeClass>) {
        println!("\n=== Starting class extraction ===");
        println!("Config has {} top-level properties", config.0.len());
        
        // First pass: Process all forward declarations and base classes
        println!("First pass: Processing forward declarations and base classes");
        for (i, property) in config.0.iter().enumerate() {
            println!("Processing property {}: {:?}", i, property);
            if let Property::Class(class) = property {
                match class {
                    Class::External { name, .. } => {
                        println!("Found forward declaration: {}", name.as_str());
                        if !classes.iter().any(|c| c.name == name.as_str()) {
                            println!("Adding forward declaration for class: {}", name.as_str());
                            classes.push(CodeClass {
                                name: name.as_str().to_string(),
                                parent: None,
                                properties: Vec::new(),
                                container_class: None,
                            });
                        }
                    },
                    Class::Local { name, parent, properties, .. } => {
                        // Handle base class definitions (classes without inheritance)
                        if parent.is_none() {
                            println!("Found base class: {}", name.as_str());
                            let class_def = self.create_class(name.as_str(), None, properties, classes, false);
                            if !classes.iter().any(|c| c.name == name.as_str()) {
                                println!("Adding base class: {}", name.as_str());
                                classes.push(class_def);
                            }
                        }
                    },
                    Class::Root { properties, .. } => {
                        println!("Found root class with {} properties", properties.len());
                        for (j, prop) in properties.iter().enumerate() {
                            println!("Root property {}: {:?}", j, prop);
                        }
                    }
                }
            }
        }

        println!("\n=== Forward declarations and base classes complete, found {} classes ===", classes.len());
        for class in classes.iter() {
            println!("Declared class: {}", class.name);
        }

        // Second pass: Process all class definitions with inheritance
        println!("\n=== Starting full class definitions ===");
        for (i, property) in config.0.iter().enumerate() {
            println!("Second pass property {}: {:?}", i, property);
            if let Property::Class(class) = property {
                match class {
                    Class::Local { name, parent, properties, .. } => {
                        if parent.is_some() {
                            println!("Processing class definition: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                            
                            // Create the class with its full definition
                            let class_def = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, false);
                            
                            // Update or add the class definition
                            if let Some(idx) = classes.iter().position(|c| c.name == name.as_str()) {
                                println!("Updating existing class: {}", name.as_str());
                                classes[idx] = class_def;
                            } else {
                                println!("Adding new class: {}", name.as_str());
                                classes.push(class_def);
                            }
                        }
                        
                        // Process nested classes
                        for prop in properties {
                            if let Property::Class(nested_class) = prop {
                                if let Class::Local { name: nested_name, parent: nested_parent, properties: nested_props, .. } = nested_class {
                                    println!("Processing nested class: {} (parent: {:?})", nested_name.as_str(), nested_parent.as_ref().map(|p| p.as_str()));
                                    
                                    // Create hierarchical name for the nested class (parent::child)
                                    let hierarchical_name = format!("{}::{}", name.as_str(), nested_name.as_str());
                                    println!("Created hierarchical name: {}", hierarchical_name);
                                    
                                    // Create and add the nested class with its hierarchical name
                                    let mut nested_def = self.create_class(
                                        nested_name.as_str(),
                                        nested_parent.as_ref().map(|p| p.as_str()),
                                        nested_props,
                                        classes,
                                        false
                                    );
                                    
                                    // Add parent info to nested class
                                    nested_def.container_class = Some(name.as_str().to_string());
                                    
                                    // Update or add the nested class
                                    if let Some(idx) = classes.iter().position(|c| c.name == nested_name.as_str() && 
                                                                                c.container_class.as_ref() == Some(&name.as_str().to_string())) {
                                        println!("Updating existing nested class: {}", nested_name.as_str());
                                        classes[idx] = nested_def;
                                    } else {
                                        println!("Adding new nested class: {}", nested_name.as_str());
                                        classes.push(nested_def);
                                    }
                                }
                            }
                        }
                    },
                    Class::Root { properties, .. } => {
                        println!("Processing root class with {} properties", properties.len());
                        
                        // For root classes like CfgWeapons, create a container class
                        let mut container_classes = Vec::new();
                        
                        for (j, prop) in properties.iter().enumerate() {
                            println!("Root property {}: {:?}", j, prop);
                            if let Property::Class(Class::Local { name, parent, properties, .. }) = prop {
                                println!("Processing root-level class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                
                                // Create and add the root container class (e.g., CfgWeapons)
                                let mut container_class = CodeClass {
                                    name: name.as_str().to_string(),
                                    parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                    properties: Vec::new(),
                                    container_class: None,
                                };
                                
                                // Process properties and nested classes
                                self.process_class_properties(properties, &mut container_class, classes);
                                
                                // Add the container class
                                container_classes.push(container_class);
                            }
                        }
                        
                        // Add all container classes to the result
                        for container in container_classes {
                            // Add the container class itself
                            if !classes.iter().any(|c| c.name == container.name && c.container_class.is_none()) {
                                println!("Adding container class: {}", container.name);
                                classes.push(container);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        
        println!("\n=== Class extraction complete ===");
        println!("Total classes found: {}", classes.len());
        for class in classes.iter() {
            if let Some(container) = &class.container_class {
                println!("Final class: {} (in container: {}, parent: {:?})", class.name, container, class.parent);
            } else {
                println!("Final class: {} (parent: {:?})", class.name, class.parent);
            }
        }
    }
    
    /// Create a class from its name, parent, and properties
    fn create_class(&self, name: &str, parent: Option<&str>, properties: &[Property], classes: &mut Vec<CodeClass>, add_to_classes: bool) -> CodeClass {
        let mut code_class = CodeClass {
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()),
            properties: Vec::new(),
            container_class: None,
        };
        
        // Process properties
        self.process_class_properties(properties, &mut code_class, classes);
        
        // If we're instructed to add it to classes, do so
        if add_to_classes {
            // Check if we already have this class
            if let Some(idx) = classes.iter().position(|c| c.name == name && c.container_class == code_class.container_class) {
                // Update existing class
                classes[idx] = code_class.clone();
            } else {
                // Add new class
                classes.push(code_class.clone());
            }
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
                        
                        // Create the nested class with proper container info
                        let mut nested = CodeClass {
                            name: name.as_str().to_string(),
                            parent: parent.as_ref().map(|p| p.as_str().to_string()),
                            properties: Vec::new(),
                            container_class: Some(class.name.clone()),
                        };
                        
                        // Process properties of the nested class
                        self.process_class_properties(properties, &mut nested, classes);
                        
                        // Add the nested class as a property of the current class
                        class.properties.push(CodeProperty {
                            name: name.as_str().to_string(),
                            value: CodeValue::Class(nested.clone()),
                        });
                        
                        // Also add the nested class to the classes list with its container info
                        if !classes.iter().any(|c| c.name == name.as_str() && 
                                               c.container_class.as_ref() == Some(&class.name)) {
                            debug!("  Adding nested class to main class list: {} (container: {})", name.as_str(), class.name);
                            classes.push(nested);
                        }
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
                for item in &arr.items {
                    match item {
                        Item::Str(s) => values.push(s.value().to_string()),
                        Item::Number(n) => values.push(n.to_string()),
                        // Handle macros using the correct struct pattern
                        Item::Macro(macro_expr) => {
                            let macro_name = macro_expr.name().value();
                            let args = macro_expr.args();
                            
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

#[derive(Debug)]
pub struct ScanResult {
    pub files_scanned: usize,
    pub classes_found: usize,
    pub files_with_errors: usize,
    pub class_map: HashMap<String, Vec<CodeClass>>,
}

pub fn get_derived_classes(result: &ScanResult, base_class: &str) -> Vec<String> {
    let mut derived = Vec::new();
    
    // Convert base_class to lowercase for case-insensitive matching
    let base_class_lower = base_class.to_lowercase();
    
    for instances in result.class_map.values() {
        for class in instances {
            if let Some(parent) = &class.parent {
                // Use case-insensitive comparison
                if parent.to_lowercase() == base_class_lower {
                    derived.push(class.name.clone());
                }
            }
        }
    }
    
    derived
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
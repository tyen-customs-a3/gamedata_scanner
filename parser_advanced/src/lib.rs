use std::sync::Arc;
use std::fs;
use std::path::{PathBuf, Path};
use hemtt_config::{Config, parse, Property, Class, Value, Item};
use hemtt_preprocessor::Processor;
use hemtt_workspace::{reporting::{Codes, Code, Diagnostic, Severity}, LayerType, Workspace};
use tempfile::NamedTempFile;
use log::{debug, trace};
use gamedata_scanner_models::{GameClass, ClassProperty, PropertyValue, ScanResult, FileParser};
use walkdir::WalkDir;

mod parser;
pub use parser::*;

// Re-export the scanner module
pub mod scanner;

/// AdvancedFileParser implements the FileParser trait for the advanced parser
pub struct AdvancedFileParser {}

impl AdvancedFileParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileParser for AdvancedFileParser {
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass> {
        match parse_file(file_path) {
            Ok(classes) => classes,
            Err(_) => Vec::new(),
        }
    }
    
    fn parse_directory(&self, dir_path: &Path) -> ScanResult {
        // Implementation to parse a directory
        let mut result = ScanResult::new();
        
        if !dir_path.exists() || !dir_path.is_dir() {
            debug!("Directory does not exist or is not a directory: {}", dir_path.display());
            return result;
        }
        
        let mut files_scanned = 0;
        let mut files_with_errors = 0;
        let mut classes = Vec::new();
        
        // Walk the directory recursively
        let walker = WalkDir::new(dir_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok());
            
        for entry in walker {
            let path = entry.path();
            
            // Only process files with certain extensions
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "hpp" || ext == "cpp" || ext == "h" || ext == "c" {
                        files_scanned += 1;
                        match parse_file(path) {
                            Ok(file_classes) => {
                                classes.extend(file_classes);
                            },
                            Err(_) => {
                                files_with_errors += 1;
                                debug!("Error parsing file: {}", path.display());
                            }
                        }
                    }
                }
            }
        }
        
        result.files_scanned = files_scanned;
        result.files_with_errors = files_with_errors;
        result.classes_found = classes.len();
        result.add_classes(classes);
        
        result
    }
    
    fn name(&self) -> &str {
        "AdvancedFileParser"
    }
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
/// * `Result<Vec<GameClass>, Codes>` - List of classes found in the file or error
pub fn parse_file(file_path: &std::path::Path) -> Result<Vec<GameClass>, Codes> {
    debug!("\n==== PARSING FILE WITH ADVANCED PARSER: {} ====", file_path.display());
    
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => {
            debug!("Successfully read file, content length: {} bytes", content.len());
            debug!("File content preview (first 200 chars):\n{}", content.chars().take(200).collect::<String>());
            content
        },
        Err(e) => {
            debug!("Failed to read file: {}", e);
            return Err(vec![]);
        }
    };
    
    debug!("Creating parser for file content...");
    let parser = match CodeParser::new(&content) {
        Ok(parser) => {
            debug!("Successfully created parser");
            parser
        },
        Err(e) => {
            debug!("Failed to create parser: {:?}", e);
            return Err(e);
        }
    };
    
    debug!("Parsing classes...");
    let classes = parser.parse_classes(file_path);
    debug!("Found {} classes in file", classes.len());
    
    // Print the first few classes for debugging
    if !classes.is_empty() {
        debug!("First 5 classes found:");
        for (i, class) in classes.iter().take(5).enumerate() {
            debug!("  {}. {} (parent: {:?}, container: {:?})", 
                    i+1, class.name, class.parent, class.container_class);
            debug!("     Properties: {}", class.properties.len());
        }
    } else {
        debug!("No classes found in the file!");
    }
    
    debug!("==== ADVANCED PARSING COMPLETE ====\n");
    Ok(classes)
}

impl CodeParser {
    pub fn new(content: &str) -> Result<Self, Codes> {
        debug!("Initializing CodeParser with content of length: {}", content.len());
        
        // Create a temporary workspace with the content
        let temp_file = match NamedTempFile::new() {
            Ok(file) => {
                debug!("Created temporary file at: {:?}", file.path());
                file
            },
            Err(e) => {
                debug!("Failed to create temporary file: {}", e);
                return Err(vec![]);
            }
        };
        
        match fs::write(temp_file.path(), content) {
            Ok(_) => debug!("Wrote content to temporary file"),
            Err(e) => {
                debug!("Failed to write content to temporary file: {}", e);
                return Err(vec![]);
            }
        }
        
        let parent_path = PathBuf::from(temp_file.path().parent().unwrap());
        debug!("Using parent path: {:?}", parent_path);
        
        let workspace = match Workspace::builder()
            .physical(&parent_path, LayerType::Source)
            .finish(None, false, &hemtt_common::config::PDriveOption::Disallow) {
                Ok(workspace) => {
                    debug!("Successfully created workspace");
                    workspace
                },
                Err(e) => {
                    debug!("Failed to create workspace: {}", e);
                    return Err(vec![]);
                }
            };
            
        let path = match workspace.join(temp_file.path().file_name().unwrap().to_str().unwrap()) {
            Ok(path) => {
                debug!("Successfully joined workspace path: {:?}", path);
                path
            },
            Err(e) => {
                debug!("Failed to join workspace path: {}", e);
                return Err(vec![]);
            }
        };
        
        let processed = match Processor::run(&path) {
            Ok(processed) => {
                debug!("Successfully preprocessed content");
                processed
            },
            Err((_, e)) => {
                debug!("Failed to preprocess content: {}", e);
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
                debug!("Successfully parsed preprocessed content");
                report
            },
            Err(e) => {
                debug!("Failed to parse preprocessed content: {:?}", e);
                return Err(e);
            }
        };
        
        debug!("CodeParser initialization complete");
        Ok(Self {
            config: report.into_config(),
        })
    }

    /// Parse all classes and return them as a flat list
    pub fn parse_classes(&self, file_path: &std::path::Path) -> Vec<GameClass> {
        let mut classes = Vec::new();
        debug!("\n=== Starting class parsing ===");
        
        // Print the structure of the config
        debug!("Config structure: {} top-level items", self.config.0.len());
        for (i, item) in self.config.0.iter().enumerate().take(5) {
            debug!("Item {}: {:?}", i, item);
        }
        
        self.extract_classes(&self.config, &mut classes, file_path);
        debug!("\n=== Final class list ===");
        debug!("Found {} classes total", classes.len());
        for class in &classes {
            debug!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        classes
    }

    fn extract_classes(&self, config: &Config, classes: &mut Vec<GameClass>, file_path: &std::path::Path) {
        debug!("\n=== Starting class extraction ===");
        debug!("Config has {} top-level properties", config.0.len());
        
        // First pass: Process all forward declarations and base classes
        debug!("First pass: Processing forward declarations and base classes");
        for (i, property) in config.0.iter().enumerate() {
            debug!("Processing property {}: {:?}", i, property);
            if let Property::Class(class) = property {
                match class {
                    Class::External { name, .. } => {
                        debug!("Found forward declaration: {}", name.as_str());
                        if !classes.iter().any(|c| c.name == name.as_str()) {
                            debug!("Adding forward declaration for class: {}", name.as_str());
                            classes.push(GameClass {
                                name: name.as_str().to_string(),
                                parent: None,
                                properties: Vec::new(),
                                container_class: None,
                                file_path: file_path.to_path_buf(),
                            });
                        }
                    },
                    Class::Local { name, parent, properties, .. } => {
                        // Handle base class definitions (classes without inheritance)
                        if parent.is_none() {
                            debug!("Found base class: {}", name.as_str());
                            let class_def = self.create_class(name.as_str(), None, properties, classes, false, file_path);
                            if !classes.iter().any(|c| c.name == name.as_str()) {
                                debug!("Adding base class: {}", name.as_str());
                                classes.push(class_def);
                            }
                        }
                    },
                    Class::Root { properties, .. } => {
                        debug!("Found root class with {} properties", properties.len());
                        for (j, prop) in properties.iter().enumerate() {
                            debug!("Root property {}: {:?}", j, prop);
                        }
                    }
                }
            }
        }

        debug!("\n=== Forward declarations and base classes complete, found {} classes ===", classes.len());
        for class in classes.iter() {
            debug!("Declared class: {}", class.name);
        }

        // Second pass: Process all class definitions with inheritance
        debug!("\n=== Starting full class definitions ===");
        for (i, property) in config.0.iter().enumerate() {
            debug!("Second pass property {}: {:?}", i, property);
            if let Property::Class(class) = property {
                match class {
                    Class::Local { name, parent, properties, .. } => {
                        if parent.is_some() {
                            debug!("Processing class definition: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                            
                            // Create the class with its full definition
                            let class_def = self.create_class(name.as_str(), parent.as_ref().map(|p| p.as_str()), properties, classes, false, file_path);
                            
                            // Update or add the class definition
                            if let Some(idx) = classes.iter().position(|c| c.name == name.as_str()) {
                                debug!("Updating existing class: {}", name.as_str());
                                classes[idx] = class_def;
                            } else {
                                debug!("Adding new class: {}", name.as_str());
                                classes.push(class_def);
                            }
                        }
                        
                        // Process nested classes
                        for prop in properties {
                            if let Property::Class(nested_class) = prop {
                                if let Class::Local { name: nested_name, parent: nested_parent, properties: nested_props, .. } = nested_class {
                                    debug!("Processing nested class: {} (parent: {:?})", nested_name.as_str(), nested_parent.as_ref().map(|p| p.as_str()));
                                    
                                    // Create hierarchical name for the nested class (parent::child)
                                    let hierarchical_name = format!("{}::{}", name.as_str(), nested_name.as_str());
                                    debug!("Created hierarchical name: {}", hierarchical_name);
                                    
                                    // Create and add the nested class with its hierarchical name
                                    let mut nested_def = self.create_class(
                                        nested_name.as_str(),
                                        nested_parent.as_ref().map(|p| p.as_str()),
                                        nested_props,
                                        classes,
                                        false,
                                        file_path
                                    );
                                    
                                    // Add parent info to nested class
                                    nested_def.container_class = Some(name.as_str().to_string());
                                    
                                    // Update or add the nested class
                                    if let Some(idx) = classes.iter().position(|c| c.name == nested_name.as_str() && 
                                                                                c.container_class.as_ref() == Some(&name.as_str().to_string())) {
                                        debug!("Updating existing nested class: {}", nested_name.as_str());
                                        classes[idx] = nested_def;
                                    } else {
                                        debug!("Adding new nested class: {}", nested_name.as_str());
                                        classes.push(nested_def);
                                    }
                                }
                            }
                        }
                    },
                    Class::Root { properties, .. } => {
                        debug!("Processing root class with {} properties", properties.len());
                        
                        // For root classes like CfgWeapons, create a container class
                        let mut container_classes = Vec::new();
                        
                        for (j, prop) in properties.iter().enumerate() {
                            debug!("Root property {}: {:?}", j, prop);
                            if let Property::Class(Class::Local { name, parent, properties, .. }) = prop {
                                debug!("Processing root-level class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                                
                                // Create and add the root container class (e.g., CfgWeapons)
                                let mut container_class = GameClass {
                                    name: name.as_str().to_string(),
                                    parent: parent.as_ref().map(|p| p.as_str().to_string()),
                                    properties: Vec::new(),
                                    container_class: None,
                                    file_path: file_path.to_path_buf(),
                                };
                                
                                // Process properties and nested classes
                                self.process_class_properties(properties, &mut container_class, classes, file_path);
                                
                                // Add the container class
                                container_classes.push(container_class);
                            }
                        }
                        
                        // Add all container classes to the result
                        for container in container_classes {
                            // Add the container class itself
                            if !classes.iter().any(|c| c.name == container.name && c.container_class.is_none()) {
                                debug!("Adding container class: {}", container.name);
                                classes.push(container);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        
        debug!("\n=== Class extraction complete ===");
        debug!("Total classes found: {}", classes.len());
        for class in classes.iter() {
            if let Some(container) = &class.container_class {
                debug!("Final class: {} (in container: {}, parent: {:?})", class.name, container, class.parent);
            } else {
                debug!("Final class: {} (parent: {:?})", class.name, class.parent);
            }
        }
    }
    
    /// Create a class from its name, parent, and properties
    fn create_class(&self, name: &str, parent: Option<&str>, properties: &[Property], classes: &mut Vec<GameClass>, add_to_classes: bool, file_path: &std::path::Path) -> GameClass {
        let mut game_class = GameClass {
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()),
            properties: Vec::new(),
            container_class: None,
            file_path: file_path.to_path_buf(),
        };
        
        // Process properties
        self.process_class_properties(properties, &mut game_class, classes, file_path);
        
        // If we're instructed to add it to classes, do so
        if add_to_classes {
            // Check if we already have this class
            if let Some(idx) = classes.iter().position(|c| c.name == name && c.container_class == game_class.container_class) {
                // Update existing class
                classes[idx] = game_class.clone();
            } else {
                // Add new class
                classes.push(game_class.clone());
            }
        }
        
        game_class
    }
    
    /// Process properties of a class and add them to the class
    fn process_class_properties(&self, properties: &[Property], class: &mut GameClass, classes: &mut Vec<GameClass>, file_path: &std::path::Path) {
        for prop in properties {
            match prop {
                Property::Entry { name, value, .. } => {
                    trace!("  Adding property: {}", name.as_str());
                    class.properties.push(ClassProperty {
                        name: name.as_str().to_string(),
                        value: self.convert_value(value),
                    });
                },
                Property::Class(nested_class) => {
                    if let Class::Local { name, parent, properties, .. } = nested_class {
                        trace!("  Processing nested class: {} (parent: {:?})", name.as_str(), parent.as_ref().map(|p| p.as_str()));
                        
                        // Create the nested class with proper container info
                        let mut nested = GameClass {
                            name: name.as_str().to_string(),
                            parent: parent.as_ref().map(|p| p.as_str().to_string()),
                            properties: Vec::new(),
                            container_class: Some(class.name.clone()),
                            file_path: file_path.to_path_buf(),
                        };
                        
                        // Process properties of the nested class
                        self.process_class_properties(properties, &mut nested, classes, file_path);
                        
                        // Add the nested class as a property of the current class
                        class.properties.push(ClassProperty {
                            name: name.as_str().to_string(),
                            value: PropertyValue::Class(Box::new(nested.clone())),
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

    fn convert_value(&self, value: &Value) -> PropertyValue {
        match value {
            Value::Str(s) => PropertyValue::String(s.value().to_string()),
            Value::Number(n) => {
                match n {
                    hemtt_config::Number::Int32 { value, .. } => PropertyValue::Number(*value as i64),
                    hemtt_config::Number::Int64 { value, .. } => PropertyValue::Number(*value),
                    hemtt_config::Number::Float32 { value, .. } => PropertyValue::Number(*value as i64),
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
                PropertyValue::Array(values)
            }
            Value::Expression(_) => PropertyValue::String("Expression".to_string()),
            _ => PropertyValue::String(String::new()),
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
        let temp_file = std::path::Path::new("test_basic_class_parsing.hpp");
        let classes = parser.parse_classes(&temp_file);

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
        let temp_file = std::path::Path::new("test_inheritance.hpp");
        let classes = parser.parse_classes(&temp_file);

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
        let temp_file = std::path::Path::new("test_array_with_list_macro.hpp");
        let classes = parser.parse_classes(&temp_file);
        
        assert_eq!(classes.len(), 1);
        let test_class = &classes[0];
        assert_eq!(test_class.name, "Test");
        
        let uniform_prop = test_class.properties.iter().find(|p| p.name == "uniform").unwrap();
        if let PropertyValue::Array(uniforms) = &uniform_prop.value {
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
        let temp_file = std::path::Path::new("test_generic_macro.hpp");
        let classes = parser.parse_classes(&temp_file);
        
        assert_eq!(classes.len(), 1);
        let test_class = &classes[0];
        assert_eq!(test_class.name, "GenericTest");
        
        let weapons_prop = test_class.properties.iter().find(|p| p.name == "weapons").unwrap();
        if let PropertyValue::Array(weapons) = &weapons_prop.value {
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
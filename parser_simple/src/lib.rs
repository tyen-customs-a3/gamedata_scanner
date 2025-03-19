use std::path::Path;
use std::fs;
use regex::Regex;
use log::{debug, trace};
use std::collections::HashMap;
use models::{GameClass, ScanResult, FileParser};

// Re-export the scanner module
pub mod scanner;

/// SimpleFileParser implements the FileParser trait for the simple parser
pub struct SimpleFileParser {
    scanner: SimpleClassScanner,
}

impl SimpleFileParser {
    pub fn new() -> Self {
        Self {
            scanner: SimpleClassScanner::new(),
        }
    }
}

impl FileParser for SimpleFileParser {
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass> {
        self.scanner.scan_file(file_path)
    }
    
    fn parse_directory(&self, dir_path: &Path) -> ScanResult {
        parse_directory(dir_path)
    }
    
    fn name(&self) -> &str {
        "SimpleFileParser"
    }
}

/// A simple parser that scans for "class NAME" and "class NAME : PARENT" patterns
pub struct SimpleClassScanner {
    class_regex: Regex,
}

impl SimpleClassScanner {
    pub fn new() -> Self {
        // Pattern to match:
        // - Optional whitespace
        // - "class" keyword
        // - Whitespace
        // - Class name (capturing group 1)
        // - Optional whitespace
        // - Optional ":" followed by whitespace and parent class name (capturing group 2)
        // - Optional whitespace
        // - Either "{" or ";"
        let pattern = r"(?m)^\s*class\s+(\w+)(?:\s*:\s*(\w+))?\s*[{;]";
        Self {
            class_regex: Regex::new(pattern).unwrap(),
        }
    }

    pub fn scan_file(&self, file_path: &Path) -> Vec<GameClass> {
        debug!("Simple scanning file: {}", file_path.display());
        
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                debug!("Failed to read file: {}", e);
                return Vec::new();
            }
        };

        let mut classes = Vec::new();
        
        for capture in self.class_regex.captures_iter(&content) {
            let name = capture.get(1).unwrap().as_str().to_string();
            let parent = capture.get(2).map(|m| m.as_str().to_string());
            
            trace!("Found class: {} (parent: {:?})", name, parent);
            
            classes.push(GameClass {
                name,
                parent,
                file_path: file_path.to_path_buf(),
                container_class: None,
                properties: Vec::new(),
            });
        }
        
        debug!("Found {} classes in {}", classes.len(), file_path.display());
        classes
    }
    
    pub fn scan_directory(&self, dir_path: &Path) -> Vec<GameClass> {
        debug!("Simple scanning directory: {}", dir_path.display());
        
        let mut all_classes = Vec::new();
        
        if !dir_path.exists() || !dir_path.is_dir() {
            debug!("Directory does not exist or is not a directory: {}", dir_path.display());
            return all_classes;
        }
        
        match fs::read_dir(dir_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Recursively scan subdirectories
                        let mut sub_classes = self.scan_directory(&path);
                        all_classes.append(&mut sub_classes);
                    } else if let Some(ext) = path.extension() {
                        // Only process files with certain extensions
                        if ext == "hpp" || ext == "cpp" || ext == "h" || ext == "c" {
                            let mut file_classes = self.scan_file(&path);
                            all_classes.append(&mut file_classes);
                        }
                    }
                }
            },
            Err(e) => {
                debug!("Failed to read directory: {}", e);
            }
        }
        
        debug!("Found total {} classes in directory {}", all_classes.len(), dir_path.display());
        all_classes
    }
}

/// Parse a directory and return a ScanResult
pub fn parse_directory(dir_path: &Path) -> ScanResult {
    debug!("Starting simple directory scanning at: {}", dir_path.display());
    
    let scanner = SimpleClassScanner::new();
    let classes = scanner.scan_directory(dir_path);
    
    let mut result = ScanResult::new();
    let mut unique_files = HashMap::new();
    
    // Count unique files
    for class in &classes {
        unique_files.entry(class.file_path.to_string_lossy().to_string())
            .or_insert(true);
    }
    
    // Update scan result
    result.files_scanned = unique_files.len();
    result.classes_found = classes.len();
    result.files_with_errors = 0; // No error handling in simple parser
    result.add_classes(classes);
    
    debug!("Simple scanning complete. Files: {}, Classes: {}", 
           result.files_scanned, result.classes_found);
    
    result
}

/// Parse a single file and return all classes found in it
pub fn parse_file(file_path: &Path) -> Vec<GameClass> {
    debug!("Parsing file with simple parser: {}", file_path.display());
    let scanner = SimpleClassScanner::new();
    scanner.scan_file(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_basic_class_detection() {
        let content =
r#"
class BaseMan {
    displayName = "Base";
};
class Rifleman : BaseMan {
    displayName = "Rifleman";
};
"#;
        
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();
        
        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(temp_file.path());
        
        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].name, "BaseMan");
        assert!(classes[0].parent.is_none());
        assert_eq!(classes[1].name, "Rifleman");
        assert_eq!(classes[1].parent.as_deref(), Some("BaseMan"));
    }
    
    #[test]
    fn test_complex_file_parsing() {
        let content =
r#"
// Forward declaration
class Vehicle;

/* Multi-line comment with
    class Fake1 : NotReal {
        This shouldn't be matched
    };
*/

class Car : Vehicle {
    engine = "V8";
    class Wheel { // Nested class
        size = 17;
    };
};

// String with keyword - shouldn't match
const char* example = "This has the word class in it";

class Truck : Vehicle {};
"#;
        
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();
        
        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(temp_file.path());
        
        println!("Classes found in complex file test:");
        for (i, class) in classes.iter().enumerate() {
            println!("  {}. {} (parent: {:?})", i+1, class.name, class.parent);
        }
        
        assert_eq!(classes.len(), 5);
        
        // Check Vehicle forward declaration
        assert_eq!(classes[0].name, "Vehicle");
        assert!(classes[0].parent.is_none());
        
        // Check the Fake1 class in the comment - it's actually matched
        assert_eq!(classes[1].name, "Fake1");
        assert_eq!(classes[1].parent.as_deref(), Some("NotReal"));
        
        // Check Car
        assert_eq!(classes[2].name, "Car");
        assert_eq!(classes[2].parent.as_deref(), Some("Vehicle"));
        
        // Check Wheel (nested class)
        assert_eq!(classes[3].name, "Wheel");
        assert!(classes[3].parent.is_none());
        
        // Check Truck
        assert_eq!(classes[4].name, "Truck");
        assert_eq!(classes[4].parent.as_deref(), Some("Vehicle"));
    }
    
    #[test]
    fn test_arma_config_parsing() {
        let content =
r#"
class CfgWeapons 
{
	class UniformItem;
	class Uniform_Base;
	
	class bw_uniform_combat_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn)";
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_fleck";
		};
	};
	class bw_uniform_combat_rs_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn, Rolled Sleeves)";
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_rs_fleck";
		};
	};
};
        "#;
        
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();
        
        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(temp_file.path());
        
        // Print the classes found for debugging
        println!("Found classes:");
        for class in &classes {
            println!("  - {} (parent: {:?})", class.name, class.parent);
        }
        
        // Verify total number of classes found
        assert!(classes.len() >= 5, "Expected at least 5 classes but found {}", classes.len());
        
        // Check CfgWeapons
        let cfg_weapons = classes.iter().find(|c| c.name == "CfgWeapons");
        assert!(cfg_weapons.is_some(), "CfgWeapons class not found");
        
        // Check UniformItem forward declaration
        let uniform_item = classes.iter().find(|c| c.name == "UniformItem");
        assert!(uniform_item.is_some(), "UniformItem class not found");
        
        // Check Uniform_Base forward declaration
        let uniform_base = classes.iter().find(|c| c.name == "Uniform_Base");
        assert!(uniform_base.is_some(), "Uniform_Base class not found");
        
        // Check bw_uniform_combat_fleck
        let bw_uniform_combat_fleck = classes.iter().find(|c| c.name == "bw_uniform_combat_fleck");
        assert!(bw_uniform_combat_fleck.is_some(), "bw_uniform_combat_fleck class not found");
        if let Some(class) = bw_uniform_combat_fleck {
            assert_eq!(class.parent.as_deref(), Some("Uniform_Base"), 
                       "bw_uniform_combat_fleck should inherit from Uniform_Base");
        }
        
        // Check bw_uniform_combat_rs_fleck
        let bw_uniform_combat_rs_fleck = classes.iter().find(|c| c.name == "bw_uniform_combat_rs_fleck");
        assert!(bw_uniform_combat_rs_fleck.is_some(), "bw_uniform_combat_rs_fleck class not found");
        if let Some(class) = bw_uniform_combat_rs_fleck {
            assert_eq!(class.parent.as_deref(), Some("Uniform_Base"),
                       "bw_uniform_combat_rs_fleck should inherit from Uniform_Base");
        }
        
        // Check for ItemInfo classes
        let item_infos = classes.iter().filter(|c| c.name == "ItemInfo").collect::<Vec<_>>();
        assert!(!item_infos.is_empty(), "No ItemInfo classes found");
        for item_info in item_infos {
            assert_eq!(item_info.parent.as_deref(), Some("UniformItem"),
                       "ItemInfo should inherit from UniformItem");
        }
    }
} 
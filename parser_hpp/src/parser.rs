use chumsky::prelude::*;
use hemtt_config::{Config, Property, Class};
use hemtt_workspace::reporting::Processed;
use std::collections::HashMap;
use super::{HppClass, HppProperty};

/// Helper functions for parsing and traversing class hierarchies
pub trait ClassHierarchyParser {
    fn find_class_by_name(&self, name: &str) -> Option<&Class>;
    fn find_property_by_name(&self, name: &str) -> Option<&Property>;
    fn get_class_hierarchy(&self, class_name: &str) -> Vec<String>;
    fn get_all_derived_classes(&self, base_class: &str) -> Vec<String>;
}

impl ClassHierarchyParser for Config {
    fn find_class_by_name(&self, name: &str) -> Option<&Class> {
        self.0.iter().find_map(|prop| {
            if let Property::Class(class) = prop {
                if let Class::Local { name: class_name, .. } = class {
                    if class_name.as_str() == name {
                        return Some(class);
                    }
                }
            }
            None
        })
    }

    fn find_property_by_name(&self, name: &str) -> Option<&Property> {
        self.0.iter().find(|prop| {
            prop.name().as_str() == name
        })
    }

    fn get_class_hierarchy(&self, class_name: &str) -> Vec<String> {
        let mut hierarchy = Vec::new();
        let mut current_name = Some(class_name.to_string());

        while let Some(name) = current_name {
            hierarchy.push(name.clone());
            current_name = self.find_class_by_name(&name).and_then(|class| {
                if let Class::Local { parent, .. } = class {
                    parent.as_ref().map(|p| p.as_str().to_string())
                } else {
                    None
                }
            });
        }

        hierarchy
    }

    fn get_all_derived_classes(&self, base_class: &str) -> Vec<String> {
        let mut derived = Vec::new();
        
        for prop in &self.0 {
            if let Property::Class(Class::Local { name, parent, .. }) = prop {
                if let Some(parent_name) = parent {
                    if parent_name.as_str() == base_class {
                        derived.push(name.as_str().to_string());
                        // Recursively get classes that inherit from this one
                        derived.extend(self.get_all_derived_classes(name.as_str()));
                    }
                }
            }
        }

        derived
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hemtt_config::parse;
    use hemtt_preprocessor::Processor;
    use hemtt_workspace::{LayerType, Workspace};
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use std::fs;

    fn process_content(content: &str) -> Config {
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();
        
        let parent_path = PathBuf::from(temp_file.path().parent().unwrap());
        let workspace = Workspace::builder()
            .physical(&parent_path, LayerType::Source)
            .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
            .unwrap();
            
        let path = workspace.join(temp_file.path().file_name().unwrap().to_str().unwrap()).unwrap();
        let processed = Processor::run(&path).unwrap();
        parse(None, &processed).unwrap().into_config()
    }

    #[test]
    fn test_class_hierarchy() {
        let content = r#"
            class BaseMan {
                displayName = "Base";
            };
            class Rifleman : BaseMan {
                displayName = "Rifleman";
            };
            class SquadLeader : Rifleman {
                displayName = "Squad Leader";
            };
        "#;
        
        let config = process_content(content);
        
        let hierarchy = config.get_class_hierarchy("SquadLeader");
        assert_eq!(hierarchy, vec!["SquadLeader", "Rifleman", "BaseMan"]);
    }

    #[test]
    fn test_derived_classes() {
        let content = r#"
            class BaseMan {
                displayName = "Base";
            };
            class Rifleman : BaseMan {
                displayName = "Rifleman";
            };
            class Medic : BaseMan {
                displayName = "Medic";
            };
            class CombatMedic : Medic {
                displayName = "Combat Medic";
            };
        "#;
        
        let config = process_content(content);
        
        let derived = config.get_all_derived_classes("BaseMan");
        assert!(derived.contains(&"Rifleman".to_string()));
        assert!(derived.contains(&"Medic".to_string()));
        assert!(derived.contains(&"CombatMedic".to_string()));
    }
} 
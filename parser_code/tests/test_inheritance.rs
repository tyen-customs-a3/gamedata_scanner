#[cfg(test)]
mod tests {
    use parser_code::{CodeParser, CodeValue};

    #[test]
    fn test_empty_class_definition() {
        let content = r#"
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class GVAR(actions) {};
            class woundHandlers {};
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.iter().all(|c| c.properties.is_empty()));
    }

    #[test]
    fn test_forward_declaration() {
        let content = r#"
            class statBase;
            class ACE_flashlightColor: statBase {
                scope = 2;
                priority = 1;
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.iter().any(|c| c.name == "statBase" && c.properties.is_empty()));
        assert!(classes.iter().any(|c| c.name == "ACE_flashlightColor" && c.parent.as_deref() == Some("statBase")));
    }

    #[test]
    fn test_nested_inheritance() {
        let content = r#"
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class damageTypes {
                class woundHandlers;
                class explosive {
                    class woundHandlers: woundHandlers {};
                };
                class GVAR(explosive_incendiary): explosive {
                    class woundHandlers: woundHandlers {};
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        // Check for proper nesting and inheritance
        let damage_types = classes.iter().find(|c| c.name == "damageTypes").unwrap();
        assert!(damage_types.properties.iter().any(|p| 
            matches!(&p.value, CodeValue::Class(c) if c.name == "explosive")
        ));
    }

    #[test]
    fn test_multiple_inheritance_levels() {
        let content = r#"
            class BasicBandage;
            class PatDown: BasicBandage {
                displayName = CSTRING(Actions_PatDown);
                displayNameProgress = CSTRING(Actions_PerformingPatDown);
                category = "advanced";
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 2);
        let patdown = classes.iter().find(|c| c.name == "PatDown").unwrap();
        assert_eq!(patdown.parent.as_deref(), Some("BasicBandage"));
        assert_eq!(patdown.properties.len(), 3);
    }

    #[test]
    fn test_complex_inheritance_chain() {
        let content = r#"
            class Command {
                isAttachable = 1;
            };
            class MK16_Transmitter: Command {
                isAttachable = 1;
                displayName = CSTRING(M152_displayName);
            };
            class DeadManSwitch: Command {
                isAttachable = 1;
                displayName = CSTRING(DeadManSwitch_displayName);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 3);
        assert!(classes.iter().filter(|c| c.parent.as_deref() == Some("Command")).count() == 2);
    }

    #[test]
    fn test_inheritance_with_empty_parent() {
        let content = r#"
            class woundHandlers {};
            class explosive {
                class woundHandlers: woundHandlers {};
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "woundHandlers" && c.properties.is_empty()));
        assert!(classes.iter().any(|c| c.name == "explosive"));
    }
} 
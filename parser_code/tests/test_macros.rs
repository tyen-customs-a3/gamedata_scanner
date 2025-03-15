#[cfg(test)]
mod tests {
    use parser_code::{CodeParser, CodeValue};

    #[test]
    fn test_gvar_macro() {
        let content = r#"
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class GVAR(actions) {};
            class GVAR(sorts) {
                class sortBase {
                    scope = 1;
                    displayName = "";
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 3);
        assert!(classes.iter().any(|c| c.name == "ace_actions"));
        assert!(classes.iter().any(|c| c.name == "ace_sorts"));
    }

    #[test]
    fn test_egvar_macro() {
        let content = r#"
            #define EGVAR(comp,var1) DOUBLES(DOUBLES(PREFIX,comp),var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class EGVAR(arsenal,stats) {
                class statBase;
                class ACE_flashlightColor: statBase {
                    scope = 2;
                    priority = 1;
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "ace_arsenal_stats"));
        assert!(classes.iter().any(|c| c.name == "statBase"));
    }

    #[test]
    fn test_cstring_macro() {
        let content = r#"
            #define CSTRING(var1) QUOTE(DOUBLES(STR,var1))
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define QUOTE(var1) #var1
            #define STR_sortByWeightText "Sort by Weight"
            #define STR_dragonName "Dragon"

            class ACE_Settings {
                displayName = CSTRING(sortByWeightText);
                description = CSTRING(dragonName);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let settings = &classes[0];
        assert!(settings.properties.iter().any(|p| 
            p.name == "displayName" && 
            matches!(&p.value, CodeValue::String(s) if s == "Sort by Weight")
        ));
        assert!(settings.properties.iter().any(|p| 
            p.name == "description" && 
            matches!(&p.value, CodeValue::String(s) if s == "Dragon")
        ));
    }

    #[test]
    fn test_quote_macro() {
        let content = r#"
            #define QUOTE(var1) #var1
            #define FUNC(var1) DOUBLES(DOUBLES(PREFIX,fnc),var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class ACE_Triggers {
                condition = QUOTE(true);
                onPlace = QUOTE(_this call FUNC(AddClacker);false);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let triggers = &classes[0];
        assert!(triggers.properties.iter().any(|p| 
            p.name == "condition" && 
            matches!(&p.value, CodeValue::String(s) if s == "true")
        ));
        assert!(triggers.properties.iter().any(|p| 
            p.name == "onPlace" && 
            matches!(&p.value, CodeValue::String(s) if s == "_this call ace_fnc_AddClacker;false")
        ));
    }

    #[test]
    fn test_qfunc_macro() {
        let content = r#"
            #define QFUNC(var1) QUOTE(FUNC(var1))
            #define QUOTE(var1) #var1
            #define FUNC(var1) DOUBLES(DOUBLES(PREFIX,fnc),var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace

            class PatDown {
                condition = QFUNC(medical_canPatDown);
                callbackSuccess = QFUNC(medical_success);
                callbackProgress = QFUNC(medical_progress);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let patdown = &classes[0];
        assert!(patdown.properties.iter().all(|p| 
            matches!(&p.value, CodeValue::String(s) if 
                s == "ace_fnc_medical_canPatDown" ||
                s == "ace_fnc_medical_success" ||
                s == "ace_fnc_medical_progress"
            )
        ));
    }

    #[test]
    fn test_qpathtof_macro() {
        let content = r#"
            #define QPATHTOF(var1) QUOTE(PATHTOF(var1))
            #define PATHTOF(var1) PATHTO_SYS(PREFIX,COMPONENT_F,var1)
            #define PATHTO_SYS(var1,var2,var3) ##var1\##var2\##var3
            #define PREFIX ace
            #define COMPONENT_F Data

            class Command {
                picture = QPATHTOF(UI\Clacker.paa);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        let command = &classes[0];
        assert!(command.properties.iter().any(|p| 
            p.name == "picture" && 
            matches!(&p.value, CodeValue::String(s) if s == "ace\\Data\\UI\\Clacker.paa")
        ));
    }
} 
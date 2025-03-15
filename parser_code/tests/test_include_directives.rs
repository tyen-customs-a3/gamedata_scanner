#[cfg(test)]
mod tests {
    use parser_code::{CodeParser, CodeValue};

    #[test]
    fn test_basic_includes() {
        let content = r#"
            #include "script_component.hpp"
            #include "script_macros.hpp"
            
            class CfgPatches {
                class ADDON {
                    name = COMPONENT_NAME;
                    units[] = {};
                    weapons[] = {};
                    requiredVersion = REQUIRED_VERSION;
                    requiredAddons[] = {"ace_common"};
                    author = ECSTRING(common,ACETeam);
                    VERSION_CONFIG;
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "CfgPatches"));
    }


    #[test]
    fn test_multiline_macros() {
        let content = r#"
            #define MACRO_ADDWEAPON(WEAPON,COUNT) \
                class _xx_##WEAPON { \
                    weapon = #WEAPON; \
                    count = COUNT; \
                }
            
            class TransportWeapons {
                MACRO_ADDWEAPON(arifle_MX_F,2);
                MACRO_ADDWEAPON(arifle_MX_SW_F,1);
                MACRO_ADDWEAPON(launch_NLAW_F,1);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        // This test mainly checks that the parser doesn't crash
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_function_like_macros() {
        let content = r#"
            #define FUNC(var1) DOUBLES(DOUBLES(PREFIX,fnc),var1)
            #define EFUNC(var1,var2) TRIPLES(PREFIX,var1,var2)
            #define QFUNC(var1) QUOTE(FUNC(var1))
            #define QEFUNC(var1,var2) QUOTE(EFUNC(var1,var2))
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define TRIPLES(var1,var2,var3) ##var1##_##var2##_##var3
            #define QUOTE(var1) #var1
            #define PREFIX ace
            
            class Action {
                condition = QUOTE(_this call FUNC(canUseWeapon));
                statement = QUOTE(_this call EFUNC(common,execute));
                onSuccess = QFUNC(onSuccess);
                onFailure = QEFUNC(common,handleFailure);
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Action");
    }

    #[test]
    fn test_nested_includes() {
        let content = r#"
            #include "script_component.hpp"
            
            // This file includes script_mod.hpp, which includes script_macros.hpp

            class CfgEden {
                class Attributes {
                    class Default;
                    class Title: Default {
                        class Controls {
                            class Title;
                        };
                    };
                    class GVAR(fatigue): Title {
                        attributeLoad = QUOTE([ARR_2(_this,_value)] call FUNC(fatigueAttributeLoad));
                        attributeSave = QUOTE(_this call FUNC(fatigueAttributeSave));
                        class Controls: Controls {
                            class Title: Title {};
                            class Value: ctrlToolbox {
                                idc = 100;
                                rows = 1;
                                columns = 2;
                                strings[] = {CSTRING(Disabled), CSTRING(Enabled)};
                                tooltips[] = {CSTRING(Disabled), CSTRING(Enabled)};
                            };
                        };
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        // This test mainly ensures the parser doesn't crash
        assert!(classes.len() > 0);
    }
} 
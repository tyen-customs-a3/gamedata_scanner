use parser_advanced::CodeParser;
use std::fs;
use std::path::Path;

#[test]
fn test_config_rhs() {
    let content = fs::read_to_string("tests/fixtures/rhs_cfg.hpp").unwrap();
    let parser = CodeParser::new(&content).unwrap();
    let file_path = Path::new("tests/fixtures/rhs_cfg.hpp");
    let classes = parser.parse_classes(file_path);

    // RHS classes to check for existence
    let rhs_classes = vec![
        "Item_rhs_uniform_g3_m81",
        "Item_rhs_uniform_g3_aor2",
        "Item_rhs_uniform_g3_tan",
        "Item_rhs_uniform_g3_rgr",
        "Item_rhs_uniform_g3_blk",
        "Item_rhs_uniform_cu_ocp",
        "Item_rhs_uniform_cu_ucp",
        "Item_rhs_uniform_cu_ocp_1stcav",
        "Item_rhs_uniform_cu_ucp_1stcav",
        "Item_rhs_uniform_cu_ocp_82nd",
        "Item_rhs_uniform_cu_ucp_82nd",
        "Item_rhs_uniform_cu_ocp_101st",
        "Item_rhs_uniform_cu_ucp_101st",
        "Item_rhs_uniform_cu_ocp_10th",
        "Item_rhs_uniform_cu_ucp_10th",
        "Item_rhs_uniform_FROG01_d",
        "Item_rhs_uniform_FROG01_wd",
        "Item_rhs_uniform_acu_ucp",
        "Item_rhs_uniform_acu_ucp2",
        "Item_rhs_uniform_acu_ucpd",
        "Item_rhs_uniform_acu_oefcp",
        "Item_rhs_uniform_acu_ocp",
        "Item_rhs_uniform_abu",
        "Item_rhs_uniform_bdu_erdl",
        "Item_rhsusf_patrolcap_ocp",
        "Item_rhsusf_patrolcap_ucp",
        "Item_rhs_xmas_antlers",
        "Item_rhs_Booniehat_ocp",
        "Item_rhs_Booniehat_ucp",
        "Item_rhs_Booniehat_m81",
        "Item_rhs_booniehat2_marpatd",
        "Item_rhs_booniehat2_marpatwd",
        "Item_rhs_8point_marpatd",
        "Item_rhs_8point_marpatwd",
        "Item_rhsusf_ach_helmet_ocp",
        "Item_rhsusf_ach_helmet_ocp_alt",
        "Item_rhsusf_ach_helmet_ocp_norotos",
        "Item_rhsusf_ach_helmet_ucp",
        "Item_rhsusf_ach_helmet_ucp_alt",
        "Item_rhsusf_ach_helmet_ucp_norotos",
        "Item_rhsusf_ach_helmet_M81",
        "Item_rhsusf_ach_helmet_DCU",
        "Item_rhsusf_ach_helmet_DCU_early",
        "Item_rhsusf_ach_helmet_DCU_early_rhino",
        "Item_rhsusf_ach_helmet_camo_ocp",
        "Item_rhsusf_ach_helmet_headset_ocp",
        "Item_rhsusf_ach_helmet_headset_ocp_alt",
        "Item_rhsusf_ach_helmet_headset_ucp",
        "Item_rhsusf_ach_helmet_headset_ucp_alt",
        "Item_rhsusf_ach_helmet_ESS_ocp",
        "Item_rhsusf_ach_helmet_ESS_ocp_alt",
        "Item_rhsusf_ach_helmet_ESS_ucp",
        "Item_rhsusf_ach_helmet_ESS_ucp_alt",
        "Item_rhsusf_ach_helmet_headset_ess_ocp",
        "Item_rhsusf_ach_helmet_headset_ess_ocp_alt",
        "Item_rhsusf_ach_helmet_headset_ess_ucp",
        "Item_rhsusf_ach_helmet_headset_ess_ucp_alt",
        "Item_rhsusf_ach_bare",
        "Item_rhsusf_ach_bare_ess",
        "Item_rhsusf_ach_bare_headset",
        "Item_rhsusf_ach_bare_headset_ess",
        "Item_rhsusf_ach_bare_tan",
        "Item_rhsusf_ach_bare_tan_ess",
        "Item_rhsusf_ach_bare_tan_headset",
        "Item_rhsusf_ach_bare_tan_headset_ess",
        "Item_rhsusf_ach_bare_wood",
        "Item_rhsusf_ach_bare_wood_ess",
        "Item_rhsusf_ach_bare_wood_headset",
        "Item_rhsusf_ach_bare_wood_headset_ess",
        "Item_rhsusf_ach_bare_des",
        "Item_rhsusf_ach_bare_des_ess",
        "Item_rhsusf_ach_bare_des_headset",
        "Item_rhsusf_ach_bare_des_headset_ess",
        "Item_rhsusf_ach_bare_semi",
        "Item_rhsusf_ach_bare_semi_ess",
        "Item_rhsusf_ach_bare_semi_headset",
        "Item_rhsusf_ach_bare_semi_headset_ess",
        "Item_rhsusf_opscore_fg",
        "Item_rhsusf_opscore_fg_pelt",
        "Item_rhsusf_opscore_fg_pelt_nsw",
    ];

    // Collect missing classes
    let mut missing_classes = Vec::new();
    
    for class_name in rhs_classes {
        if !classes.iter().any(|c| c.name == class_name) {
            missing_classes.push(class_name);
            println!("Missing class: {}", class_name);
        }
    }
    
    // Debug print all classes
    println!("Found classes:");
    for class in &classes {
        println!("Class: {} (parent: {:?})", class.name, class.parent);
    }
    
    println!("Missing classes: {:?}", missing_classes);

    // Fail if any classes are missing
    assert!(missing_classes.is_empty(), 
        "Missing {} RHS classes: {:?}", 
        missing_classes.len(), 
        missing_classes);
}
////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sun Mar 16 19:54:11 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

//(13 Enums)
enum {
	destructengine = 2,
	destructdefault = 6,
	destructwreck = 7,
	destructtree = 3,
	destructtent = 4,
	stabilizedinaxisx = 1,
	stabilizedinaxesxyz = 4,
	stabilizedinaxisy = 2,
	stabilizedinaxesboth = 3,
	destructno = 0,
	stabilizedinaxesnone = 0,
	destructman = 5,
	destructbuilding = 1
};

class CfgPatches
{
	class rhsusf_c_troops
	{
		units[] = {"Item_rhs_8point_marpatd","Item_rhs_8point_marpatwd","Item_rhs_booniehat2_marpatd","Item_rhs_booniehat2_marpatwd","Item_rhs_Booniehat_m81","Item_rhs_Booniehat_ocp","Item_rhs_Booniehat_ucp","Item_RHS_jetpilot_usaf","Item_rhs_uniform_abu","Item_rhs_uniform_acu_ocp","Item_rhs_uniform_acu_oefcp","Item_rhs_uniform_acu_ucp","Item_rhs_uniform_acu_ucp2","Item_rhs_uniform_acu_ucpd","Item_rhs_uniform_bdu_erdl","Item_rhs_uniform_cu_ocp","Item_rhs_uniform_cu_ocp_101st","Item_rhs_uniform_cu_ocp_10th","Item_rhs_uniform_cu_ocp_1stcav","Item_rhs_uniform_cu_ocp_82nd","Item_rhs_uniform_cu_ucp","Item_rhs_uniform_cu_ucp_101st","Item_rhs_uniform_cu_ucp_10th","Item_rhs_uniform_cu_ucp_1stcav","Item_rhs_uniform_cu_ucp_82nd","Item_rhs_uniform_FROG01_d","Item_rhs_uniform_FROG01_wd","Item_rhs_uniform_g3_aor2","Item_rhs_uniform_g3_blk","Item_rhs_uniform_g3_m81","Item_rhs_uniform_g3_rgr","Item_rhs_uniform_g3_tan","Item_rhs_xmas_antlers","Item_rhsusf_ach_bare","Item_rhsusf_ach_bare_des","Item_rhsusf_ach_bare_des_ess","Item_rhsusf_ach_bare_des_headset","Item_rhsusf_ach_bare_des_headset_ess","Item_rhsusf_ach_bare_ess","Item_rhsusf_ach_bare_headset","Item_rhsusf_ach_bare_headset_ess","Item_rhsusf_ach_bare_semi","Item_rhsusf_ach_bare_semi_ess","Item_rhsusf_ach_bare_semi_headset","Item_rhsusf_ach_bare_semi_headset_ess","Item_rhsusf_ach_bare_tan","Item_rhsusf_ach_bare_tan_ess","Item_rhsusf_ach_bare_tan_headset","Item_rhsusf_ach_bare_tan_headset_ess","Item_rhsusf_ach_bare_wood","Item_rhsusf_ach_bare_wood_ess","Item_rhsusf_ach_bare_wood_headset","Item_rhsusf_ach_bare_wood_headset_ess","Item_rhsusf_ach_helmet_camo_ocp","Item_rhsusf_ach_helmet_DCU","Item_rhsusf_ach_helmet_DCU_early","Item_rhsusf_ach_helmet_DCU_early_rhino","Item_rhsusf_ach_helmet_ESS_ocp","Item_rhsusf_ach_helmet_ESS_ocp_alt","Item_rhsusf_ach_helmet_ESS_ucp","Item_rhsusf_ach_helmet_ESS_ucp_alt","Item_rhsusf_ach_helmet_headset_ess_ocp","Item_rhsusf_ach_helmet_headset_ess_ocp_alt","Item_rhsusf_ach_helmet_headset_ess_ucp","Item_rhsusf_ach_helmet_headset_ess_ucp_alt","Item_rhsusf_ach_helmet_headset_ocp","Item_rhsusf_ach_helmet_headset_ocp_alt","Item_rhsusf_ach_helmet_headset_ucp","Item_rhsusf_ach_helmet_headset_ucp_alt","Item_rhsusf_ach_helmet_M81","Item_rhsusf_ach_helmet_ocp","Item_rhsusf_ach_helmet_ocp_alt","Item_rhsusf_ach_helmet_ocp_norotos","Item_rhsusf_ach_helmet_ucp","Item_rhsusf_ach_helmet_ucp_alt","Item_rhsusf_ach_helmet_ucp_norotos","Item_rhsusf_ANPVS_14","Item_rhsusf_ANPVS_15","Item_rhsusf_Bowman","Item_rhsusf_bowman_cap","Item_rhsusf_cvc_alt_helmet","Item_rhsusf_cvc_ess","Item_rhsusf_cvc_green_alt_helmet","Item_rhsusf_cvc_green_ess","Item_rhsusf_cvc_green_helmet","Item_rhsusf_cvc_helmet","Item_rhsusf_hgu56p","Item_rhsusf_hgu56p_black","Item_rhsusf_hgu56p_green","Item_rhsusf_hgu56p_mask","Item_rhsusf_hgu56p_mask_black","Item_rhsusf_hgu56p_mask_black_skull","Item_rhsusf_hgu56p_mask_green","Item_rhsusf_hgu56p_mask_green_mo","Item_rhsusf_hgu56p_mask_mo","Item_rhsusf_hgu56p_mask_pink","Item_rhsusf_hgu56p_mask_saf","Item_rhsusf_hgu56p_mask_skull","Item_rhsusf_hgu56p_mask_smiley","Item_rhsusf_hgu56p_mask_tan","Item_rhsusf_hgu56p_pink","Item_rhsusf_hgu56p_saf","Item_rhsusf_hgu56p_tan","Item_rhsusf_hgu56p_usa","Item_rhsusf_hgu56p_visor","Item_rhsusf_hgu56p_visor_black","Item_rhsusf_hgu56p_visor_green","Item_rhsusf_hgu56p_visor_mask","Item_rhsusf_hgu56p_visor_mask_black","Item_rhsusf_hgu56p_visor_mask_black_skull","Item_rhsusf_hgu56p_visor_mask_Empire_black","Item_rhsusf_hgu56p_visor_mask_green","Item_rhsusf_hgu56p_visor_mask_green_mo","Item_rhsusf_hgu56p_visor_mask_mo","Item_rhsusf_hgu56p_visor_mask_pink","Item_rhsusf_hgu56p_visor_mask_saf","Item_rhsusf_hgu56p_visor_mask_skull","Item_rhsusf_hgu56p_visor_mask_smiley","Item_rhsusf_hgu56p_visor_mask_tan","Item_rhsusf_hgu56p_visor_pink","Item_rhsusf_hgu56p_visor_saf","Item_rhsusf_hgu56p_visor_tan","Item_rhsusf_hgu56p_visor_usa","Item_rhsusf_hgu56p_visor_white","Item_rhsusf_hgu56p_white","Item_rhsusf_ihadss","Item_rhsusf_iotv_ocp","Item_rhsusf_iotv_ocp_Grenadier","Item_rhsusf_iotv_ocp_Medic","Item_rhsusf_iotv_ocp_Repair","Item_rhsusf_iotv_ocp_Rifleman","Item_rhsusf_iotv_ocp_SAW","Item_rhsusf_iotv_ocp_Squadleader","Item_rhsusf_iotv_ocp_Teamleader","Item_rhsusf_iotv_ucp","Item_rhsusf_iotv_ucp_Grenadier","Item_rhsusf_iotv_ucp_Medic","Item_rhsusf_iotv_ucp_Repair","Item_rhsusf_iotv_ucp_Rifleman","Item_rhsusf_iotv_ucp_SAW","Item_rhsusf_iotv_ucp_Squadleader","Item_rhsusf_iotv_ucp_Teamleader","Item_rhsusf_lwh_helmet_M1942","Item_rhsusf_lwh_helmet_marpatd","Item_rhsusf_lwh_helmet_marpatd_ess","Item_rhsusf_lwh_helmet_marpatd_headset","Item_rhsusf_lwh_helmet_marpatwd","Item_rhsusf_lwh_helmet_marpatwd_blk_ess","Item_rhsusf_lwh_helmet_marpatwd_ess","Item_rhsusf_lwh_helmet_marpatwd_headset","Item_rhsusf_lwh_helmet_marpatwd_headset_blk","Item_rhsusf_lwh_helmet_marpatwd_headset_blk2","Item_rhsusf_mbav","Item_rhsusf_mbav_grenadier","Item_rhsusf_mbav_light","Item_rhsusf_mbav_medic","Item_rhsusf_mbav_mg","Item_rhsusf_mbav_rifleman","Item_rhsusf_mich_bare","Item_rhsusf_mich_bare_alt","Item_rhsusf_mich_bare_alt_semi","Item_rhsusf_mich_bare_alt_tan","Item_rhsusf_mich_bare_headset","Item_rhsusf_mich_bare_norotos","Item_rhsusf_mich_bare_norotos_alt","Item_rhsusf_mich_bare_norotos_alt_headset","Item_rhsusf_mich_bare_norotos_alt_semi","Item_rhsusf_mich_bare_norotos_alt_semi_headset","Item_rhsusf_mich_bare_norotos_alt_tan","Item_rhsusf_mich_bare_norotos_alt_tan_headset","Item_rhsusf_mich_bare_norotos_arc","Item_rhsusf_mich_bare_norotos_arc_alt","Item_rhsusf_mich_bare_norotos_arc_alt_headset","Item_rhsusf_mich_bare_norotos_arc_alt_semi","Item_rhsusf_mich_bare_norotos_arc_alt_semi_headset","Item_rhsusf_mich_bare_norotos_arc_alt_tan","Item_rhsusf_mich_bare_norotos_arc_alt_tan_headset","Item_rhsusf_mich_bare_norotos_arc_headset","Item_rhsusf_mich_bare_norotos_arc_semi","Item_rhsusf_mich_bare_norotos_arc_semi_headset","Item_rhsusf_mich_bare_norotos_arc_tan","Item_rhsusf_mich_bare_norotos_headset","Item_rhsusf_mich_bare_norotos_semi","Item_rhsusf_mich_bare_norotos_semi_headset","Item_rhsusf_mich_bare_norotos_tan","Item_rhsusf_mich_bare_norotos_tan_headset","Item_rhsusf_mich_bare_semi","Item_rhsusf_mich_bare_semi_headset","Item_rhsusf_mich_bare_tan","Item_rhsusf_mich_bare_tan_headset","Item_rhsusf_mich_helmet_marpatd","Item_rhsusf_mich_helmet_marpatd_alt","Item_rhsusf_mich_helmet_marpatd_alt_headset","Item_rhsusf_mich_helmet_marpatd_headset","Item_rhsusf_mich_helmet_marpatd_norotos","Item_rhsusf_mich_helmet_marpatd_norotos_arc","Item_rhsusf_mich_helmet_marpatd_norotos_arc_headset","Item_rhsusf_mich_helmet_marpatd_norotos_headset","Item_rhsusf_mich_helmet_marpatwd","Item_rhsusf_mich_helmet_marpatwd_alt","Item_rhsusf_mich_helmet_marpatwd_alt_headset","Item_rhsusf_mich_helmet_marpatwd_headset","Item_rhsusf_mich_helmet_marpatwd_norotos","Item_rhsusf_mich_helmet_marpatwd_norotos_arc","Item_rhsusf_mich_helmet_marpatwd_norotos_arc_headset","Item_rhsusf_mich_helmet_marpatwd_norotos_headset","Item_rhsusf_opscore_aor1","Item_rhsusf_opscore_aor1_pelt","Item_rhsusf_opscore_aor1_pelt_nsw","Item_rhsusf_opscore_aor2","Item_rhsusf_opscore_aor2_pelt","Item_rhsusf_opscore_aor2_pelt_nsw","Item_rhsusf_opscore_bk","Item_rhsusf_opscore_bk_pelt","Item_rhsusf_opscore_coy_cover","Item_rhsusf_opscore_coy_cover_pelt","Item_rhsusf_opscore_fg","Item_rhsusf_opscore_fg_pelt","Item_rhsusf_opscore_fg_pelt_cam","Item_rhsusf_opscore_fg_pelt_nsw","Item_rhsusf_opscore_mar_fg","Item_rhsusf_opscore_mar_fg_pelt","Item_rhsusf_opscore_mar_ut","Item_rhsusf_opscore_mar_ut_pelt","Item_rhsusf_opscore_mc","Item_rhsusf_opscore_mc_cover","Item_rhsusf_opscore_mc_cover_pelt","Item_rhsusf_opscore_mc_cover_pelt_cam","Item_rhsusf_opscore_mc_cover_pelt_nsw","Item_rhsusf_opscore_mc_pelt","Item_rhsusf_opscore_mc_pelt_nsw","Item_rhsusf_opscore_paint","Item_rhsusf_opscore_paint_pelt","Item_rhsusf_opscore_paint_pelt_nsw","Item_rhsusf_opscore_paint_pelt_nsw_cam","Item_rhsusf_opscore_rg_cover","Item_rhsusf_opscore_rg_cover_pelt","Item_rhsusf_opscore_ut","Item_rhsusf_opscore_ut_pelt","Item_rhsusf_opscore_ut_pelt_cam","Item_rhsusf_opscore_ut_pelt_nsw","Item_rhsusf_opscore_ut_pelt_nsw_cam","Item_rhsusf_patrolcap_ocp","Item_rhsusf_patrolcap_ucp","Item_rhsusf_plateframe_grenadier","Item_rhsusf_plateframe_light","Item_rhsusf_plateframe_machinegunner","Item_rhsusf_plateframe_marksman","Item_rhsusf_plateframe_medic","Item_rhsusf_plateframe_rifleman","Item_rhsusf_plateframe_sapi","Item_rhsusf_plateframe_teamleader","Item_rhsusf_protech_helmet","Item_rhsusf_protech_helmet_ess","Item_rhsusf_protech_helmet_rhino","Item_rhsusf_protech_helmet_rhino_ess","Item_rhsusf_Rhino","Item_rhsusf_spc","Item_rhsusf_spc_corpsman","Item_rhsusf_spc_crewman","Item_rhsusf_spc_iar","Item_rhsusf_spc_light","Item_rhsusf_spc_marksman","Item_rhsusf_spc_mg","Item_rhsusf_spc_patchless","Item_rhsusf_spc_patchless_radio","Item_rhsusf_spc_rifleman","Item_rhsusf_spc_sniper","Item_rhsusf_spc_squadleader","Item_rhsusf_spc_teamleader","Item_rhsusf_spcs_ocp","Item_rhsusf_spcs_ocp_crewman","Item_rhsusf_spcs_ocp_grenadier","Item_rhsusf_spcs_ocp_machinegunner","Item_rhsusf_spcs_ocp_medic","Item_rhsusf_spcs_ocp_rifleman","Item_rhsusf_spcs_ocp_rifleman_alt","Item_rhsusf_spcs_ocp_saw","Item_rhsusf_spcs_ocp_sniper","Item_rhsusf_spcs_ocp_squadleader","Item_rhsusf_spcs_ocp_teamleader","Item_rhsusf_spcs_ocp_teamleader_alt","Item_rhsusf_spcs_ucp","Item_rhsusf_spcs_ucp_crewman","Item_rhsusf_spcs_ucp_grenadier","Item_rhsusf_spcs_ucp_machinegunner","Item_rhsusf_spcs_ucp_medic","Item_rhsusf_spcs_ucp_rifleman","Item_rhsusf_spcs_ucp_rifleman_alt","Item_rhsusf_spcs_ucp_saw","Item_rhsusf_spcs_ucp_sniper","Item_rhsusf_spcs_ucp_squadleader","Item_rhsusf_spcs_ucp_teamleader","Item_rhsusf_spcs_ucp_teamleader_alt","rhsusf_airforce_jetpilot","rhsusf_airforce_pilot","rhsusf_airforce_security_force_rifleman","rhsusf_army_ocp_aa","rhsusf_army_ocp_ah64_pilot","rhsusf_army_ocp_arb_autorifleman","rhsusf_army_ocp_arb_autoriflemana","rhsusf_army_ocp_arb_engineer","rhsusf_army_ocp_arb_grenadier","rhsusf_army_ocp_arb_maaws","rhsusf_army_ocp_arb_machinegunner","rhsusf_army_ocp_arb_machinegunnera","rhsusf_army_ocp_arb_marksman","rhsusf_army_ocp_arb_medic","rhsusf_army_ocp_arb_rifleman","rhsusf_army_ocp_arb_rifleman_m590","rhsusf_army_ocp_arb_riflemanat","rhsusf_army_ocp_arb_riflemanl","rhsusf_army_ocp_arb_sniper_m107","rhsusf_army_ocp_arb_squadleader","rhsusf_army_ocp_arb_teamleader","rhsusf_army_ocp_autorifleman","rhsusf_army_ocp_autoriflemana","rhsusf_army_ocp_combatcrewman","rhsusf_army_ocp_crewman","rhsusf_army_ocp_driver","rhsusf_army_ocp_driver_armored","rhsusf_army_ocp_engineer","rhsusf_army_ocp_explosives","rhsusf_army_ocp_fso","rhsusf_army_ocp_grenadier","rhsusf_army_ocp_helicrew","rhsusf_army_ocp_helipilot","rhsusf_army_ocp_javelin","rhsusf_army_ocp_javelin_assistant","rhsusf_army_ocp_jfo","rhsusf_army_ocp_maaws","rhsusf_army_ocp_machinegunner","rhsusf_army_ocp_machinegunnera","rhsusf_army_ocp_marksman","rhsusf_army_ocp_medic","rhsusf_army_ocp_officer","rhsusf_army_ocp_rifleman","rhsusf_army_ocp_rifleman_101st","rhsusf_army_ocp_rifleman_10th","rhsusf_army_ocp_rifleman_1stcav","rhsusf_army_ocp_rifleman_82nd","rhsusf_army_ocp_rifleman_arb_m16","rhsusf_army_ocp_rifleman_m16","rhsusf_army_ocp_rifleman_m4","rhsusf_army_ocp_rifleman_m590","rhsusf_army_ocp_riflemanat","rhsusf_army_ocp_riflemanl","rhsusf_army_ocp_sniper","rhsusf_army_ocp_sniper_m107","rhsusf_army_ocp_sniper_m24sws","rhsusf_army_ocp_squadleader","rhsusf_army_ocp_teamleader","rhsusf_army_ocp_uav","rhsusf_army_ucp_aa","rhsusf_army_ucp_ah64_pilot","rhsusf_army_ucp_arb_autorifleman","rhsusf_army_ucp_arb_autoriflemana","rhsusf_army_ucp_arb_engineer","rhsusf_army_ucp_arb_grenadier","rhsusf_army_ucp_arb_maaws","rhsusf_army_ucp_arb_machinegunner","rhsusf_army_ucp_arb_machinegunnera","rhsusf_army_ucp_arb_marksman","rhsusf_army_ucp_arb_medic","rhsusf_army_ucp_arb_rifleman","rhsusf_army_ucp_arb_rifleman_m590","rhsusf_army_ucp_arb_riflemanat","rhsusf_army_ucp_arb_riflemanl","rhsusf_army_ucp_arb_sniper_m107","rhsusf_army_ucp_arb_squadleader","rhsusf_army_ucp_arb_teamleader","rhsusf_army_ucp_autorifleman","rhsusf_army_ucp_autoriflemana","rhsusf_army_ucp_combatcrewman","rhsusf_army_ucp_crewman","rhsusf_army_ucp_driver","rhsusf_army_ucp_driver_armored","rhsusf_army_ucp_engineer","rhsusf_army_ucp_explosives","rhsusf_army_ucp_fso","rhsusf_army_ucp_grenadier","rhsusf_army_ucp_helicrew","rhsusf_army_ucp_helipilot","rhsusf_army_ucp_javelin","rhsusf_army_ucp_javelin_assistant","rhsusf_army_ucp_jfo","rhsusf_army_ucp_maaws","rhsusf_army_ucp_machinegunner","rhsusf_army_ucp_machinegunnera","rhsusf_army_ucp_marksman","rhsusf_army_ucp_medic","rhsusf_army_ucp_officer","rhsusf_army_ucp_rifleman","rhsusf_army_ucp_rifleman_101st","rhsusf_army_ucp_rifleman_10th","rhsusf_army_ucp_rifleman_1stcav","rhsusf_army_ucp_rifleman_82nd","rhsusf_army_ucp_rifleman_arb_m16","rhsusf_army_ucp_rifleman_m16","rhsusf_army_ucp_rifleman_m4","rhsusf_army_ucp_rifleman_m590","rhsusf_army_ucp_riflemanat","rhsusf_army_ucp_riflemanl","rhsusf_army_ucp_sniper","rhsusf_army_ucp_sniper_m107","rhsusf_army_ucp_sniper_m24sws","rhsusf_army_ucp_squadleader","rhsusf_army_ucp_teamleader","rhsusf_army_ucp_uav","rhsusf_assault_eagleaiii_coy","rhsusf_assault_eagleaiii_ocp","rhsusf_assault_eagleaiii_ucp","rhsusf_eject_Parachute_backpack","rhsusf_falconii","rhsusf_falconii_coy","rhsusf_falconii_mc","rhsusf_infantry_socom_armysf_rifleman","rhsusf_navy_marpat_d_medic","rhsusf_navy_marpat_wd_medic","rhsusf_navy_sarc_d","rhsusf_navy_sarc_d_fast","rhsusf_navy_sarc_w","rhsusf_navy_sarc_w_fast","rhsusf_socom_marsoc_cso","rhsusf_socom_marsoc_cso_breacher","rhsusf_socom_marsoc_cso_cqb","rhsusf_socom_marsoc_cso_eod","rhsusf_socom_marsoc_cso_grenadier","rhsusf_socom_marsoc_cso_light","rhsusf_socom_marsoc_cso_mechanic","rhsusf_socom_marsoc_cso_mk17","rhsusf_socom_marsoc_cso_mk17_light","rhsusf_socom_marsoc_elementleader","rhsusf_socom_marsoc_jfo","rhsusf_socom_marsoc_jtac","rhsusf_socom_marsoc_marksman","rhsusf_socom_marsoc_sarc","rhsusf_socom_marsoc_sniper","rhsusf_socom_marsoc_sniper_m107","rhsusf_socom_marsoc_spotter","rhsusf_socom_marsoc_teamchief","rhsusf_socom_marsoc_teamleader","rhsusf_socom_swcc_crewman","rhsusf_socom_swcc_officer","rhsusf_usmc_lar_marpat_d_autorifleman","rhsusf_usmc_lar_marpat_d_combatcrewman","rhsusf_usmc_lar_marpat_d_crewman","rhsusf_usmc_lar_marpat_d_grenadier_m32","rhsusf_usmc_lar_marpat_d_machinegunner","rhsusf_usmc_lar_marpat_d_marksman","rhsusf_usmc_lar_marpat_d_rifleman","rhsusf_usmc_lar_marpat_d_rifleman_light","rhsusf_usmc_lar_marpat_d_riflemanat","rhsusf_usmc_lar_marpat_d_squadleader","rhsusf_usmc_lar_marpat_d_teamleader","rhsusf_usmc_lar_marpat_wd_autorifleman","rhsusf_usmc_lar_marpat_wd_combatcrewman","rhsusf_usmc_lar_marpat_wd_crewman","rhsusf_usmc_lar_marpat_wd_grenadier_m32","rhsusf_usmc_lar_marpat_wd_machinegunner","rhsusf_usmc_lar_marpat_wd_marksman","rhsusf_usmc_lar_marpat_wd_rifleman","rhsusf_usmc_lar_marpat_wd_rifleman_light","rhsusf_usmc_lar_marpat_wd_riflemanat","rhsusf_usmc_lar_marpat_wd_squadleader","rhsusf_usmc_lar_marpat_wd_teamleader","rhsusf_usmc_marpat_d_autorifleman","rhsusf_usmc_marpat_d_autorifleman_m249","rhsusf_usmc_marpat_d_autorifleman_m249_ass","rhsusf_usmc_marpat_d_combatcrewman","rhsusf_usmc_marpat_d_crewman","rhsusf_usmc_marpat_d_driver","rhsusf_usmc_marpat_d_engineer","rhsusf_usmc_marpat_d_explosives","rhsusf_usmc_marpat_d_fso","rhsusf_usmc_marpat_d_grenadier","rhsusf_usmc_marpat_d_grenadier_m32","rhsusf_usmc_marpat_d_gunner","rhsusf_usmc_marpat_d_helicrew","rhsusf_usmc_marpat_d_helipilot","rhsusf_usmc_marpat_d_javelin","rhsusf_usmc_marpat_d_javelin_assistant","rhsusf_usmc_marpat_d_jfo","rhsusf_usmc_marpat_d_machinegunner","rhsusf_usmc_marpat_d_machinegunner_ass","rhsusf_usmc_marpat_d_marksman","rhsusf_usmc_marpat_d_officer","rhsusf_usmc_marpat_d_rifleman","rhsusf_usmc_marpat_d_rifleman_law","rhsusf_usmc_marpat_d_rifleman_light","rhsusf_usmc_marpat_d_rifleman_m4","rhsusf_usmc_marpat_d_rifleman_m590","rhsusf_usmc_marpat_d_riflemanat","rhsusf_usmc_marpat_d_smaw","rhsusf_usmc_marpat_d_sniper","rhsusf_usmc_marpat_d_sniper_m107","rhsusf_usmc_marpat_d_sniper_m110","rhsusf_usmc_marpat_d_spotter","rhsusf_usmc_marpat_d_squadleader","rhsusf_usmc_marpat_d_stinger","rhsusf_usmc_marpat_d_teamleader","rhsusf_usmc_marpat_d_uav","rhsusf_usmc_marpat_wd_autorifleman","rhsusf_usmc_marpat_wd_autorifleman_m249","rhsusf_usmc_marpat_wd_autorifleman_m249_ass","rhsusf_usmc_marpat_wd_combatcrewman","rhsusf_usmc_marpat_wd_crewman","rhsusf_usmc_marpat_wd_driver","rhsusf_usmc_marpat_wd_engineer","rhsusf_usmc_marpat_wd_explosives","rhsusf_usmc_marpat_wd_fso","rhsusf_usmc_marpat_wd_grenadier","rhsusf_usmc_marpat_wd_grenadier_m32","rhsusf_usmc_marpat_wd_gunner","rhsusf_usmc_marpat_wd_helicrew","rhsusf_usmc_marpat_wd_helipilot","rhsusf_usmc_marpat_wd_javelin","rhsusf_usmc_marpat_wd_javelin_assistant","rhsusf_usmc_marpat_wd_jfo","rhsusf_usmc_marpat_wd_machinegunner","rhsusf_usmc_marpat_wd_machinegunner_ass","rhsusf_usmc_marpat_wd_marksman","rhsusf_usmc_marpat_wd_officer","rhsusf_usmc_marpat_wd_rifleman","rhsusf_usmc_marpat_wd_rifleman_law","rhsusf_usmc_marpat_wd_rifleman_light","rhsusf_usmc_marpat_wd_rifleman_m4","rhsusf_usmc_marpat_wd_rifleman_m590","rhsusf_usmc_marpat_wd_riflemanat","rhsusf_usmc_marpat_wd_smaw","rhsusf_usmc_marpat_wd_sniper","rhsusf_usmc_marpat_wd_sniper_M107","rhsusf_usmc_marpat_wd_sniper_m110","rhsusf_usmc_marpat_wd_spotter","rhsusf_usmc_marpat_wd_squadleader","rhsusf_usmc_marpat_wd_stinger","rhsusf_usmc_marpat_wd_teamleader","rhsusf_usmc_marpat_wd_uav","rhsusf_usmc_recon_marpat_d_autorifleman","rhsusf_usmc_recon_marpat_d_autorifleman_fast","rhsusf_usmc_recon_marpat_d_autorifleman_lite","rhsusf_usmc_recon_marpat_d_grenadier_m32","rhsusf_usmc_recon_marpat_d_machinegunner","rhsusf_usmc_recon_marpat_d_machinegunner_m249","rhsusf_usmc_recon_marpat_d_machinegunner_m249_fast","rhsusf_usmc_recon_marpat_d_machinegunner_m249_lite","rhsusf_usmc_recon_marpat_d_marksman","rhsusf_usmc_recon_marpat_d_marksman_fast","rhsusf_usmc_recon_marpat_d_marksman_lite","rhsusf_usmc_recon_marpat_d_officer","rhsusf_usmc_recon_marpat_d_rifleman","rhsusf_usmc_recon_marpat_d_rifleman_at","rhsusf_usmc_recon_marpat_d_rifleman_at_fast","rhsusf_usmc_recon_marpat_d_rifleman_at_lite","rhsusf_usmc_recon_marpat_d_rifleman_fast","rhsusf_usmc_recon_marpat_d_rifleman_lite","rhsusf_usmc_recon_marpat_d_sniper_M107","rhsusf_usmc_recon_marpat_d_teamleader","rhsusf_usmc_recon_marpat_d_teamleader_fast","rhsusf_usmc_recon_marpat_d_teamleader_lite","rhsusf_usmc_recon_marpat_wd_autorifleman","rhsusf_usmc_recon_marpat_wd_autorifleman_fast","rhsusf_usmc_recon_marpat_wd_autorifleman_lite","rhsusf_usmc_recon_marpat_wd_grenadier_m32","rhsusf_usmc_recon_marpat_wd_machinegunner","rhsusf_usmc_recon_marpat_wd_machinegunner_m249","rhsusf_usmc_recon_marpat_wd_machinegunner_m249_fast","rhsusf_usmc_recon_marpat_wd_machinegunner_m249_lite","rhsusf_usmc_recon_marpat_wd_marksman","rhsusf_usmc_recon_marpat_wd_marksman_fast","rhsusf_usmc_recon_marpat_wd_marksman_lite","rhsusf_usmc_recon_marpat_wd_officer","rhsusf_usmc_recon_marpat_wd_rifleman","rhsusf_usmc_recon_marpat_wd_rifleman_at","rhsusf_usmc_recon_marpat_wd_rifleman_at_fast","rhsusf_usmc_recon_marpat_wd_rifleman_at_lite","rhsusf_usmc_recon_marpat_wd_rifleman_fast","rhsusf_usmc_recon_marpat_wd_rifleman_lite","rhsusf_usmc_recon_marpat_wd_sniper_M107","rhsusf_usmc_recon_marpat_wd_teamleader","rhsusf_usmc_recon_marpat_wd_teamleader_fast","rhsusf_usmc_recon_marpat_wd_teamleader_lite"};
		weapons[] = {"rhs_uniform_g3_mc","rhs_uniform_g3_m81","rhs_uniform_g3_aor2","rhs_uniform_g3_tan","rhs_uniform_g3_rgr","rhs_uniform_g3_blk","rhs_uniform_cu_ocp","rhs_uniform_cu_ucp","rhs_uniform_cu_ocp","rhs_uniform_cu_ocp_1stcav","rhs_uniform_cu_ocp","rhs_uniform_cu_ucp_1stcav","rhs_uniform_cu_ucp","rhs_uniform_cu_ocp_82nd","rhs_uniform_cu_ocp","rhs_uniform_cu_ucp_82nd","rhs_uniform_cu_ucp","rhs_uniform_cu_ocp_101st","rhs_uniform_cu_ocp","rhs_uniform_cu_ucp_101st","rhs_uniform_cu_ucp","rhs_uniform_cu_ocp_10th","rhs_uniform_cu_ocp","rhs_uniform_cu_ucp_10th","rhs_uniform_cu_ucp","rhs_uniform_cu_ucp_patchless","rhs_uniform_cu_ocp","rhs_uniform_cu_ocp_patchless","rhs_uniform_cu_ucp","rhs_uniform_FROG01_d","rhs_uniform_cu_ocp","rhs_uniform_FROG01_wd","rhs_uniform_FROG01_d","rhs_uniform_FROG01_m81","rhs_uniform_acu_ucp","rhs_uniform_acu_ucp2","rhs_uniform_acu_ucpd","rhs_uniform_acu_oefcp","rhs_uniform_acu_ocp","rhs_uniform_abu","rhs_uniform_bdu_erdl","rhs_xmas_antlers","rhs_Booniehat_ocp","rhs_Booniehat_ucp","rhs_Booniehat_marpatd","rhs_Booniehat_marpatwd","rhs_Booniehat_m81","rhs_booniehat2_marpatd","rhs_booniehat2_marpatwd","rhs_8point_marpatd","rhs_8point_marpatwd","RHS_jetpilot_usaf","rhsusf_patrolcap_ocp","rhsusf_patrolcap_ucp","rhsusf_ach_helmet_ocp","rhsusf_ach_helmet_ocp_alt","rhsusf_ach_helmet_ocp_norotos","rhsusf_ach_helmet_ucp","rhsusf_ach_helmet_ucp_alt","rhsusf_ach_helmet_ucp_norotos","rhsusf_ach_helmet_M81","rhsusf_ach_helmet_camo_ocp","rhsusf_ach_helmet_headset_ocp","rhsusf_ach_helmet_headset_ocp_alt","rhsusf_ach_helmet_headset_ucp","rhsusf_ach_helmet_headset_ucp_alt","rhsusf_ach_helmet_ESS_ocp","rhsusf_ach_helmet_ESS_ocp_alt","rhsusf_ach_helmet_ESS_ucp","rhsusf_ach_helmet_ESS_ucp_alt","rhsusf_ach_helmet_headset_ess_ocp","rhsusf_ach_helmet_headset_ess_ocp_alt","rhsusf_ach_helmet_headset_ess_ucp","rhsusf_ach_helmet_headset_ess_ucp_alt","rhsusf_ach_bare","rhsusf_ach_bare_ess","rhsusf_ach_bare_headset","rhsusf_ach_bare_headset_ess","rhsusf_ach_bare_tan","rhsusf_ach_bare_tan_ess","rhsusf_ach_bare_tan_headset","rhsusf_ach_bare_tan_headset_ess","rhsusf_ach_bare_wood","rhsusf_ach_bare_wood_ess","rhsusf_ach_bare_wood_headset","rhsusf_ach_bare_wood_headset_ess","rhsusf_ach_bare_des","rhsusf_ach_bare_des_ess","rhsusf_ach_bare_des_headset","rhsusf_ach_bare_des_headset_ess","rhsusf_ach_bare_semi","rhsusf_ach_bare_semi_ess","rhsusf_ach_bare_semi_headset","rhsusf_ach_bare_semi_headset_ess","rhsusf_opscore_01","rhsusf_opscore_fg","rhsusf_opscore_fg_pelt","rhsusf_opscore_fg_pelt_nsw","rhsusf_opscore_fg_pelt_cam","rhsusf_opscore_ut","rhsusf_opscore_ut_pelt","rhsusf_opscore_ut_pelt_cam","rhsusf_opscore_ut_pelt_nsw","rhsusf_opscore_ut_pelt_nsw_cam","rhsusf_opscore_bk","rhsusf_opscore_bk_pelt","rhsusf_opscore_mc","rhsusf_opscore_mc_pelt","rhsusf_opscore_mc_pelt_nsw","rhsusf_opscore_aor1","rhsusf_opscore_aor1_pelt","rhsusf_opscore_aor1_pelt_nsw","rhsusf_opscore_aor2","rhsusf_opscore_aor2_pelt","rhsusf_opscore_aor2_pelt_nsw","rhsusf_opscore_paint","rhsusf_opscore_paint_pelt","rhsusf_opscore_paint_pelt_nsw","rhsusf_opscore_paint_pelt_nsw_cam","rhsusf_opscore_cover","rhsusf_opscore_mc_cover","rhsusf_opscore_mc_cover_pelt","rhsusf_opscore_mc_cover_pelt_nsw","rhsusf_opscore_mc_cover_pelt_cam","rhsusf_opscore_rg_cover","rhsusf_opscore_rg_cover_pelt","rhsusf_opscore_coy_cover","rhsusf_opscore_coy_cover_pelt","rhsusf_opscore_mar_01","rhsusf_opscore_mar_ut","rhsusf_opscore_mar_ut_pelt","rhsusf_opscore_mar_fg","rhsusf_opscore_mar_fg_pelt","rhsusf_opscore_01_tan","rhsusf_opscore_02","rhsusf_opscore_02_tan","rhsusf_opscore_03_ocp","rhsusf_opscore_04_ocp","rhsusf_cvc_helmet","rhsusf_cvc_alt_helmet","rhsusf_cvc_green_helmet","rhsusf_cvc_green_alt_helmet","rhsusf_cvc_ess","rhsusf_cvc_green_ess","rhsusf_hgu56p","rhsusf_hgu56p_visor","rhsusf_hgu56p_mask","rhsusf_hgu56p_visor_mask","rhsusf_hgu56p_visor_mask_skull","rhsusf_hgu56p_mask_skull","rhsusf_hgu56p_visor_mask_mo","rhsusf_hgu56p_mask_mo","rhsusf_hgu56p_mask_smiley","rhsusf_hgu56p_visor_mask_smiley","rhsusf_hgu56p_black","rhsusf_hgu56p_visor_black","rhsusf_hgu56p_mask_black","rhsusf_hgu56p_visor_mask_black","rhsusf_hgu56p_visor_mask_black_skull","rhsusf_hgu56p_mask_black_skull","rhsusf_hgu56p_visor_mask_Empire_black","rhsusf_hgu56p_green","rhsusf_hgu56p_visor_green","rhsusf_hgu56p_mask_green","rhsusf_hgu56p_visor_mask_green","rhsusf_hgu56p_mask_green_mo","rhsusf_hgu56p_visor_mask_green_mo","rhsusf_hgu56p_tan","rhsusf_hgu56p_visor_tan","rhsusf_hgu56p_mask_tan","rhsusf_hgu56p_visor_mask_tan","rhsusf_hgu56p_pink","rhsusf_hgu56p_visor_pink","rhsusf_hgu56p_mask_pink","rhsusf_hgu56p_visor_mask_pink","rhsusf_hgu56p_usa","rhsusf_hgu56p_visor_usa","rhsusf_hgu56p_saf","rhsusf_hgu56p_visor_saf","rhsusf_hgu56p_mask_saf","rhsusf_hgu56p_visor_mask_saf","rhsusf_ANPVS_14","rhsusf_ANPVS_15","rhsusf_Rhino","rhsusf_anvis_nvg_bc_caps","rhsusf_ANVIS","rhsusf_mich_helmet_marpatwd","rhsusf_mich_helmet_marpatwd_headset","rhsusf_mich_helmet_marpatd","rhsusf_mich_helmet_marpatd_headset","rhsusf_mich_helmet_marpatwd_alt","rhsusf_mich_helmet_marpatwd_alt_headset","rhsusf_mich_helmet_marpatd_alt","rhsusf_mich_helmet_marpatd_alt_headset","rhsusf_mich_helmet_marpatwd_norotos","rhsusf_mich_helmet_marpatwd_norotos_headset","rhsusf_mich_helmet_marpatd_norotos","rhsusf_mich_helmet_marpatd_norotos_headset","rhsusf_mich_helmet_marpatwd_norotos_arc","rhsusf_mich_helmet_marpatwd_norotos_arc_headset","rhsusf_mich_helmet_marpatd_norotos_arc","rhsusf_mich_helmet_marpatd_norotos_arc_headset","rhsusf_mich_bare","rhsusf_mich_bare_headset","rhsusf_mich_bare_alt","rhsusf_mich_bare_norotos","rhsusf_mich_bare_norotos_headset","rhsusf_mich_bare_norotos_alt","rhsusf_mich_bare_norotos_alt_headset","rhsusf_mich_bare_norotos_arc","rhsusf_mich_bare_norotos_arc_headset","rhsusf_mich_bare_norotos_arc_alt","rhsusf_mich_bare_norotos_arc_alt_headset","rhsusf_mich_bare_tan","rhsusf_mich_bare_tan_headset","rhsusf_mich_bare_alt_tan","rhsusf_mich_bare_norotos_tan","rhsusf_mich_bare_norotos_tan_headset","rhsusf_mich_bare_norotos_alt_tan","rhsusf_mich_bare_norotos_alt_tan_headset","rhsusf_mich_bare_norotos_arc_tan","rhsusf_mich_bare_norotos_arc_tan_headset","rhsusf_mich_bare_norotos_arc_alt_tan","rhsusf_mich_bare_norotos_arc_alt_tan_headset","rhsusf_mich_bare_semi","rhsusf_mich_bare_semi_headset","rhsusf_mich_bare_alt_semi","rhsusf_mich_bare_norotos_semi","rhsusf_mich_bare_norotos_semi_headset","rhsusf_mich_bare_norotos_alt_semi","rhsusf_mich_bare_norotos_alt_semi_headset","rhsusf_mich_bare_norotos_arc_semi","rhsusf_mich_bare_norotos_arc_semi_headset","rhsusf_mich_bare_norotos_arc_alt_semi","rhsusf_mich_bare_norotos_arc_alt_semi_headset","rhsusf_ach_helmet_ocp","rhsusf_lwh_helmet_marpatd","rhsusf_lwh_helmet_marpatd_ess","rhsusf_lwh_helmet_marpatd_headset","rhsusf_lwh_helmet_marpatwd","rhsusf_lwh_helmet_marpatwd_ess","rhsusf_lwh_helmet_marpatwd_blk_ess","rhsusf_lwh_helmet_marpatwd_headset","rhsusf_lwh_helmet_marpatwd_headset_blk","rhsusf_lwh_helmet_M1942","rhsusf_Bowman","rhsusf_bowman_cap","rhsusf_protech_helmet","rhsusf_protech_helmet_ess","rhsusf_protech_helmet_rhino","rhsusf_protech_helmet_rhino_ess","rhsusf_iotv_ocp","rhsusf_iotv_ocp_Grenadier","rhsusf_iotv_ocp_Medic","rhsusf_iotv_ocp_Repair","rhsusf_iotv_ocp_Rifleman","rhsusf_iotv_ocp_SAW","rhsusf_iotv_ocp_Squadleader","rhsusf_iotv_ocp_Teamleader","rhsusf_iotv_ucp","rhsusf_iotv_ucp_Grenadier","rhsusf_iotv_ucp_Medic","rhsusf_iotv_ucp_Repair","rhsusf_iotv_ucp_Rifleman","rhsusf_iotv_ucp_SAW","rhsusf_iotv_ucp_Squadleader","rhsusf_iotv_ucp_Teamleader","rhsusf_spcs_ocp_squadleader","rhsusf_spcs_ocp_teamleader","rhsusf_spcs_ocp_teamleader_alt","rhsusf_spcs_ocp_saw","rhsusf_spcs_ocp_grenadier","rhsusf_spcs_ocp_rifleman","rhsusf_spcs_ocp_rifleman_alt","rhsusf_spcs_ocp_medic","rhsusf_spcs_ocp_crewman","rhsusf_spcs_ocp_machinegunner","rhsusf_spcs_ocp_sniper","rhsusf_spcs_ucp","rhsusf_spcs_ucp_squadleader","rhsusf_spcs_ucp_teamleader","rhsusf_spcs_ucp_teamleader_alt","rhsusf_spcs_ucp_saw","rhsusf_spcs_ucp_grenadier","rhsusf_spcs_ucp_rifleman","rhsusf_spcs_ucp_rifleman_alt","rhsusf_spcs_ucp_medic","rhsusf_spcs_ucp_crewman","rhsusf_spcs_ucp_machinegunner","rhsusf_spcs_ucp_sniper","rhsusf_mbav","rhsusf_mbav_light","rhsusf_mbav_rifleman","rhsusf_mbav_mg","rhsusf_mbav_grenadier","rhsusf_mbav_medic","rhsusf_plateframe_sapi"};
		requiredVersion = 1.72;
		requiredAddons[] = {"rhsusf_main","rhsusf_c_weapons"};
		name = "RHSUSF Infantry & Equipment";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		url = "http://www.rhsmods.org/";
	};
};
class CfgFunctions
{
	class RHS
	{
		tag = "RHS";
		class functions
		{
			class nvgHandler
			{
				file = "\rhsusf\addons\rhsusf_c_troops\scripts\rhs_nvghandler.sqf";
				description = "NVG equip";
			};
		};
	};
	class RHSUSF
	{
		tag = "RHS";
		class functions
		{
			class nvgHandler
			{
				file = "\rhsusf\addons\rhsusf_c_troops\scripts\rhs_nvghandler.sqf";
				description = "NVG equip";
			};
		};
	};
};
class UniformSlotInfo;
class CfgVehicles
{
	class B_AssaultPack_Base;
	class rhsusf_assault_eagleaiii_ucp: B_AssaultPack_Base
	{
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_assault_eagleaiii_ucp_ca.paa";
		dlc = "RHS_USAF";
		scope = 2;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		model = "\rhsusf\addons\rhsusf_infantry\gear\backpacks\rhsusf_eagleaIII";
		displayName = "Eagle A-III UCP";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\eagleaiii_ucp_co.paa"};
		maximumLoad = 240;
		mass = 35;
	};
	class rhsusf_assault_eagleaiii_ocp: rhsusf_assault_eagleaiii_ucp
	{
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_assault_eagleaiii_ocp_ca.paa";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\eagleaii_ocp_co.paa"};
	};
	class rhsusf_assault_eagleaiii_ocp_engineer: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (Engineer)";
		maximumLoad = 160;
		mass = 20;
		class TransportItems
		{
			class _xx_ToolKit
			{
				name = "ToolKit";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_engineer: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (Engineer)";
		class TransportItems
		{
			class _xx_ToolKit
			{
				name = "ToolKit";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_medic: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (Medic)";
		class TransportItems
		{
			class _xx_MediKit
			{
				name = "MediKit";
				count = 1;
			};
			class _xx_FirstAidKit
			{
				name = "FirstAidKit";
				count = 10;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_medic: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (Medic)";
		class TransportItems
		{
			class _xx_MediKit
			{
				name = "MediKit";
				count = 1;
			};
			class _xx_FirstAidKit
			{
				name = "FirstAidKit";
				count = 10;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_demo: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (Demolitions)";
		class TransportItems
		{
			class _xx_rhsusf_m112x4_mag
			{
				name = "rhsusf_m112x4_mag";
				count = 1;
			};
			class _xx_rhsusf_m112_mag
			{
				name = "rhsusf_m112_mag";
				count = 3;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_demo: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (Demolitions)";
		class TransportItems
		{
			class _xx_rhsusf_m112x4_mag
			{
				name = "rhsusf_m112x4_mag";
				count = 1;
			};
			class _xx_rhsusf_m112_mag
			{
				name = "rhsusf_m112_mag";
				count = 3;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_ar: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (AR)";
		class TransportItems
		{
			class _xx_rhsusf_100Rnd_556x45_soft_pouch
			{
				name = "rhsusf_100Rnd_556x45_soft_pouch";
				count = 4;
			};
			class _xx_rhsusf_200Rnd_556x45_mixed_soft_pouch_coyote
			{
				name = "rhsusf_200Rnd_556x45_mixed_soft_pouch_coyote";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_ar: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (AR)";
		class TransportItems
		{
			class _xx_rhsusf_100Rnd_556x45_soft_pouch
			{
				name = "rhsusf_100Rnd_556x45_soft_pouch";
				count = 4;
			};
			class _xx_rhsusf_200Rnd_556x45_mixed_soft_pouch_ucp
			{
				name = "rhsusf_200Rnd_556x45_mixed_soft_pouch_ucp";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_mg: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (MG)";
		class TransportItems
		{
			class _xx_rhsusf_50Rnd_762x51
			{
				name = "rhsusf_50Rnd_762x51";
				count = 8;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_mg: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (MG)";
		class TransportItems
		{
			class _xx_rhsusf_50Rnd_762x51
			{
				name = "rhsusf_50Rnd_762x51";
				count = 8;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_at: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OEF-CP (AT)";
		class TransportItems
		{
			class _xx_rhs_fgm148_magazine_AT
			{
				name = "rhs_fgm148_magazine_AT";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_at: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (AT)";
		class TransportItems
		{
			class _xx_rhs_fgm148_magazine_AT
			{
				name = "rhs_fgm148_magazine_AT";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ucp_maaws: rhsusf_assault_eagleaiii_ucp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III UCP (MAAWS)";
		class TransportItems
		{
			class _xx_rhs_mag_maaws_HEDP
			{
				name = "rhs_mag_maaws_HEDP";
				count = 2;
			};
		};
	};
	class rhsusf_assault_eagleaiii_ocp_maaws: rhsusf_assault_eagleaiii_ocp
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III OCP (MAAWS)";
		class TransportItems
		{
			class _xx_rhs_mag_maaws_HEDP
			{
				name = "rhs_mag_maaws_HEDP";
				count = 2;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy: rhsusf_assault_eagleaiii_ucp
	{
		author = "$STR_RHSUSF_AUTHOR_FULL";
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_assault_eagleaiii_coy_ca.paa";
		displayName = "Eagle A-III Coyote";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\eagleaii_coy_co.paa"};
	};
	class rhsusf_assault_eagleaiii_coy_engineer: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (Engineer)";
		class TransportItems
		{
			class _xx_ToolKit
			{
				name = "ToolKit";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_demo: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (Demolitions)";
		class TransportItems
		{
			class _xx_rhsusf_m112x4_mag
			{
				name = "rhsusf_m112x4_mag";
				count = 1;
			};
			class _xx_rhsusf_m112_mag
			{
				name = "rhsusf_m112_mag";
				count = 2;
			};
			class _xx_ClaymoreDirectionalMine_Remote_Mag
			{
				name = "ClaymoreDirectionalMine_Remote_Mag";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_eod: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		class TransportItems
		{
			class _xx_rhsusf_m112x4_mag
			{
				name = "rhsusf_m112x4_mag";
				count = 1;
			};
			class _xx_rhsusf_m112_mag
			{
				name = "rhsusf_m112_mag";
				count = 2;
			};
			class _xx_ToolKit
			{
				name = "ToolKit";
				count = 1;
			};
			class _xx_MineDetector
			{
				name = "MineDetector";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_m27: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (AR M27)";
		class TransportItems
		{
			class _xx_rhs_mag_30Rnd_556x45_M855_Stanag
			{
				name = "rhs_mag_30Rnd_556x45_M855_Stanag";
				count = 10;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_ar: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (AR M249)";
		class TransportItems
		{
			class _xx_rhsusf_100Rnd_556x45_M855_soft_pouch
			{
				name = "rhsusf_100Rnd_556x45_M855_soft_pouch";
				count = 4;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_mg: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (MG)";
		class TransportItems
		{
			class _xx_rhsusf_50Rnd_762x51
			{
				name = "rhsusf_50Rnd_762x51";
				count = 8;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_assaultman: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (Assaultman)";
		class TransportItems
		{
			class _xx_rhs_mag_smaw_HEDP
			{
				name = "rhs_mag_smaw_HEDP";
				count = 1;
			};
			class _xx_rhsusf_m112_mag
			{
				name = "rhsusf_m112_mag";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_at: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (AT)";
		class TransportItems
		{
			class _xx_rhs_fgm148_magazine_AT
			{
				name = "rhs_fgm148_magazine_AT";
				count = 1;
			};
		};
	};
	class rhsusf_assault_eagleaiii_coy_aa: rhsusf_assault_eagleaiii_coy
	{
		scope = 1;
		author = "$STR_RHSUSF_AUTHOR_FULL";
		displayName = "Eagle A-III Coyote (AA)";
		class TransportItems
		{
			class _xx_rhs_fim92_mag
			{
				name = "rhs_fim92_mag";
				count = 1;
			};
		};
	};
	class B_AssaultPack_rgr;
	class rhsusf_falconii: B_AssaultPack_rgr
	{
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_falconii_ca.paa";
		dlc = "RHS_USAF";
		author = "$STR_A3_Bohemia_Interactive";
		scope = 2;
		displayName = "Falcon-II RGR";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\falconii_rgr_co.paa"};
	};
	class rhsusf_falconii_coy: B_AssaultPack_rgr
	{
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_falconii_coy_ca.paa";
		dlc = "RHS_USAF";
		author = "$STR_A3_Bohemia_Interactive";
		scope = 2;
		displayName = "Falcon-II Coyote";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\falconii_coy_co.paa"};
	};
	class rhsusf_falconii_mc: B_AssaultPack_rgr
	{
		picture = "\rhsusf\addons\rhsusf_inventoryicons\data\backpacks\rhsusf_falconii_mc_ca.paa";
		dlc = "RHS_USAF";
		author = "$STR_A3_Bohemia_Interactive";
		scope = 2;
		displayName = "Falcon-II MC";
		hiddenSelectionsTextures[] = {"rhsusf\addons\rhsusf_infantry\gear\backpacks\data\falconii_mc_co.paa"};
	};
	class rhsusf_falconii_gr: rhsusf_falconii
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhs_mag_M433_HEDP
			{
				name = "rhs_mag_M433_HEDP";
				count = 10;
			};
			class _xx_rhs_mag_M583A1_white
			{
				name = "rhs_mag_M583A1_white";
				count = 5;
			};
		};
	};
	class rhsusf_falconii_coy_gr: rhsusf_falconii_coy
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhs_mag_M433_HEDP
			{
				name = "rhs_mag_M433_HEDP";
				count = 10;
			};
			class _xx_rhs_mag_M583A1_white
			{
				name = "rhs_mag_M583A1_white";
				count = 5;
			};
		};
	};
	class rhsusf_falconii_gr_m32: rhsusf_falconii_coy
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhsusf_mag_6Rnd_M433_HEDP
			{
				name = "rhsusf_mag_6Rnd_M433_HEDP";
				count = 6;
			};
			class _xx_rhsusf_mag_6Rnd_M714_white
			{
				name = "rhsusf_mag_6Rnd_M714_white";
				count = 2;
			};
		};
	};
	class rhsusf_falconii_breach: rhsusf_falconii
	{
		scope = 1;
		class TransportItems
		{
			class _xx_SatchelCharge_Remote_Mag
			{
				name = "SatchelCharge_Remote_Mag";
				count = 1;
			};
		};
	};
	class rhsusf_falconii_recon: rhsusf_falconii_coy
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhs_mag_M433_HEDP
			{
				name = "rhs_mag_M433_HEDP";
				count = 6;
			};
			class _xx_rhs_mag_M583A1_white
			{
				name = "rhs_mag_M583A1_white";
				count = 5;
			};
			class _xx_rhs_mag_mk84
			{
				name = "rhs_mag_mk84";
				count = 2;
			};
			class _xx_rhs_mag_m18_red
			{
				name = "rhs_mag_m18_red";
				count = 2;
			};
			class _xx_rhs_mag_m18_green
			{
				name = "rhs_mag_m18_green";
				count = 2;
			};
			class _xx_rhs_mag_m18_purple
			{
				name = "rhs_mag_m18_purple";
				count = 2;
			};
			class _xx_rhs_mag_m18_yellow
			{
				name = "rhs_mag_m18_yellow";
				count = 2;
			};
			class _xx_FirstAidKit
			{
				name = "FirstAidKit";
				count = 2;
			};
		};
	};
	class B_Carryall_cbr;
	class rhsusf_pack_slackman_m240: B_Carryall_cbr
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhsusf_50Rnd_762x51
			{
				name = "rhsusf_50Rnd_762x51";
				count = 6;
			};
		};
	};
	class rhsusf_pack_slackman_m249: B_Carryall_cbr
	{
		scope = 1;
		class TransportItems
		{
			class _xx_rhsusf_100Rnd_556x45_M855_soft_pouch
			{
				name = "rhsusf_100Rnd_556x45_M855_soft_pouch";
				count = 4;
			};
			class _xx_rhs_mag_30Rnd_556x45_M855_Stanag
			{
				name = "rhs_mag_30Rnd_556x45_M855_Stanag";
				count = 4;
			};
			class _xx_ClaymoreDirectionalMine_Remote_Mag
			{
				name = "ClaymoreDirectionalMine_Remote_Mag";
				count = 2;
			};
			class _xx_rhs_mag_mk84
			{
				name = "rhs_mag_mk84";
				count = 4;
			};
			class _xx_Chemlight_red
			{
				name = "Chemlight_red";
				count = 2;
			};
			class _xx_Chemlight_green
			{
				name = "Chemlight_green";
				count = 2;
			};
			class _xx_rhs_mag_m18_red
			{
				name = "rhs_mag_m18_red";
				count = 2;
			};
			class _xx_rhs_mag_m18_green
			{
				name = "rhs_mag_m18_green";
				count = 2;
			};
			class _xx_rhs_mag_m18_purple
			{
				name = "rhs_mag_m18_purple";
				count = 2;
			};
			class _xx_rhs_mag_m18_yellow
			{
				name = "rhs_mag_m18_yellow";
				count = 2;
			};
			class _xx_FirstAidKit
			{
				name = "FirstAidKit";
				count = 2;
			};
		};
	};
	class B_Parachute;
	class Steerable_Parachute_F;
	class rhsusf_eject_Parachute: Steerable_Parachute_F
	{
		scope = 1;
		scopeArsenal = 0;
		side = 0;
		vehicleClass = "Air";
		maxgravity = -10;
		maxliftduration = 4;
		maxliftthrust = -40;
		maxrotationx = 0.1;
		maxrotationz = 0.4;
		maxsensitivityhorizontal = 0.05;
		maxsensitivityvertical = 0.01;
		mingravity = -2;
		minliftduration = 4;
		minliftthrust = -40;
		minrotationx = -0.1;
		minrotationz = -0.4;
		normalgravity = -5;
		thrustaccel = 0.001;
		thrustdeccel = 0.001;
		thrustnormal = 0.001;
		turnforcescale = 0.0001;
		displayName = "Static Parachute";
		driverAction = "RHS_StaticPara_Pilot";
		model = "\rhsusf\addons\rhsusf_a2port_air\parachute\para.p3d";
	};
	class rhsusf_eject_Parachute_backpack: B_Parachute
	{
		scope = 2;
		scopeArsenal = 2;
		displayName = "Static Parachute Bag";
		ParachuteClass = "rhsusf_eject_Parachute";
		maximumLoad = 0;
		mass = 130;
	};
	class Item_Base_F;
	class Item_rhs_uniform_g3_m81: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "G3 Uniform (M81)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_g3_m81
			{
				name = "rhs_uniform_g3_m81";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_g3_aor2: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "G3 Uniform (AOR2)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_g3_aor2
			{
				name = "rhs_uniform_g3_aor2";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_g3_tan: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "G3 Uniform (Tan)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_g3_tan
			{
				name = "rhs_uniform_g3_tan";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_g3_rgr: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "G3 Uniform (Ranger Green)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_g3_rgr
			{
				name = "rhs_uniform_g3_rgr";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_g3_blk: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "G3 Uniform (Black)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_g3_blk
			{
				name = "rhs_uniform_g3_blk";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform OCP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ocp
			{
				name = "rhs_uniform_cu_ocp";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform UCP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ucp
			{
				name = "rhs_uniform_cu_ucp";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ocp_1stcav: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform OCP (1st Cavalry Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ocp_1stcav
			{
				name = "rhs_uniform_cu_ocp_1stcav";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ucp_1stcav: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform UCP (1st Cavalry Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ucp_1stcav
			{
				name = "rhs_uniform_cu_ucp_1stcav";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ocp_82nd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform OCP (82nd Airborne Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ocp_82nd
			{
				name = "rhs_uniform_cu_ocp_82nd";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ucp_82nd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform UCP (82nd Airborne Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ucp_82nd
			{
				name = "rhs_uniform_cu_ucp_82nd";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ocp_101st: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform OCP (101st Airborne Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ocp_101st
			{
				name = "rhs_uniform_cu_ocp_101st";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ucp_101st: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform UCP (101st Airborne Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ucp_101st
			{
				name = "rhs_uniform_cu_ucp_101st";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ocp_10th: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform OCP (10th Mountain Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ocp_10th
			{
				name = "rhs_uniform_cu_ocp_10th";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_cu_ucp_10th: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Combat Uniform UCP (10th Mountain Div.)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_cu_ucp_10th
			{
				name = "rhs_uniform_cu_ucp_10th";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_FROG01_d: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "FROG MARPAT-D";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_FROG01_d
			{
				name = "rhs_uniform_FROG01_d";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_FROG01_wd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "FROG MARPAT-WD";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_FROG01_wd
			{
				name = "rhs_uniform_FROG01_wd";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_acu_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Army Combat Uniform (UCP)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_acu_ucp
			{
				name = "rhs_uniform_acu_ucp";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_acu_ucp2: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Army Combat Uniform IR Flag (UCP)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_acu_ucp2
			{
				name = "rhs_uniform_acu_ucp2";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_acu_ucpd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Army Combat Uniform (UCP-D)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_acu_ucpd
			{
				name = "rhs_uniform_acu_ucpd";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_acu_oefcp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Army Combat Uniform (OEF-CP)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_acu_oefcp
			{
				name = "rhs_uniform_acu_oefcp";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_acu_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Army Combat Uniform (OCP)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_acu_ocp
			{
				name = "rhs_uniform_acu_ocp";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_abu: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Airman Battle Uniform";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_abu
			{
				name = "rhs_uniform_abu";
				count = 1;
			};
		};
	};
	class Item_rhs_uniform_bdu_erdl: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Battle Dress Uniform (ERDL)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Uniforms";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_uniform_bdu_erdl
			{
				name = "rhs_uniform_bdu_erdl";
				count = 1;
			};
		};
	};
	class Item_rhsusf_patrolcap_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Patrol Cap OEF-CP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_patrolcap_ocp
			{
				name = "rhsusf_patrolcap_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_patrolcap_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Patrol Cap UCP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_patrolcap_ucp
			{
				name = "rhsusf_patrolcap_ucp";
				count = 1;
			};
		};
	};
	class Item_rhs_xmas_antlers: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Christmas Antlers";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_xmas_antlers
			{
				name = "rhs_xmas_antlers";
				count = 1;
			};
		};
	};
	class Item_rhs_Booniehat_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Booniehat OEF-CP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_Booniehat_ocp
			{
				name = "rhs_Booniehat_ocp";
				count = 1;
			};
		};
	};
	class Item_rhs_Booniehat_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Booniehat UCP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_Booniehat_ucp
			{
				name = "rhs_Booniehat_ucp";
				count = 1;
			};
		};
	};
	class Item_rhs_Booniehat_m81: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Booniehat M81";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_Booniehat_m81
			{
				name = "rhs_Booniehat_m81";
				count = 1;
			};
		};
	};
	class Item_rhs_booniehat2_marpatd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Booniehat MARPAT-D";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_booniehat2_marpatd
			{
				name = "rhs_booniehat2_marpatd";
				count = 1;
			};
		};
	};
	class Item_rhs_booniehat2_marpatwd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Booniehat MARPAT-WD";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_booniehat2_marpatwd
			{
				name = "rhs_booniehat2_marpatwd";
				count = 1;
			};
		};
	};
	class Item_rhs_8point_marpatd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Utility Cap MARPAT-D";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_8point_marpatd
			{
				name = "rhs_8point_marpatd";
				count = 1;
			};
		};
	};
	class Item_rhs_8point_marpatwd: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "Utility Cap MARPAT-WD";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhs_8point_marpatwd
			{
				name = "rhs_8point_marpatwd";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ocp
			{
				name = "rhsusf_ach_helmet_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ocp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ocp_alt
			{
				name = "rhsusf_ach_helmet_ocp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ocp_norotos: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Norotos)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ocp_norotos
			{
				name = "rhsusf_ach_helmet_ocp_norotos";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ucp
			{
				name = "rhsusf_ach_helmet_ucp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ucp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ucp_alt
			{
				name = "rhsusf_ach_helmet_ucp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ucp_norotos: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Norotos)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ucp_norotos
			{
				name = "rhsusf_ach_helmet_ucp_norotos";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_M81: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH M81";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_M81
			{
				name = "rhsusf_ach_helmet_M81";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_DCU: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH DCU";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_DCU
			{
				name = "rhsusf_ach_helmet_DCU";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_DCU_early: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH DCU (Early)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_DCU_early
			{
				name = "rhsusf_ach_helmet_DCU_early";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_DCU_early_rhino: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH DCU (Early/Rhino)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_DCU_early_rhino
			{
				name = "rhsusf_ach_helmet_DCU_early_rhino";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_camo_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Netting)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_camo_ocp
			{
				name = "rhsusf_ach_helmet_camo_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ocp
			{
				name = "rhsusf_ach_helmet_headset_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ocp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Headset/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ocp_alt
			{
				name = "rhsusf_ach_helmet_headset_ocp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ucp
			{
				name = "rhsusf_ach_helmet_headset_ucp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ucp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Headset/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ucp_alt
			{
				name = "rhsusf_ach_helmet_headset_ucp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ESS_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ESS_ocp
			{
				name = "rhsusf_ach_helmet_ESS_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ESS_ocp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (ESS/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ESS_ocp_alt
			{
				name = "rhsusf_ach_helmet_ESS_ocp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ESS_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ESS_ucp
			{
				name = "rhsusf_ach_helmet_ESS_ucp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_ESS_ucp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (ESS/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_ESS_ucp_alt
			{
				name = "rhsusf_ach_helmet_ESS_ucp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ess_ocp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ess_ocp
			{
				name = "rhsusf_ach_helmet_headset_ess_ocp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ess_ocp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH OEF-CP (Headset/ESS/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ess_ocp_alt
			{
				name = "rhsusf_ach_helmet_headset_ess_ocp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ess_ucp: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ess_ucp
			{
				name = "rhsusf_ach_helmet_headset_ess_ucp";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_helmet_headset_ess_ucp_alt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH UCP (Headset/ESS/Alt)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_helmet_headset_ess_ucp_alt
			{
				name = "rhsusf_ach_helmet_headset_ess_ucp_alt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare
			{
				name = "rhsusf_ach_bare";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_ess
			{
				name = "rhsusf_ach_bare_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_headset: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_headset
			{
				name = "rhsusf_ach_bare_headset";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_headset_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_headset_ess
			{
				name = "rhsusf_ach_bare_headset_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_tan: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Tan)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_tan
			{
				name = "rhsusf_ach_bare_tan";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_tan_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Tan/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_tan_ess
			{
				name = "rhsusf_ach_bare_tan_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_tan_headset: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Tan/Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_tan_headset
			{
				name = "rhsusf_ach_bare_tan_headset";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_tan_headset_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Tan/Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_tan_headset_ess
			{
				name = "rhsusf_ach_bare_tan_headset_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_wood: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Woodland)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_wood
			{
				name = "rhsusf_ach_bare_wood";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_wood_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Woodland/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_wood_ess
			{
				name = "rhsusf_ach_bare_wood_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_wood_headset: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Woodland/Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_wood_headset
			{
				name = "rhsusf_ach_bare_wood_headset";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_wood_headset_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Woodland/Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_wood_headset_ess
			{
				name = "rhsusf_ach_bare_wood_headset_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_des: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Desert)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_des
			{
				name = "rhsusf_ach_bare_des";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_des_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Desert/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_des_ess
			{
				name = "rhsusf_ach_bare_des_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_des_headset: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Desert/Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_des_headset
			{
				name = "rhsusf_ach_bare_des_headset";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_des_headset_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Desert/Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_des_headset_ess
			{
				name = "rhsusf_ach_bare_des_headset_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_semi: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Semi-Arid)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_semi
			{
				name = "rhsusf_ach_bare_semi";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_semi_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Semi-Arid/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_semi_ess
			{
				name = "rhsusf_ach_bare_semi_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_semi_headset: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Semi-Arid/Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_semi_headset
			{
				name = "rhsusf_ach_bare_semi_headset";
				count = 1;
			};
		};
	};
	class Item_rhsusf_ach_bare_semi_headset_ess: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "ACH (Semi-Arid/Headset/ESS)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_ach_bare_semi_headset_ess
			{
				name = "rhsusf_ach_bare_semi_headset_ess";
				count = 1;
			};
		};
	};
	class Item_rhsusf_opscore_fg: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "FAST Ballistic (Foliage Green)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_opscore_fg
			{
				name = "rhsusf_opscore_fg";
				count = 1;
			};
		};
	};
	class Item_rhsusf_opscore_fg_pelt: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "FAST Ballistic (Foliage Green/Headset)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_opscore_fg_pelt
			{
				name = "rhsusf_opscore_fg_pelt";
				count = 1;
			};
		};
	};
	class Item_rhsusf_opscore_fg_pelt_nsw: Item_Base_F
	{
		scope = 2;
		scopeCurator = 2;
		displayName = "FAST Ballistic (Foliage Green/Headset/NSW)";
		author = "$STR_RHSUSF_AUTHOR_FULL";
		vehicleClass = "ItemsHeadgear";
		editorCategory = "EdCat_Equipment";
		editorSubcategory = "EdSubcat_Helmets";
		model = "\A3\Weapons_f\dummyweapon.p3d";
		class TransportItems
		{
			class rhsusf_opscore_fg_pelt_nsw
			{
				name = "rhsusf_opscore_fg_pelt_nsw";
				count = 1;
			};
		};
	};
};

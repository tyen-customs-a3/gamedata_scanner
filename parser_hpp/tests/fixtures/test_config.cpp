class CfgPatches 
{
	class bw_gear
	{
		units[]={};
		weapons[]={};
		requiredVersion=0.1;
		requiredAddons[]=
		{
			"A3_Characters_F",
			"A3_Characters_F_Beta",
			"A3_Characters_F_Enoch",
			"A3_Weapons_F_Ammoboxes",
			"rhsgref_c_troops",
			"rhssaf_c_gear"
		};
	};
};

class CfgVehicles 
{
	class I_Soldier_base_F;
	class I_Soldier_02_F;
	class I_E_Uniform_01_F;
	class I_E_Uniform_01_shortsleeve_F;
	
	class bw_combat_fleck: I_Soldier_base_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Flecktarn)";
		uniformClass="bw_uniform_combat_fleck";
		hiddenSelections[] = 
		{
			"Camo",
			"insignia"
		};
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\massif_fleck_co.paa"
		};
	};
	class bw_combat_rs_fleck: I_Soldier_02_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Flecktarn, Rolled Sleeves)";
		uniformClass="bw_uniform_combat_rs_fleck";
		hiddenSelections[] = 
		{
			"Camo",
			"insignia"
		};
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\massif_fleck_co.paa"
		};
	};
	class bw_jacket_fleck: I_E_Uniform_01_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Flecktarn, Jacket)";
		uniformClass="bw_uniform_jacket_fleck";
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\jacket_fleck_co.paa",
			"\bw_gear\data\massif_fleck_co.paa",
			"\a3\characters_f_enoch\uniforms\data\i_e_soldier_01_gloves_black_co.paa"
		};
	};
	class bw_jacket_rs_fleck: I_E_Uniform_01_shortsleeve_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Flecktarn, Jacket, Rolled Sleeves)";
		uniformClass="bw_uniform_jacket_rs_fleck";
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\jacket_fleck_co.paa",
			"\bw_gear\data\massif_fleck_co.paa"
		};
	};
	class bw_combat_tropen: I_Soldier_base_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Tropentarn)";
		uniformClass="bw_uniform_combat_tropen";
		hiddenSelections[] = 
		{
			"Camo",
			"insignia"
		};
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\massif_tropen_co.paa"
		};
	};
	class bw_combat_rs_tropen: I_Soldier_02_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Tropentarn, Rolled Sleeves)";
		uniformClass="bw_uniform_combat_rs_tropen";
		hiddenSelections[] = 
		{
			"Camo",
			"insignia"
		};
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\massif_tropen_co.paa"
		};
	};
	class bw_jacket_tropen: I_E_Uniform_01_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Tropentarn, Jacket)";
		uniformClass="bw_uniform_jacket_tropen";
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\jacket_tropen_co.paa",
			"\bw_gear\data\massif_tropen_co.paa",
			"\a3\characters_f_enoch\uniforms\data\i_e_soldier_01_gloves_black_co.paa"
		};
	};
	class bw_jacket_rs_tropen: I_E_Uniform_01_shortsleeve_F 
	{
		author="BW";
		scope=1;
		displayName="Massif Combat Uniform (Tropentarn, Jacket, Rolled Sleeves)";
		uniformClass="bw_uniform_jacket_rs_tropen";
		hiddenSelectionsTextures[] = 
		{
			"\bw_gear\data\jacket_tropen_co.paa",
			"\bw_gear\data\massif_tropen_co.paa"
		};
	};
	
	class B_AssaultPack_rgr;
	class bw_assaultpack_fleck: B_AssaultPack_rgr
	{
		author="BW";
		scope=2;
		displayName="Assault Pack (Flecktarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\assaultpack_fleck_co.paa"
		};
	};
	class bw_assaultpack_tropen: B_AssaultPack_rgr
	{
		author="BW";
		scope=2;
		displayName="Assault Pack (Tropentarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\assaultpack_tropen_co.paa"
		};
	};
	
	class B_Kitbag_rgr;
	class bw_kitbag_fleck: B_Kitbag_rgr
	{
		author="BW";
		scope=2;
		displayName="Kitbag (Flecktarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\kitbag_fleck_co.paa"
		};
	};
	class bw_kitbag_tropen: B_Kitbag_rgr
	{
		author="BW";
		scope=2;
		displayName="Kitbag (Tropentarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\kitbag_tropen_co.paa"
		};
	};
	
	class B_Carryall_oli;
	class bw_carryall_fleck: B_Carryall_oli
	{
		author="BW";
		scope=2;
		displayName="Carryall (Flecktarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\carryall_fleck_co.paa"
		};
	};
	class bw_carryall_tropen: B_Carryall_oli
	{
		author="BW";
		scope=2;
		displayName="Carryall (Tropentarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\carryall_tropen_co.paa"
		};
	};
};

class CfgWeapons 
{
	class UniformItem;
	class Uniform_Base;
	
	class bw_uniform_combat_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn)";
		picture="\bw_gear\data\ui\icon_massif_fleck_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelections[]=
		{
			"Camo"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\massif_fleck_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_fleck";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_combat_rs_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn, Rolled Sleeves)";
		picture="\bw_gear\data\ui\icon_massif_fleck_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelections[]=
		{
			"Camo"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\massif_fleck_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_rs_fleck";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_jacket_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn, Jacket)";
		picture="\bw_gear\data\ui\icon_massif_fleck_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\jacket_fleck_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_jacket_fleck";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_jacket_rs_fleck: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Flecktarn, Jacket, Rolled Sleeves)";
		picture="\bw_gear\data\ui\icon_massif_fleck_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\jacket_fleck_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_jacket_rs_fleck";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_combat_tropen: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Tropentarn)";
		picture="\bw_gear\data\ui\icon_massif_tropen_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelections[]=
		{
			"Camo"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\massif_tropen_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_tropen";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_combat_rs_tropen: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Tropentarn, Rolled Sleeves)";
		picture="\bw_gear\data\ui\icon_massif_tropen_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelections[]=
		{
			"Camo"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\massif_tropen_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_combat_rs_tropen";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_jacket_tropen: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Tropentarn, Jacket)";
		picture="\bw_gear\data\ui\icon_massif_tropen_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\jacket_tropen_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_jacket_tropen";
			containerClass="Supply60";
			mass=40;
		};
	};
	class bw_uniform_jacket_rs_tropen: Uniform_Base
	{
		author="BW";
		scope=2;
		displayName="Massif Combat Uniform (Tropentarn, Jacket, Rolled Sleeves)";
		picture="\bw_gear\data\ui\icon_massif_tropen_ca.paa";
		model="\a3\characters_f\common\suitpacks\suitpack_universal_F.p3d";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\jacket_tropen_co.paa"
		};
		class ItemInfo: UniformItem 
		{
			uniformModel="-";
			uniformClass="bw_jacket_rs_tropen";
			containerClass="Supply60";
			mass=40;
		};
	};
	
	class rhsgref_helmet_pasgt_erdl;
	class rhsgref_helmet_pasgt_erdl_rhino;
	class bw_pasgt_fleck: rhsgref_helmet_pasgt_erdl
	{
		author="BW";
		scope=2;
		displayName="PASGT (Flecktarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_fleck_co.paa"
		};
	};
	class bw_pasgt_fleck_rhino: rhsgref_helmet_pasgt_erdl_rhino
	{
		author="BW";
		scope=2;
		displayName="PASGT (Flecktarn, Rhino)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_fleck_co.paa"
		};
	};
	class bw_pasgt_tropen: rhsgref_helmet_pasgt_erdl
	{
		author="BW";
		scope=2;
		displayName="PASGT (Tropentarn)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_tropen_co.paa"
		};
	};
	class bw_pasgt_tropen_rhino: rhsgref_helmet_pasgt_erdl_rhino
	{
		author="BW";
		scope=2;
		displayName="PASGT (Tropentarn, Rhino)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_tropen_co.paa"
		};
	};
	
	class rhssaf_helmet_m97_woodland_black_ess;
	class rhssaf_helmet_m97_woodland_black_ess_bare;
	class bw_pasgt_fleck_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		author="BW";
		scope=2;
		displayName="PASGT (Flecktarn, ESS)";
		hiddenSelections[]=
		{
			"camo",
			"camo2"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_fleck_co.paa",
			"\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"
		};
	};
	class bw_pasgt_fleck_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		author="BW";
		scope=2;
		displayName="PASGT (Flecktarn, ESS Bare)";
		hiddenSelections[]=
		{
			"camo",
			"camo2"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_fleck_co.paa",
			"\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"
		};
	};
	class bw_pasgt_tropen_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		author="BW";
		scope=2;
		displayName="PASGT (Tropentarn, ESS)";
		hiddenSelections[]=
		{
			"camo",
			"camo2"
		};
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_tropen_co.paa",
			"\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"
		};
	};
	class bw_pasgt_tropen_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		author="BW";
		scope=2;
		displayName="PASGT (Tropentarn, ESS Bare)";
		hiddenSelectionsTextures[]=
		{
			"\bw_gear\data\pasgt_tropen_co.paa",
			"\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"
		};
	};
};
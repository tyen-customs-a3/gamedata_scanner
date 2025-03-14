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
};
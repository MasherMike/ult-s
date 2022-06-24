use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
static mut LIGHTSPEED :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.5, z: 0.0 };
static mut LIGHTSPEED_ROT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 180.0, z: 0.0 };
use smash::phx::Vector2f;

#[acmd_script(
    agent = "sonic",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT)]
unsafe fn sonic_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 0, 13, 2.5, 95, -28.5, 131, 1.05, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 0, 17.5, 0.5, 80, 24, 180, 0.95, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=22)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3",
    category = ACMD_EFFECT)]
unsafe fn sonic_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 1, -3, -22, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_throwf",
    category = ACMD_EFFECT)]
unsafe fn sonic_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc_d"), hash40("top"), -2, 13, 4, 6, -70, 80, 0.9, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_FLIP(hash40("sys_smash_flash_s"), hash40("sys_smash_flash_s"), hash40("throw"), 3, 2, 3, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3lw",
    category = ACMD_EFFECT)]
unsafe fn sonic_ftiltlw_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 3.5, -4, -5, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 6, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3hi",
    category = ACMD_EFFECT)]
unsafe fn sonic_ftilthi_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 0, -3, -42, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 13.2, 12.2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT)]
unsafe fn sonic_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), -2, 2, 2, 1, 5, 5, 0.925, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(1.5)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackairb",
    category = ACMD_EFFECT)]
unsafe fn sonic_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), 0, 7.5, -6.5, 160, 60, 15, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=16)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), true, true)
		}
    });
}			
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT)]
unsafe fn sonic_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 7.5, 2.5, -10, 0, 0, 0.5, true)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), -1, 7.5, -2.5, 190, 0, 0, 0.5, true)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sonic_atk_hi"), hash40("sonic_atk_hi"), hash40("top"), 0.8, 8.7, 0, 59, 74, 154, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sonic_atk_hi"), hash40("sonic_atk_hi"), hash40("top"), -0.9, 8.7, 0, 245, -77, 22, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=14)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 19.5, 0, 0, 0, 0, 1.55, 0, 0, 0, 0, 0, 360, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackdash",
    category = ACMD_EFFECT)]
unsafe fn sonic_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 1, 7, -9, -7, 0, 0, 1.2, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=5)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 14, 9.5, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, false)
		}
		frame(Frame=22)
		for(3 Iterations){
			if(is_excute){
				FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
			wait(Frames=5)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attack13",
    category = ACMD_EFFECT)]
unsafe fn sonic_jab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), -1.7, 6.5, -2, -10, 7, 0, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=4)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 13, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT)]
unsafe fn sonic_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=11)
		if(is_excute){
			EFFECT(hash40("sys_attack_line"), hash40("top"), 0, 2.5, -2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
			EFFECT(hash40("sys_attack_line"), hash40("top"), -1, 2.5, 4, 180, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=12)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_RATE(1.2)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 2.5, 17, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 2.5, -15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 2.5, -2, 0, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 2.5, 4, 0, 180, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
    });
}		

#[acmd_script(
    agent = "sonic",
    script =  "game_attackairb",
    category = ACMD_GAME)]
unsafe fn sonic_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.8)
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=6.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=76, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=76, FKB=0, BKB=40, Size=5.0, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=33)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attackairhi",
    category = ACMD_GAME)]
unsafe fn sonic_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=110, KBG=50, FKB=90, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=110, KBG=50, FKB=90, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=123, KBG=50, FKB=120, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=123, KBG=50, FKB=120, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=7.0, X=0.0, Y=20.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=6.0, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=4.5, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=4.8, X=0.0, Y=10.0, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn sonic_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=90, KBG=100, FKB=0, BKB=50, Size=3.4, X=7.5, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			JostleModule::set_status(true)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "game_attackhi3",
    category = ACMD_GAME)]
unsafe fn sonic_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=96, KBG=100, FKB=100, BKB=0, Size=5.2, X=0.0, Y=8.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=95, KBG=100, FKB=35, BKB=0, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=95, KBG=100, FKB=12, BKB=0, Size=5.0, X=7.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=5.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=7.0, X=7.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			CancelModule::enable_cancel()
		}
		frame(Frame=28)
		FT_MOTION_RATE(FSM=0.6)
		frame(Frame=38)
		FT_MOTION_RATE(FSM=1)
    });
}		
#[fighter_frame_callback]
pub fn sonic(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_SONIC {
			if [hash40("special_s_start")].contains(&MotionModule::motion_kind(boma)) {
				WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
				WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
				WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					MotionModule::set_rate(boma, 0.6075);
				} else {
					MotionModule::set_rate(boma, 0.43875);
				};
				if MotionModule::frame(boma) >= 4.65 && MotionModule::frame(boma) < 6.0{
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 20.0*PostureModule::lr(boma), y: 0.0}, false) == 0 {
							let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+(20.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma), z: 0.0 };
							PostureModule::set_pos(boma, &pos);
							PostureModule::init_pos(boma, &pos, true, true);
						};
					} else {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 10.0*PostureModule::lr(boma), y: 0.0}, false) == 0 {
							let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+(10.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma), z: 0.0 };
							PostureModule::set_pos(boma, &pos);
							PostureModule::init_pos(boma, &pos, true, true);
						};
					};
				} else {
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				if MotionModule::frame(boma) >= 4.909086446285048 && MotionModule::frame(boma) < 5.72726752066589 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
						let speed = smash::phx::Vector3f { x: 35.0, y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 25.0, y: 1.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
				};
				if MotionModule::frame(boma) >= 6.0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						if PostureModule::lr(boma) == 1.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_r"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_l"), 0.0, 1.0, false, 0.0, false, false);
						};
					} else {
						if PostureModule::lr(boma) == 1.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_r"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_l"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					let speed = smash::phx::Vector3f { x: 5.0, y: 0.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);
					let lightspeed: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("top"), &LIGHTSPEED, &LIGHTSPEED_ROT, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, lightspeed, 0.2, 0.4, 10.0);
					EffectModule::set_rate(boma, lightspeed, 0.25);
					acmd!(lua_state, {
						STOP_SE(hash40("se_sonic_smash_h01"))
						PLAY_SE(hash40("vc_sonic_attack05"))
						PLAY_SE(hash40("se_sonic_swing_m"))
						PLAY_SE(hash40("se_sonic_swing_l"))
						PLAY_SE(hash40("se_sonic_attackair_l01"))
					});
				};
			} else {
				VisibilityModule::set_whole(boma, true);
			};
			if [hash40("run_brake_r"), hash40("run_brake_l")].contains(&MotionModule::motion_kind(boma)) && status_kind != *FIGHTER_STATUS_KIND_RUN_BRAKE && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND{
				MotionModule::set_rate(boma, 0.5);
				if MotionModule::frame(boma) >= 23.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				} else {
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					};
				};
			};
			if [hash40("run_brake_r"), hash40("run_brake_l")].contains(&MotionModule::motion_kind(boma)) && StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
				if MotionModule::frame(boma) >= 26.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
				};
			};
			if [*FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP].contains(&status_kind) || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && [hash40("special_s_start"), hash40("run_brake_r"), hash40("run_brake_l")].contains(&MotionModule::motion_kind(boma)) == false ){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
	};
}

#[acmd_script(
    agent = "sonic",
    script =  "expression_specialshold",
    category = ACMD_EXPRESSION)]
unsafe fn sonic_lightspeed_ball_fix(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		rust {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "expression_specialsholdjump",
    category = ACMD_EXPRESSION)]
unsafe fn sonic_lightspeed_ball_fix2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		rust {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_specialsstart",
    category = ACMD_EFFECT)]
unsafe fn sonic_lightspeed_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "sound_specialsstart",
    category = ACMD_SOUND)]
unsafe fn sonic_lightspeed_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("se_sonic_smash_h01"))
		}
    });
}	
		
pub fn install() {
    smashline::install_acmd_scripts!(
		sonic_bair,
		sonic_uair,
		sonic_dtilt,
		sonic_utilt,
		sonic_lightspeed_dash,
		sonic_lightspeed_dash_sound,
		sonic_lightspeed_ball_fix,
		sonic_lightspeed_ball_fix2,
		sonic_bair_eff,
		sonic_da_eff,
		sonic_dsmash_eff,
		sonic_dtilt_eff,
		sonic_jab_eff,
		sonic_uair_eff,
		sonic_utilt_eff,
		sonic_ftilt_eff,
		sonic_ftilthi_eff,
		sonic_ftiltlw_eff,
		sonic_fthrow_eff
    );
	smashline::install_agent_frame_callbacks!(sonic);
}
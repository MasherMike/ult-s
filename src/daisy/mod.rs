use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use crate::util::*;
static mut STATIC_MUT : [i32; 8] = [6; 8];

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT && KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
			KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
		};
    }
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_DAISY_SPECIAL_N_HIT && KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
			KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
		};
    }
}
#[acmd_script(
    agent = "daisy",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 0, 8, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=30, KBG=40, FKB=0, BKB=50, Size=3.8, X=0.0, Y=2.5, Z=7.5, X2=0.0, Y2=2.5, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=0.5)
		wait(Frames=6)
		FT_MOTION_RATE(FSM=1)
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			EFFECT_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 0, 4.0, 4.5, 10, -35, 8, 0.8, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ, 0.3)
			LAST_EFFECT_SET_RATE(1.4)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=4.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=6.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=4.0, X=0.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=41)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("daisy_smash_racket"), hash40("daisy_smash_racket_l"), hash40("top"), 4.5, 5.5, 0, 0, 180, 0, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(0.5)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "sound_attackairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn daisy_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_daisy_rnd_attack"))
			PLAY_SE(hash40("se_daisy_attackhard_s01"))
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "expression_attackairhi",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn daisy_uair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			VisibilityModule::set_int64(hash40("smash_item") as i64, hash40("smash_item_racket") as i64)
			ItemModule::set_have_item_visibility(false, 0)
		}
		frame(Frame=27)
		if(is_excute){
			VisibilityModule::set_int64(hash40("smash_item") as i64, hash40("smash_item_none") as i64)
			ItemModule::set_have_item_visibility(true, 0)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=55, FKB=0, BKB=72, Size=5.0, X=0.0, Y=8.0, Z=11.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=55, FKB=0, BKB=72, Size=4.5, X=0.0, Y=8.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 2, 7.5, 2, 11, 77, 16, 0.6, true, EF_FLIP_YZ, 0.4)
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), -2, 9, 2, 140, 120, -50, 0.6, true, EF_FLIP_YZ, 0.4)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=13.0, Angle=65, KBG=72, FKB=0, BKB=50, Size=3.4, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=65, KBG=72, FKB=0, BKB=50, Size=4.4, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=21)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=23)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=4.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=4.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=4.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=10.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=10.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=10.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=0, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialhistart", "game_specialairhistart"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			if(is_excute){
				ArticleModule::generate_article(FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0)
				ArticleModule::change_motion(FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR,smash::phx::Hash40::new("special_hi_start"),false,0.0)
			}
			frame(Frame=6)
			if(is_excute){
				WorkModule::on_flag(Flag=FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS)
			}
			frame(Frame=7)
			if(is_excute){
				sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
				AttackModule::set_attack_reference_joint_id(Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y))
        		ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=70, Size=5.0, X=0.0, Y=5.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PARASOL)
				ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PARASOL)
			}
			frame(Frame=11)
			if(is_excute){
				AttackModule::set_attack_reference_joint_id(Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y))
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=80, KBG=100, FKB=0, BKB=10, Size=4.0, X=0.0, Y=10.0, Z=8.0, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PARASOL)
				ATTACK(ID=1, Part=0, Bone=hash40("havel"), Damage=7.0, Angle=80, KBG=100, FKB=0, BKB=10, Size=1.5, X=2.0, Y=5.0, Z=3.5, X2=2.0, Y2=2.5, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PARASOL)
				ATTACK(ID=2, Part=0, Bone=hash40("havel"), Damage=7.0, Angle=80, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=0.0, Y2=-1.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PARASOL)
			}
			frame(Frame=33)
			if(is_excute){
				AttackModule::clear_all()
			}			
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attack11",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_jab1_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			frame(Frame=2)
			if(is_excute){
				EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, 9.0, 12, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 360, true, 0.4)
			}	
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attack11",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=270, KBG=25, FKB=0, BKB=32, Size=2.0, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=270, KBG=15, FKB=0, BKB=24, Size=2.0, X=0.0, Y=9.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=270, KBG=15, FKB=0, BKB=24, Size=2.4, X=0.0, Y=9.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=270, KBG=25, FKB=0, BKB=32, Size=2.0, X=0.0, Y=9.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=270, KBG=15, FKB=0, BKB=24, Size=2.4, X=0.0, Y=9.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=10)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attack12",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=50, KBG=100, FKB=0, BKB=40, Size=3.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=15.0, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=50, KBG=100, FKB=0, BKB=40, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=8)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attack12",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_jab2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc_c"), hash40("sys_attack_arc_c"), hash40("top"), 0, 11.5, 3, 0, 9, 90, 0.8, true, EF_FLIP_YZ, 0.3)
			LAST_EFFECT_SET_RATE(1.5)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_throwf",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=9.0, Angle=45, KBG=83, FKB=0, BKB=48, Hitlag=1.2, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=10)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwf",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			FOOT_EFFECT(hash40("null"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			EFFECT_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 0, 10, 7.5, 10, -35, 8, 0.8, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ, 0.3)
			LAST_EFFECT_SET_RATE(1.4)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_throwlw",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=70, KBG=60, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=34)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=20, BKB=0, Size=3.8, X=0.0, Y=4.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
			AttackModule::set_catch_only_all(true, false)
			CHECK_FINISH_CAMERA(1, 1)
		}
		frame(Frame=36)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=37)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwlw",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_dthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=34)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwhi",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_uthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=18)
		if(is_excute){
			EFFECT_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 17, 0, 90, 0, 0, 1, 0, 1, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
		frame(Frame=26)
		if(is_excute){
			EFFECT_FLIP(hash40("sys_smash_flash_s"), hash40("sys_smash_flash_s"), hash40("top"), 0, 27, 0, 0, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
		frame(Frame=40)
		if(is_excute){
			LANDING_EFFECT_FLIP(hash40("sys_landing_smoke_s"), hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwb",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_bthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=18)
		if(is_excute){
			EFFECT(hash40("peach_smash_heart"), hash40("top"), 0, 10, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=19)
		if(is_excute){
			LANDING_EFFECT_FLIP(hash40("sys_atk_smoke"), hash40("sys_atk_smoke"), hash40("top"), -1, 0, -2, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
		}
		frame(Frame=27)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash_s"), hash40("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_catchattack",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_pummel_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			FOOT_EFFECT_FLIP(hash40("sys_run_smoke"), hash40("sys_run_smoke"), hash40("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
		}
		frame(Frame=1)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 11, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_catchpull", "game_catchwait"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_remove_toad(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_catchcut"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_catch_release_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
		if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_SPECIAL_N_RAISE) {
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_N_RAISE);
		};
	};
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
	};
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialnhit", "game_specialairnhit", "game_specialnturn", "game_specialairnturn"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_neutralb_hit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 5.0);
	//macros::FT_MOTION_RATE(fighter, 0.75);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
		/*macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);*/
	};
    frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		AttackModule::clear_all(fighter.module_accessor);
	};
    frame(fighter.lua_state_agent, 45.0);
	//macros::FT_MOTION_RATE(fighter, 0.4);
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("top"), 0, 6, 7, 0, 0, 0, 1.6, true)
		}
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=8)
		if(is_excute){
			FLASH(1, 1, 1, 0.75)
		}
		wait(Frames=1)
		for(3 Iterations){
			if(is_excute){
				FLASH(0.7, 0.7, 0.7, 0.5)
			}
			wait(Frames=2)
			if(is_excute){
				FLASH(0.67, 0, 0.78, 0.31)
			}
			wait(Frames=2)
			if(is_excute){
				COL_NORMAL()
			}
			wait(Frames=2)
		}
		if(is_excute){
			FLASH(0.7, 0.7, 0.7, 0.5)
		}
		wait(Frames=2)
		if(is_excute){
			COL_NORMAL()
		}
    });
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_daisyspecialn", "effect_daisyspecialairn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn kirby_daisy_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("top"), 0, 6, 7, 0, 0, 0, 1.6, true)
		}
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=8)
		if(is_excute){
			FLASH(1, 1, 1, 0.75)
		}
		wait(Frames=1)
		for(3 Iterations){
			if(is_excute){
				FLASH(0.7, 0.7, 0.7, 0.5)
			}
			wait(Frames=2)
			if(is_excute){
				FLASH(0.67, 0, 0.78, 0.31)
			}
			wait(Frames=2)
			if(is_excute){
				COL_NORMAL()
			}
			wait(Frames=2)
		}
		if(is_excute){
			FLASH(0.7, 0.7, 0.7, 0.5)
		}
		wait(Frames=2)
		if(is_excute){
			COL_NORMAL()
		}
    });
}

#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_specialnhit", "effect_specialairnhit", "effect_specialnturn", "effect_specialairnturn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_neutralb_hit_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_COLOR(255.0/255.0, 210.0/255.0, 46.0/255.0)
		}
		frame(Frame=27)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 7, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_daisyspecialnhit", "effect_daisyspecialairnhit", "effect_daisyspecialnturn", "effect_daisyspecialairnturn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn kirby_daisy_neutralb_hit_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_COLOR(255.0/255.0, 210.0/255.0, 46.0/255.0)
		}
		frame(Frame=27)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 7, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
    });
}



pub fn install() {
	smashline::install_acmd_scripts!(
		daisy_downb,
		daisy_downb_eff,
		daisy_dtilt,
		daisy_dtilt_eff,
		daisy_uair,
		daisy_uair_eff,
		daisy_uair_snd,
		daisy_uair_expr,
		daisy_da,
		daisy_da_eff,
		daisy_fair,
		daisy_upb,
		daisy_jab1,
		daisy_jab1_eff,
		daisy_jab2,
		daisy_jab2_eff,
		daisy_nair,
		daisy_pummel_eff,
		daisy_fthrow,
		daisy_fthrow_eff,
		daisy_dthrow,
		daisy_dthrow_eff,
		daisy_uthrow_eff,
		daisy_bthrow_eff,
		daisy_remove_toad,
		daisy_catch_release_eff,
		daisy_neutralb,
		daisy_neutralb_eff,
		daisy_neutralb_hit,
		daisy_neutralb_hit_eff,
		kirby_daisy_neutralb_eff,
		kirby_daisy_neutralb_hit_eff
	);
	smashline::install_agent_frames!(daisy_frame);
	smashline::install_agent_frames!(kirby_daisy_frame);
}

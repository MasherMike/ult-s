use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
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
use smash::phx::Vector3f;
use crate::util::*;
use super::*;
use crate::ike::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		ike_dair,
		ike_bair_eff, ike_dair_eff, ike_fair_eff, ike_nair_eff,
		ike_uair_eff
    );
}
#[acmd_script(
    agent = "ike",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ike_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		wait(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 50, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_IKE, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_IKE, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ -6.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_IKE, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		if IKE_INSTALL[entry_id] > 0 {
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
		   WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			}
		};
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
#[acmd_script(
    agent = "ike",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_bair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_fair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 4);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_uair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_dair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("ike_attack_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "effect_attackairn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_nair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_u"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}
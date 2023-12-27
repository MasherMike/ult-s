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
use crate::murabito::*;
use super::*;



pub fn install() {
    smashline::install_agent_frames!(
        murabito_frame,
        tree_frame,
        seed_frame
    );
}
#[fighter_frame( agent = FIGHTER_KIND_MURABITO, main )]
fn murabito_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let motion_kind = MotionModule::motion_kind(boma);
            let situation_kind = StatusModule::situation_kind(boma);
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let end_frame = MotionModule::end_frame(boma);
            if  [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET].contains(&status_kind) {
                if frame < 2.0 && situation_kind == *SITUATION_KIND_GROUND {
                    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED) && ![hash40("special_n2"), hash40("special_n2_fail")].contains(&motion_kind) {
                        if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST {
                            MotionModule::change_motion(boma, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(boma, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
                        }
                    } else if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) && motion_kind ![hash40("special_n3"), hash40("special_n2_fail")].contains(&motion_kind) {
                        if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST {
                            MotionModule::change_motion(boma, Hash40::new("special_n3"), 0.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(boma, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
                        }
                    } else {
                        MotionModule::change_motion(boma, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }
                if situation_kind != SITUATION_KIND_GROUND && motion_kind != hash40("special_air_n") {
                    MotionModule::change_motion(boma, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                }
                if [hash40("special_n3")].contains(&motion_kind) {
                    if (!ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) || IS_FALLEN[ENTRY_ID]  || !((TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST)) {
                        StatusModule::change_status_request_from_script(weapon.module_accessor, *FIGHTER_STATUS_KIND_SLIP, false);
                    }
                }
                if frame >= end_frame {
                    if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                    } else {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                    };
                }
            }
            if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) && !DO_BOUNCE[ENTRY_ID] {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL){
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
                    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                        MotionModule::set_rate(boma, 0.5);
                    };
                    DO_BOUNCE[ENTRY_ID] = true;
                };
            };
            if situation_kind != *SITUATION_KIND_AIR {
                DO_BOUNCE[ENTRY_ID] = false;
            }
		}
    }
}
#[weapon_frame( agent = WEAPON_KIND_MURABITO_TREE)]
fn tree_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_MURABITO && is_default(&mut *boma) {
            if status_kind == *WEAPON_MURABITO_TREE_STATUS_KIND_STAND {
                IS_FALLEN[ENTRY_ID] = false;
            } else {
                IS_FALLEN[ENTRY_ID] = true;
            }
            TREE_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
            TREE_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_MURABITO_SEED)]
fn seed_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_MURABITO && is_default(&mut *boma) {
            TREE_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
            TREE_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
		};
    }
}

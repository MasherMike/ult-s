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
use crate::jack::*;

pub fn install() {
	Agent::new("jack")
	.on_line(Main, joker_frame)
	.install();

	Agent::new("kirby")
	.on_line(Main, kirby_joker_frame)
	.install();
}

//KIRBY - MAKE SURE ANY CHANGES TO NEUTRLAB ARE ON BOTH
unsafe extern "C" fn kirby_joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_JACK {
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
				GUN_C[ENTRY_ID] = NONE;
			};
			if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
				if MotionModule::frame(boma) < 20.0 {
					MotionModule::set_rate(boma, 2.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [*FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
				if ControlModule::get_stick_y(boma) <= -0.625 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
					GUN_C[ENTRY_ID] = ATTACK_N;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
					GUN_C[ENTRY_ID] = ATTACK_S3;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
					GUN_C[ENTRY_ID] = ATTACK_S4;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if motion_kind == hash40("attack_air_f") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_F;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
					if motion_kind == hash40("attack_air_b") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_B;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
					};
					if motion_kind == hash40("attack_air_hi") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
				};
				println!("{}", GUN_C[ENTRY_ID]);
			};
		};
    }
}
unsafe extern "C" fn joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		
		if is_default(boma) {
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
				GUN_C[ENTRY_ID] = NONE;
			};
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				BATON_TYPE[ENTRY_ID] = 2;
			};
			if ![*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go(){
				ArticleModule::remove_exist(boma, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE {
				if MotionModule::frame(boma) < 20.0 {
					MotionModule::set_rate(boma, 2.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
				if ControlModule::get_stick_y(boma) <= -0.625 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
					GUN_C[ENTRY_ID] = ATTACK_N;
					macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15e858a896), true, true);
					macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0a38380378), true, true);
					macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0f6ac1c8de), true, true);
					macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x17a5cc8181), true, true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
					GUN_C[ENTRY_ID] = ATTACK_S3;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
					GUN_C[ENTRY_ID] = ATTACK_S4;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if motion_kind == hash40("attack_air_f") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_F;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					};
					if motion_kind == hash40("attack_air_b") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_B;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
					};
					if motion_kind == hash40("attack_air_hi") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					};
				};
				println!("{}", GUN_C[ENTRY_ID]);
			};
		}
    }
}
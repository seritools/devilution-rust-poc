extern crate libc;

use libc::{c_char, c_int};

extern "C" {
  static mut cineflag: c_char;
  static mut zoomflag: c_int;
  static mut sgnTimeoutCurs: c_int;
  static mut sgbMouseDown: c_char;

  fn InitCursor();
  fn InitLightTable();
  fn LoadDebugGFX();
  fn music_stop();

  fn gmenu_init_menu();
  fn InitLevelCursor();
}

extern "fastcall" {
  fn ShowProgress(uMsg: c_int);
  fn track_repeat_walk(rep: bool);
}

#[no_mangle]
pub unsafe extern "fastcall" fn start_game(uMsg: c_int) {
  cineflag = 0;
  zoomflag = 1;
  InitCursor();
  InitLightTable();
  LoadDebugGFX();
  music_stop();
  ShowProgress(uMsg);
  gmenu_init_menu();
  InitLevelCursor();
  sgnTimeoutCurs = 0;
  sgbMouseDown = 0;
  track_repeat_walk(false);
}

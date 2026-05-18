import * as wasm from './pkg/yewchat_bg.wasm';
import { __wbg_set_wasm, run_app } from './pkg/yewchat_bg.js';

__wbg_set_wasm(wasm);
run_app();

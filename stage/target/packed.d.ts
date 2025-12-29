/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h9673fe99e50fa83d: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__hc2a21c334904b0f1: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h16fd9ea8e9df6f80: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h0418efbb081035b6: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h05bd6c2921a52269: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hc184d8334a1539e6: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h51a187437e982c46: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h82156cf743500bb6: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h968bae48f274170f: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__hf013fc8ed85f4555: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__closure__destroy__h39324fdc8421b761: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h059944162b1a90b2: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h672ecbe90d113901: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hf8a9154cdcff5b4b: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h3c3265cb4b53102b: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hc1314ea4380e450b: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h9f8265ca3d56df3e: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h33e3db080b918981: (a: number, b: number) => void;
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

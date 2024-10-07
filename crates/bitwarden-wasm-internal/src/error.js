/**
 * Error thrown by the WASM module.
 * @param {string} message - Error message.
 * @extends Error
 */
export class WasmError extends Error {
  constructor(message) {
    super(message);
    this.name = 'WasmError';
  }
}

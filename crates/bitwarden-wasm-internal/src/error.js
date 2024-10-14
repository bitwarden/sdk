/**
 * Error thrown by the WASM module.
 * @param {string} message - Error message.
 * @extends Error
 */
class WasmError extends Error {
  constructor(message) {
    super(message);
    this.name = "WasmError";
  }
}

exports.WasmError = WasmError;

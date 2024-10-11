/**
 * Error thrown by the WASM module.
 * @param {string} message - Error message.
 * @extends Error
 */
class WasmError extends Error {
  constructor(message, name) {
    super(message);
    this.name = name ?? "WasmError";
  }
}

exports.WasmError = WasmError;

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
}
export declare class BitwardenClient {
  constructor(settingsInput?: string | undefined | null, logLevel?: LogLevel | undefined | null);
  runCommand(commandInput: string): Promise<string>;
}

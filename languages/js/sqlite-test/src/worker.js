import { BitwardenClient, LogLevel } from "@bitwarden/sdk-client";

import sqlite3InitModule from "@sqlite.org/sqlite-wasm";
import { r } from "./run";

const log = console.log;
const error = console.error;

window.sqlite = null;

const initializeSQLite = async () => {
  try {
    console.log(performance.now());

    log("Loading and initializing SQLite3 module...");
    const sqlite3 = await sqlite3InitModule({
      print: log,
      printErr: error,
    });


    log("Running SQLite3 version", sqlite3.version.libVersion);
    let db;
    if ("opfs" in sqlite3) {
      db = new sqlite3.oo1.OpfsDb("/mydb.sqlite3");
      log("OPFS is available, created persisted database at", db.filename);
    } else {
      db = new sqlite3.oo1.DB("/mydb.sqlite3", "ct");
      log("OPFS is not available, created transient database", db.filename);
    }

    r(db);
  } catch (err) {
    if (!(err instanceof Error)) {
      err = new Error(err.result.message);
    }
    error(err.name, err.message);
  }
  console.log(performance.now());
  const module = await import("@bitwarden/sdk-wasm");

  const client = new BitwardenClient(
    new module.BitwardenClient(
      JSON.stringify({
        apiUrl: "",
        identityUrl: "",
      }),
      module.LogLevel.Debug,
    ),
  );
  console.log(performance.now());

  console.log("INITI");
  var t0 = performance.now();

  try {
    await client.accessTokenLogin("abc");
  } catch (e) {
    console.error(e);
  }
  var t1 = performance.now();
  console.log("Call to summBrute took " + (t1 - t0) + " milliseconds.");
};

initializeSQLite();

import { BitwardenClient, LogLevel } from "@bitwarden/sdk-client";
import * as SQLite from "wa-sqlite";
import SQLiteESMFactory from "wa-sqlite/dist/wa-sqlite-async.mjs";
import { IDBMirrorVFS } from "wa-sqlite/src/examples/IDBMirrorVFS";
import { IDBBatchAtomicVFS } from "wa-sqlite/src/examples/IDBBatchAtomicVFS";


const sqliteModule = await SQLiteESMFactory();
const sqlite3 = SQLite.Factory(sqliteModule);

// Register a custom file system.
const vfs = await IDBBatchAtomicVFS.create("hello", sqliteModule);
// const vfs = await IDBMirrorVFS.create("hello", sqliteModule);
sqlite3.vfs_register(vfs, true);

class SqliteDatabase  {
  constructor(db) {
    this.db = db;
  }

  static async factory(name) {
    // Open the database.
    const db = await sqlite3.open_v2(name);

    console.debug("OPENED DATABASE: ", name);

    window.sqlite = sqlite3;
    window.test = db;

    return new SqliteDatabase(db);
  }

  async get_version() {
    console.log("GET");
  }

  async set_version(version) {
    console.log("Version", version);
  }

  async execute(sql) {
    console.log(sql);
    // localStorage.setItem("sql", sql);
    // await sqlite3.exec(this.db, sql);
  }
}


window.SqliteDatabase = SqliteDatabase;

const module = await import("@bitwarden/sdk-wasm");

  const client = new BitwardenClient(
    await module.BitwardenClient.factory(
      JSON.stringify({
        apiUrl: "",
        identityUrl: "",
      }),
      module.LogLevel.Debug,
    ),
  );

  var t0 = performance.now();
  try {
    await client.accessTokenLogin("abc");
  } catch (e) {
    console.error(e);

  }
  var t1 = performance.now();
  console.log("Call to summBrute took " + (t1 - t0) + " milliseconds.");


/*

import * as SQLite from "wa-sqlite";
import SQLiteESMFactory from "wa-sqlite/dist/wa-sqlite-async.mjs";
import { IDBBatchAtomicVFS } from "wa-sqlite/src/examples/IDBBatchAtomicVFS";


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
*/

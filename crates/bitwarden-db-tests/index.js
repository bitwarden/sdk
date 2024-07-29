import { runTests } from "./pkg";

import * as SQLite from "wa-sqlite";
import SQLiteESMFactory from "wa-sqlite/dist/wa-sqlite-async.mjs";
import { IDBMirrorVFS } from "wa-sqlite/src/examples/IDBMirrorVFS";
import { IDBBatchAtomicVFS } from "wa-sqlite/src/examples/IDBBatchAtomicVFS";

const sqliteModule = await SQLiteESMFactory();
const sqlite3 = SQLite.Factory(sqliteModule);

// Register a custom file system.
// const vfs = await IDBBatchAtomicVFS.create("hello", sqliteModule);
// const vfs = await IDBMirrorVFS.create("hello", sqliteModule);
// sqlite3.vfs_register(vfs, true);

class SqliteDatabase {
  constructor(db) {
    this.db = db;
  }

  static async factory(name) {
    console.debug("OPENED DATABASE: ", name);

    // Open the database.
    const db = await sqlite3.open_v2(name);

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

  async execute_batch(sql) {
    console.log(sql);
    // localStorage.setItem("sql", sql);
    await sqlite3.exec(this.db, sql);
  }

  async execute(sql, params) {
    console.log(sql, params);
    for await (const stmt of sqlite3.statements(this.db, sql)) {
      let rc = sqlite3.bind_collection(stmt, params);

      while ((rc = await sqlite3.step(stmt)) !== SQLite.SQLITE_DONE) {
        console.log(rc);
      }
    }

    // localStorage.setItem("sql", sql);
    // await sqlite3.exec(this.db, sql);
  }

  async query_map(sql) {
    let rows = [];
    for await (const stmt of sqlite3.statements(this.db, sql)) {
      while ((await sqlite3.step(stmt)) === SQLite.SQLITE_ROW) {
        const row = sqlite3.row(stmt);
        rows.push(row);
      }
    }
    return rows;
  }
}

window.SqliteDatabase = SqliteDatabase;

runTests();

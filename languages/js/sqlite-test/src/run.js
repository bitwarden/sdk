export async function r(a) {
  window.runSql = async (sql) => {
    //console.log(sql);
    await a.exec({
      sql: sql,
    });
  }
}

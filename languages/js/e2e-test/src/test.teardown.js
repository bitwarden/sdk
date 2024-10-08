const fs = require("fs");

function teardown() {
  if (fs.existsSync("state.json")) {
    fs.unlinkSync("state.json");
  }

  if (fs.existsSync("mutable_state.json")) {
    fs.unlinkSync("mutable_state.json");
  }
}

module.exports = teardown;

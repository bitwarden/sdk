const { pathsToModuleNameMapper } = require("ts-jest");

/** @type {import('jest').Config} */
module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  setupFilesAfterEnv: ["<rootDir>/src/test.setup.ts"],
};

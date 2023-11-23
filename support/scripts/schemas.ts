import { quicktype, InputData, JSONSchemaInput, FetchingJSONSchemaStore } from "quicktype-core";

import fs from "fs";
import path from "path";

async function* walk(dir: string): AsyncIterable<string> {
  for await (const d of await fs.promises.opendir(dir)) {
    const entry = path.join(dir, d.name);
    if (d.isDirectory()) {
      yield* walk(entry);
    } else if (d.isFile()) {
      yield entry;
    }
  }
}

async function main() {
  const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());

  const filenames: string[] = [];
  for await (const p of walk("./support/schemas")) {
    filenames.push(p);
  }

  filenames.sort();

  for (const f of filenames) {
    const buffer = fs.readFileSync(f);
    const relative = path.relative(path.join(process.cwd(), "support/schemas"), f);
    await schemaInput.addSource({ name: relative, schema: buffer.toString() });
  }

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  const ts = await quicktype({
    inputData,
    lang: "typescript",
    rendererOptions: {},
  });

  writeToFile("./languages/js/sdk-client/src/schemas.ts", ts.lines);
  writeToFile("./crates/bitwarden-napi/src-ts/bitwarden_client/schemas.ts", ts.lines);

  const python = await quicktype({
    inputData,
    lang: "python",
    rendererOptions: {
      "python-version": "3.7",
    },
  });

  writeToFile("./languages/python/BitwardenClient/schemas.py", python.lines);

  const csharp = await quicktype({
    inputData,
    lang: "csharp",
    rendererOptions: {
      namespace: "Bitwarden.Sdk",
      framework: "SystemTextJson",
      "csharp-version": "6",
    },
  });

  writeToFile("./languages/csharp/Bitwarden.Sdk/schemas.cs", csharp.lines);
}

main();
function writeToFile(filename: string, lines: string[]) {
  const output = fs.createWriteStream(filename);
  lines.forEach((line) => {
    output.write(line + "\n");
  });
  output.close();
}

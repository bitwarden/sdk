// Quick script that parses the rustdoc json output and generates a basic markdown documentation.
//
// Do note that this script follows no best practices and will not handle anything many edge cases.

import fs from "fs";
import path from "path";
import Handlebars from "handlebars";

import { Input, InputType } from "./rustdoc";

const doc = JSON.parse(fs.readFileSync("./target/doc/bitwarden_uniffi.json", "utf8"));
const command = JSON.parse(
  fs.readFileSync("./support/schemas/bitwarden_json/Command.json", "utf8"),
);

const template = Handlebars.compile(
  fs.readFileSync(path.resolve(__dirname, "template.hbs"), "utf8"),
);

// Modify this to include more root elements
const rootElements = [
  "Client",
  "ClientKdf",
  "ClientCrypto",
  "ClientVault",
  "ClientCiphers",
  "ClientCollections",
  "ClientFolders",
  "ClientPasswordHistory",
];

const localIndexArray = Object.values(doc.index).filter((entry: any) => entry.crate_id == 0);
const localIndex = localIndexArray.reduce((map: any, obj: any) => {
  map[obj.id] = obj;
  return map;
}, {}) as Record<string, any>;

let usedDefinitions: any[] = [];

const out = rootElements.map((rootElement) => {
  const root: any = localIndexArray.find((entry: any) => entry.name == rootElement);
  const impls = root.inner.struct.impls;

  const elements = impls
    .flatMap((e: any) => localIndex[e])
    .flatMap((e: any) => e.inner.impl.items)
    .map((e: any) => localIndex[e])
    .filter((e: any) => e?.docs != null);

  return {
    name: rootElement,
    elements: elements.map((e: any) => {
      return {
        name: e.name,
        docs: e.docs,
        args: e.inner.function.decl.inputs.map((e: any) => map_input(e)),
        output: map_type(e.inner.function.decl.output),
      };
    }),
  };
});

function stripDef(str: string) {
  return str.replace(/#\/definitions\//g, "");
}

Handlebars.registerHelper("stripDef", (str) => {
  return stripDef(str);
});

// Add references
for (let i = 0; i < usedDefinitions.length; i++) {
  const key = usedDefinitions[i];
  const cmd = command.definitions[key];
  if (cmd == null) {
    continue;
  }

  Object.entries(cmd.properties ?? {}).forEach((prop: any) => {
    prop[1].allOf?.forEach((e: any) => {
      usedDefinitions.push(stripDef(e["$ref"] as string));
    });
  });
}

const filteredDefinitions = [...new Set(usedDefinitions)]
  .sort()
  .map((key) => [key, command.definitions[key]])
  .filter((e) => e[1] != null)
  .reduce((obj, cur) => ({ ...obj, [cur[0]]: cur[1] }), {});

console.log(template({ sections: out, commands: filteredDefinitions }));

///
/// Implementation details below.
///

// Format
function map_input(input: Input) {
  return {
    name: input[0],
    type: map_type(input[1]),
  };
}

function map_type(t: InputType) {
  const args = t.resolved_path?.args;
  const name = t.resolved_path?.name;

  let out = "";

  if (name) {
    usedDefinitions.push(name);

    if (command.definitions[name] != null) {
      out += `[${name}](#${name})`;
    } else {
      out += name;
    }
  }

  if (args != null && args.angle_bracketed.args.length > 0) {
    out += "<";
    out += args.angle_bracketed.args.map((t: any) => {
      if (t.type.generic) {
        return t.type.generic;
      } else if (t.type.resolved_path) {
        return t.type.resolved_path.name;
      }
    });
    out += ">";
  }
  return out;
}

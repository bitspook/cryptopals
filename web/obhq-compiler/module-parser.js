import { CellParser } from "@observablehq/parser";
import findReferences from "@observablehq/parser/src/references";
import findFeatures from "@observablehq/parser/src/features";
import defaultGlobals from "@observablehq/parser/src/globals";
import { getLineInfo } from "acorn";

export function parseModule(input, { globals } = {}) {
  const program = ModuleParser.parse(input);
  for (const cell of program.cells) {
    parseReferences(cell, input, globals);
    parseFeatures(cell, input, globals);
  }
  return program;
}

export class ModuleParser extends CellParser {
  parseTopLevel(node) {
    if (!node.cells) node.cells = [];
    while (this.type.label !== "eof") {
      const cell = this.parseCell(this.startNode());
      cell.input = this.input;
      node.cells.push(cell);
    }
    this.next();
    return this.finishNode(node, "Program");
  }
}
// Find references.
// Check for illegal references to arguments.
// Check for illegal assignments to global references.
function parseReferences(cell, input, globals = defaultGlobals) {
  if (!cell.body) {
    cell.references = [];
  } else if (cell.body.type === "ImportDeclaration") {
    cell.references = cell.body.injections
      ? cell.body.injections.map((i) => i.imported)
      : [];
  } else {
    try {
      cell.references = findReferences(cell, globals);
    } catch (error) {
      if (error.node) {
        const loc = getLineInfo(input, error.node.start);
        error.message += ` (${loc.line}:${loc.column})`;
        error.pos = error.node.start;
        error.loc = loc;
        delete error.node;
      }
      throw error;
    }
  }
  return cell;
}

// Find features: file attachments, secrets, database clients.
// Check for illegal references to arguments.
// Check for illegal assignments to global references.
function parseFeatures(cell, input) {
  if (cell.body && cell.body.type !== "ImportDeclaration") {
    try {
      cell.fileAttachments = findFeatures(cell, "FileAttachment");
      cell.databaseClients = findFeatures(cell, "DatabaseClient");
      cell.secrets = findFeatures(cell, "Secret");
    } catch (error) {
      if (error.node) {
        const loc = getLineInfo(input, error.node.start);
        error.message += ` (${loc.line}:${loc.column})`;
        error.pos = error.node.start;
        error.loc = loc;
        delete error.node;
      }
      throw error;
    }
  } else {
    cell.fileAttachments = new Map();
    cell.databaseClients = new Map();
    cell.secrets = new Map();
  }
  return cell;
}

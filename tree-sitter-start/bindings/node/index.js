const root = require("path").join(__dirname, "..", "..");

module.exports = require("node-gyp-build")(root);

try {
  module.exports.nodeTypeInfo = require("../../grammars/start/src/node-types.json");
  module.exports.nodeTypeInfo = require("../../grammars/start_repl/src/node-types.json");
} catch (_) {}

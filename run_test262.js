// Copied from https://github.com/tc39/test262-parser-tests#sample-consumer

const fs = require('fs');
const assert = require('assert');
const { execFileSync } = require('child_process');

function parse(srcPath, {isModule, earlyErrors}) {
  var output = execFileSync('cargo', ['run', '--quiet', '--', srcPath]);
  return output
}

let TESTS_ROOT = 'test262-parser-tests/';

let passExcludes = [];
let failExcludes = [];
let earlyExcludes = ['557.script.js', '558.script.js', '559.script.js', '560.script.js', '561.script.js', '563.script.js', '564.script.js', '565.script.js', '566.script.js', '567.script.js', '568.script.js', '569.script.js', '570.script.js', '571.script.js', '572.script.js', '574.script.js', '575.script.js', '576.script.js', '577.script.js', '578.script.js', '579.script.js', '580.script.js', '581.script.js', '582.script.js', '583.script.js', '585.script.js', '586.script.js', '587.script.js', '588.script.js', '589.script.js', '590.script.js', '591.script.js', '592.script.js', '593.script.js', '594.script.js', '596.script.js', '597.script.js', '598.script.js', '599.script.js', '600.script.js', '601.script.js', '602.script.js'];

fs.readdirSync(TESTS_ROOT + 'pass').filter(f => !passExcludes.includes(f)).forEach(f => {
  let firstTree, secondTree;
    firstTree = parse(
      TESTS_ROOT + `pass/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: true}
    );
  assert.doesNotThrow(() => {
    firstTree = parse(
      TESTS_ROOT + `pass/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: true}
    );
  });
  assert.doesNotThrow(() => {
    secondTree = parse(
      TESTS_ROOT + `pass-explicit/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: true}
    );
  });
  assert.deepStrictEqual(firstTree, secondTree);
});

fs.readdirSync(TESTS_ROOT + 'fail').filter(f => !failExcludes.includes(f)).forEach(f => {
  assert.throws(() => {
    parse(
      TESTS_ROOT + `fail/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: false}
    );
  });
});

fs.readdirSync(TESTS_ROOT + 'early').filter(f => !earlyExcludes.includes(f)).forEach(f => {
  assert.doesNotThrow(() => {
    parse(
      TESTS_ROOT + `early/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: false}
    );
  });
  assert.throws(() => {
    parse(
      TESTS_ROOT + `early/${f}`,
      {isModule: f.match('.module.js'), earlyErrors: true}
    );
  });
});

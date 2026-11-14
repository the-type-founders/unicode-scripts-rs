import data from "./source/data/scripts.js";

const scripts = Object.entries(data).map(([name, { code, ranges }]) => ({
  code,
  name,
  ranges,
}));

console.log(JSON.stringify(scripts, null, 2));

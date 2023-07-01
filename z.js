const fs = require("fs");
const path = require("path");

const templatePackageJson = {
  name: "REPLACEME",
  version: "1.0.0",
  // make this have a lot of stuff
}

for (let i = 0; i < 1000; i++) {
  const pkgJsonData = {
    ...templatePackageJson,
    name: `@fixtures/package-${i}`
  }
  const pkgDir = path.join("__fixtures__/packages", `package-${i}`);
  fs.mkdirSync(pkgDir);
  const pkgJsonFile = path.join(pkgDir, `package.json`);
  fs.writeFileSync(pkgJsonFile, `${JSON.stringify(pkgJsonData, null, 2)}\n`);
}

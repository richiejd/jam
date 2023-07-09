const fs = require("fs");
const path = require("path");

const templatePackageJson = {
    "name": "templatejsons",
    "version": "1.0.0",
    "description": "A lot of stuff",
    "main": "index.js",
    "scripts": {
        "format": "prettier --write ."
    },
    "author": "jacobchenry",
    "dependencies": {
        "async": "^3.2.4",
        "axios": "^1.4.0",
        "express": "^4.18.2",
        "he": "^1.2.0",
        "helmet": "^7.0.0",
        "mongoose": "^7.3.1",
        "react": "18.0.0",
        "react-native": "0.69.6",
        "react-native-dropdown-picker": "^5.4.6",
        "react-native-safe-area-context": "4.3.1",
        "react-native-screens": "~3.15.0"
    },
    "keywords": [
        "node.js",
        "javascript",
        "code",
    ]
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

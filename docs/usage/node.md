# Node

```ts
import { Miho } from 'miho';

// Set up Miho and search for packages.
const options = {
  release: 'patch',
  recursive: true,
  ignore: [/test/],
  overrides: {
    'that-project': 'major'
  }
};

const miho = await new Miho(options).search();

// Get basic information on the packages found.
// This also returns an id identifying each package,
// which can eventually be used to bump them individually.
console.log(miho.getPackages());

// Bump a package by its id.
await miho.bump(package.id);

// Bump all the packages found by Miho.
await miho.bumpAll();
```

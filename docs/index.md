---
layout: doc
title: Miho
titleTemplate: false
sidebar: true
aside: true
---

# Miho

Easily bump your package.json version.

## Config file

When using [Miho CLI](./cli/), it will try to find a config file named `miho.config.js` inside the [current working directory](https://nodejs.org/dist/latest/docs/api/process.html#processcwd). Any options set in the config file will eventually be passed down to Miho.

A example would look like this:

```ts
import { defineConfig } from 'miho';

export default defineConfig({
  release: 'patch',
  recursive: false,
  overrides: {
    'project-1': 'minor'
  }
});
```

Read the [Javascript API](./javascript-api/index.md#options) section for more details about the available options.

::: tip
Other extensions, like `.ts` and `.mjs`, are also supported.
:::

## CommonJS

```ts
const { Miho } = require('miho');
```

Miho also has a [CJS](https://nodejs.org/docs/latest/api/modules.html#modules-commonjs-modules) bundle, so you can `require` it too.

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

It is recommended to use a config file. It should be named `miho.config.ts`, but other extensions are also accepted, such as `.js`. For more details on the options available, [check the API](https://tb.dev.br/miho/api/interfaces/MihoOptions.html).

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

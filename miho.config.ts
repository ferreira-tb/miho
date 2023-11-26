import * as path from 'node:path';
import * as fs from 'node:fs/promises';
import process from 'node:process';
import { Octokit } from '@octokit/core';
import { existsSync as exists } from 'node:fs';
import { defineConfig } from './src';
import { $ } from 'execa';
import config from './config.json' assert { type: 'json' };
import packageJson from './package.json' assert { type: 'json' };

export default defineConfig({
  release: 'patch',
  verbose: true,
  commit: {
    all: true,
    push: true,
    noVerify: false
  },
  jobs: {
    build: async () => {
      const dist = path.join(process.cwd(), 'dist');
      if (exists(dist)) await fs.rm(dist, { recursive: true });
      await $({ stdio: 'inherit' })`run-s rollup minify`;
    },
    publish: async () => {
      const { version } = packageJson;
      const { GITHUB_TOKEN } = config;
      const octokit = new Octokit({ auth: GITHUB_TOKEN });

      await octokit.request('POST /repos/{owner}/{repo}/releases', {
        tag_name: version,
        name: version,
        draft: false,
        prerelease: false,
        generate_release_notes: true,
        owner: 'ferreira-tb',
        repo: 'miho',
        headers: {
          'X-GitHub-Api-Version': '2022-11-28',
          accept: 'application/vnd.github+json'
        }
      });
    }
  }
});

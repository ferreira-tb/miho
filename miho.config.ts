import { Octokit } from '@octokit/core';
import { defineConfig } from './src';
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
    build: true,
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

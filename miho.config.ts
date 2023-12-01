import { Octokit } from '@octokit/core';
import { PackageManager, defineConfig } from './src';

export default defineConfig({
  packageManager: PackageManager.PNPM,
  release: 'patch',
  verbose: true,
  commit: {
    all: false,
    push: false,
    noVerify: false,
    message: (miho) => {
      const pkg = miho.getPackageByName('miho');
      if (!pkg) throw new Error('No miho in the Miho package!?');
      const { version, newVersion } = pkg;
      if (!newVersion) return null;
      return `chore: bump Miho from ${version} to ${newVersion}`;
    }
  },
  jobs: {
    skip: ['build', 'publish'],
    build: true,
    publish: async () => {
      const { version } = await import('./package.json');
      const { GITHUB_TOKEN } = await import('./config.json');
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

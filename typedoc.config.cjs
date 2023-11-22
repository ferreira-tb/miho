/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['./src/index.ts'],
  out: 'docs',
  includeVersion: true,
  excludePrivate: true,
  excludeProtected: true,
  githubPages: false
};

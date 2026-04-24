const { FusesPlugin } = require('@electron-forge/plugin-fuses');
const { FuseV1Options, FuseVersion } = require('@electron/fuses');
const path = require('path');
const fs = require('fs');

module.exports = {
  packagerConfig: {
    asar: true,
    name: 'Dascord',
    productName: 'Dascord',
    executableName: 'Dascord',
    icon: path.join(__dirname, 'assets', 'icon'),
    extraResource: [
      path.join(__dirname, '..', 'client', '.next', 'standalone'),
    ],
  },
  hooks: {
    preMake: async () => {
      const standaloneDir = path.join(__dirname, '..', 'client', '.next', 'standalone');
      const staticSrc = path.join(__dirname, '..', 'client', '.next', 'static');
      const staticDst = path.join(standaloneDir, '.next', 'static');
      const publicSrc = path.join(__dirname, '..', 'client', 'public');
      const publicDst = path.join(standaloneDir, 'public');

      if (fs.existsSync(staticSrc)) {
        fs.cpSync(staticSrc, staticDst, { recursive: true });
      }
      if (fs.existsSync(publicSrc)) {
        fs.cpSync(publicSrc, publicDst, { recursive: true });
      }
    },
  },
  rebuildConfig: {},
  makers: [
    {
      name: '@electron-forge/maker-squirrel',
      config: {
        name: 'Dascord',
        setupIcon: path.join(__dirname, 'assets', 'icon.ico'),
      },
    },
    {
      name: '@electron-forge/maker-zip',
      platforms: ['darwin'],
    },
    {
      name: '@electron-forge/maker-dmg',
      config: {
        name: 'Dascord',
        format: 'ULFO',
      },
    },
    {
      name: '@electron-forge/maker-deb',
      config: {
        options: {
          name: 'Dascord',
          bin: 'Dascord',
          productName: 'Dascord',
          description: 'A Discord-like chat application',
          categories: ['Network', 'Chat'],
          icon: path.join(__dirname, 'assets', 'icon.png'),
        },
      },
    },
    {
      name: '@electron-forge/maker-rpm',
      config: {
        options: {
          name: 'Dascord',
          bin: 'Dascord',
          productName: 'Dascord',
          description: 'A Discord-like chat application',
          categories: ['Network'],
          icon: path.join(__dirname, 'assets', 'icon.png'),
        },
      },
    },
  ],
  plugins: [
    {
      name: '@electron-forge/plugin-vite',
      config: {
        build: [
          {
            entry: 'src/main.js',
            config: 'vite.main.config.mjs',
            target: 'main',
          },
          {
            entry: 'src/preload.js',
            config: 'vite.preload.config.mjs',
            target: 'preload',
          },
        ],
        renderer: [
          {
            name: 'main_window',
            config: 'vite.renderer.config.mjs',
          },
        ],
      },
    },
    new FusesPlugin({
      version: FuseVersion.V1,
      [FuseV1Options.RunAsNode]: false,
      [FuseV1Options.EnableCookieEncryption]: true,
      [FuseV1Options.EnableNodeOptionsEnvironmentVariable]: false,
      [FuseV1Options.EnableNodeCliInspectArguments]: false,
      [FuseV1Options.EnableEmbeddedAsarIntegrityValidation]: true,
      [FuseV1Options.OnlyLoadAppFromAsar]: true,
    }),
  ],
};

const esbuild = require('esbuild');
esbuild.buildSync({ entryPoints: ['link-detector.js'], outfile: 'dist/link-detector.js', bundle: true, platform: 'browser', target: 'chrome120', format: 'iife', legalComments: 'eof' });

import * as path from 'path';
import { defineConfig } from 'rspress/config';
import { pluginLastUpdated } from '@rspress/plugin-last-updated';
import { pluginShiki } from '@rspress/plugin-shiki';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import rehypeMermaid from 'rehype-mermaid';

export default defineConfig({
  globalStyles: path.join(__dirname, "styles/katex/katex.min.css"),
  root: path.join(__dirname, 'docs'),
  title: 'Aoike',
  description: 'AzurIce\'s Blog',
  icon: '/rspress-icon.png',
  logo: {
    light: '/rspress-light-logo.png',
    dark: '/rspress-dark-logo.png',
  },
  themeConfig: {
    socialLinks: [
      { icon: 'github', mode: 'link', content: 'https://github.com/azurice' },
    ],
  },
  markdown: {
    remarkPlugins: [remarkMath],
    rehypePlugins: [
      rehypeKatex,
      rehypeMermaid,
    ],
    mdxRs: false,
  },
  mediumZoom: {
    selector: '.rspress-doc img',
  },
  plugins: [
    pluginLastUpdated(),
    // pluginShiki({
    //   langs: ["nix", "c", "shell", "powershell", "asm", "nginx"]
    // })
  ]
});

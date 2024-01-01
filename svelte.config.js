import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

import rehypeSanitize from 'rehype-sanitize'
import rehypeStringify from 'rehype-stringify'
import remarkParse from 'remark-parse'
import remarkRehype from 'remark-rehype'
import {unified} from 'unified'


/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		vitePreprocess(),
		{
      name: 'md-to-svelte',
      markup: async ({ content, filename }) => {
          if (!filename.endsWith('.md')) {
              return;
          }

          console.log(content, filename);

          const file = await unified()
              .use(remarkParse)
              .use(remarkRehype)
              .use(rehypeSanitize)
              .use(rehypeStringify)
              .process(content);

          return {
              code: file.value,
          };
      },
  	}, 
	],

  extensions: [
    '.md',
    '.svelte',
  ],

	kit: {
		adapter: adapter()
	}
};

export default config;

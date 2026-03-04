// @ts-check
import starlight from '@astrojs/starlight'
import starlightLlmsTxt from 'starlight-llms-txt'
import { defineConfig, passthroughImageService } from 'astro/config'

export default defineConfig({
  site: 'https://reklawdbox.com',
  image: {
    service: passthroughImageService(),
  },
  integrations: [
    starlight({
      title: 'reklawdbox Docs',
      social: [{
        icon: 'github',
        label: 'GitHub',
        href: 'https://github.com/ryan-voitiskis/reklawdbox',
      }],
      plugins: [
        starlightLlmsTxt(),
      ],
      sidebar: [
        {
          label: 'Getting Started',
          autogenerate: { directory: 'getting-started' },
        },
        {
          label: 'Concepts',
          autogenerate: { directory: 'concepts' },
        },
        {
          label: 'Workflows',
          autogenerate: { directory: 'workflows' },
        },
        {
          label: 'Reference',
          autogenerate: { directory: 'reference' },
        },
        {
          label: 'Agent SOPs',
          autogenerate: { directory: 'agent' },
        },
        {
          label: 'Troubleshooting',
          autogenerate: { directory: 'troubleshooting' },
        },
      ],
    }),
  ],
})

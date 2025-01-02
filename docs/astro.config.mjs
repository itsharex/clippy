// @ts-check
import node from "@astrojs/node";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  output: "server",
  site: "https://clippy.coding.global",
  integrations: [
    starlight({
      title: "Clippy Docs",
      social: {
        github: "https://github.com/0-don/clippy",
        discord: "https://discord.gg/coding",
      },
      sidebar: [
        {
          label: "Quick Start",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "Installation Guide", slug: "guides/example" },
            {
              label: "Features",
              items: [
                { label: "Clipboard History", slug: "features/clipboard-history" },
                { label: "Global Hotkeys", slug: "features/hotkeys" },
                { label: "Cloud Sync", slug: "features/cloud-sync" },
                { label: "File Support", slug: "features/file-support" },
              ]
            },
          ],
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
        {
          label: "Legal",
          items: [
            { label: "Privacy Policy", slug: "legal/privacy-policy" },
            { label: "Terms of Service", slug: "legal/terms-of-service" },
          ],
        },
      ],
    }),
    tailwind(),
    sitemap(),
  ],

  adapter: node({ mode: "standalone" }),
});

import React from 'react'
import { DocsThemeConfig } from 'nextra-theme-docs'
import { useRouter } from "next/router";

const config: DocsThemeConfig = {
  logo: <span>RJC</span>,
  project: {
    link: 'https://github.com/clearfeld/rjc',
  },
  editLink: {
		text: "Edit this page on GitHub",
	},
  // chat: {
  //   link: 'https://discord.com',
  // },
  docsRepositoryBase: 'https://github.com/clearfeld/rjc/docs',
  primaryHue: 29,
  footer: {
    text: 'RJC',
  },
  useNextSeoProps() {
		const { route } = useRouter();
		if (route !== "/") {
			return {
				titleTemplate: "%s – RJC",
			};
		}
	},
}

export default config

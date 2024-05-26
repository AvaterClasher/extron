/** @format */

import { defineConfig } from "vocs";

export default defineConfig({
	sidebar: {
		"/": [
			{
				text: "Getting Started",
				link: "/docs",
			},
			{
				text: "Start coding in Extron",
				link: "/docs/start-coding",
			},
			{
				text: "Concepts",
				items: [
					{
						text: "Data Types",
						link: "/docs/concepts/data-type",
					},
					{
						text: "Keywords",
						link: "/docs/concepts/Keywords",
					},
					{
						text: "Operators",
						link: "/docs/concepts/Operators",
					},
					{
						text: "Syntax",
						link: "/docs/concepts/Syntax",
					},
				],
			},
			{
				items: [
					{
						text: "Standard Library",
						link: "/docs/std-lib",
					},
					{
						text: "Utils",
						link: "/docs/std-lib/util",
					},
					{
						text: "Array",
						link: "/docs/std-lib/array",
					},
					{
						text: "File System",
						link: "/docs/std-lib/fs",
					},
					{
						text: "Math",
						link: "/docs/std-lib/math",
					},
					{
						text: "String",
						link: "/docs/std-lib/string",
					},
				],
			},
		],
	},
	title: "Extron",
	titleTemplate: "%s | Extron",
	description: "A interpeter for a simple programming language",
	baseUrl: "https:///extron.vercel.app",
	socials: [
		{
			icon: "github",
			link: "https://github.com/AvaterClasher/extron",
		},
	],
	theme: {
		accentColor: {
			light: "rgb(33, 131, 88)",
			dark: "rgb(61, 214, 140)",
		},
	},
	editLink: {
		pattern:
			"https://github.com/AvaterClasher/extron/edit/main/site/docs/pages/:path",
		text: "Edit this page on GitHub",
	},
	ogImageUrl: 'https://vocs.dev/api/og?logo=%logo&title=%title&description=%description',
	head: (
		<>
			<meta property="og:type" content="website" />
			<meta property="og:title" content="Extron" />
			{/* <meta property="og:image" content="https://viem.sh/og-image.png" /> */}
			<meta property="og:url" content="https://extron.vercel.app" />
			<meta
				property="og:description"
				content="A interpeter for a simple programming language"
			/>
		</>
	),
});

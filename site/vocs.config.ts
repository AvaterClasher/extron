/** @format */

import { defineConfig } from "vocs";

export default defineConfig({
	title: "Extron",
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
});

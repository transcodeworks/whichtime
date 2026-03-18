import { defineConfig } from "vitepress";

export default defineConfig({
    title: "whichtime",
    description: "High-performance natural language date parsing for Rust, Swift, Kotlin, and Python",

    // Deploy to GitHub Pages
    base: "/whichtime/",

    // Clean URLs (no .html extension)
    cleanUrls: true,

    // Theme configuration
    themeConfig: {
        logo: "/logo.svg",

        nav: [
            { text: "Guide", link: "/guide/getting-started" },
            { text: "API", link: "/api/" },
            { text: "Locales", link: "/locales/" },
            {
                text: "Languages",
                items: [
                    { text: "Rust", link: "/languages/rust" },
                    { text: "Swift", link: "/languages/swift" },
                    { text: "Kotlin", link: "/languages/kotlin" },
                    { text: "Python", link: "/languages/python" },
                ],
            },
            {
                text: "Links",
                items: [
                    { text: "GitHub", link: "https://github.com/transcodeworks/whichtime" },
                    { text: "docs.rs: whichtime", link: "https://docs.rs/whichtime" },
                    { text: "docs.rs: whichtime-sys", link: "https://docs.rs/whichtime-sys" },
                ],
            },
        ],

        sidebar: {
            "/guide/": [
                {
                    text: "Introduction",
                    items: [
                        { text: "Getting Started", link: "/guide/getting-started" },
                        { text: "Installation", link: "/guide/installation" },
                        { text: "Basic Usage", link: "/guide/basic-usage" },
                    ],
                },
                {
                    text: "Core Concepts",
                    items: [
                        { text: "Parsing Expressions", link: "/guide/parsing-expressions" },
                        { text: "Working with Locales", link: "/guide/locales" },
                        { text: "Date Ranges", link: "/guide/date-ranges" },
                        { text: "Error Handling", link: "/guide/error-handling" },
                    ],
                },
                {
                    text: "Advanced",
                    items: [
                        { text: "Performance", link: "/guide/performance" },
                        { text: "Architecture", link: "/guide/architecture" },
                    ],
                },
            ],
            "/languages/": [
                {
                    text: "Language Bindings",
                    items: [
                        { text: "Rust", link: "/languages/rust" },
                        { text: "Swift", link: "/languages/swift" },
                        { text: "Kotlin", link: "/languages/kotlin" },
                        { text: "Python", link: "/languages/python" },
                    ],
                },
            ],
            "/locales/": [
                {
                    text: "Supported Locales",
                    items: [
                        { text: "Overview", link: "/locales/" },
                        { text: "English", link: "/locales/en" },
                        { text: "German", link: "/locales/de" },
                        { text: "Spanish", link: "/locales/es" },
                        { text: "French", link: "/locales/fr" },
                        { text: "Italian", link: "/locales/it" },
                        { text: "Japanese", link: "/locales/ja" },
                        { text: "Dutch", link: "/locales/nl" },
                        { text: "Portuguese", link: "/locales/pt" },
                        { text: "Russian", link: "/locales/ru" },
                        { text: "Swedish", link: "/locales/sv" },
                        { text: "Ukrainian", link: "/locales/uk" },
                        { text: "Chinese", link: "/locales/zh" },
                    ],
                },
            ],
            "/api/": [
                {
                    text: "API Reference",
                    items: [
                        { text: "Overview", link: "/api/" },
                        { text: "WhichTime", link: "/api/whichtime" },
                        { text: "ParsedResult", link: "/api/parsed-result" },
                        { text: "Components", link: "/api/components" },
                        { text: "Errors", link: "/api/errors" },
                    ],
                },
            ],
        },

        socialLinks: [{ icon: "github", link: "https://github.com/transcodeworks/whichtime" }],

        footer: {
            message: "Released under the MIT License.",
            copyright: "Copyright © 2024-present Transcode Inc.",
        },

        search: {
            provider: "local",
        },

        editLink: {
            pattern: "https://github.com/transcodeworks/whichtime/edit/main/docs/:path",
            text: "Edit this page on GitHub",
        },
    },

    // Markdown configuration
    markdown: {
        lineNumbers: true,
    },

    head: [
        ["link", { rel: "icon", href: "/whichtime/favicon.ico" }],
        ["meta", { name: "theme-color", content: "#3c8772" }],
        ["meta", { property: "og:type", content: "website" }],
        ["meta", { property: "og:title", content: "whichtime" }],
        ["meta", { property: "og:description", content: "High-performance natural language date parsing" }],
    ],
});

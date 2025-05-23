import {defineConfig} from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "loid",
    description: "The Neural Automation Framework",
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            {text: 'Home', link: '/'},
            {text: 'Guide', link: '/guide/idea'},
            {text: 'Concepts', link: '/concepts/architecture'},
            {text: 'Blog', link: '/blog'},
        ],

        sidebar: [
            {
                text: 'Guide',
                items: [
                    {text: 'What is loid?', link: '/guide/idea'},
                    {text: 'Getting Started', link: '/guide/getting-started'},
                ]
            },
            {
                text: 'Concepts',
                items: [
                    {text: 'Architecture', link: '/concepts/architecture'},
                ]
            },
            {
                text: 'Examples',
                items: [
                    {text: 'Sensors', link: '/examples/sensors'},
                    {text: 'Neurons', link: '/examples/neurons'},
                ]
            },
            {
                text: 'Blog',
                link: "/blog",
                items: [
                    {text: 'Welcome', link: '/blog/welcome'},
                ]
            },
        ],

        socialLinks: [
            {icon: 'github', link: 'https://github.com/loid-labs/loid'}
        ],

        search: {
            provider: 'local'
        }
    },
    srcDir: 'src',
})

import {defineConfig} from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "loid",
    description: "A VitePress Site",
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            {text: 'Home', link: '/'},
            {text: 'Concepts', link: '/concepts/architecture'},
            {text: 'Examples', link: '/examples/sensors'},
        ],

        sidebar: [
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
            }
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

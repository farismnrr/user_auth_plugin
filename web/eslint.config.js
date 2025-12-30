import js from '@eslint/js'
import pluginVue from 'eslint-plugin-vue'

export default [
    js.configs.recommended,
    ...pluginVue.configs['flat/recommended'],
    {
        rules: {
            // Vue specific
            'vue/multi-word-component-names': 'off',
            'vue/no-unused-vars': 'warn',
            'vue/no-v-html': 'warn',

            // JavaScript
            'no-unused-vars': 'warn',
            'no-console': 'off',
            'no-debugger': 'warn',
            'prefer-const': 'warn',
            'no-var': 'error'
        },
        languageOptions: {
            globals: {
                window: 'readonly',
                document: 'readonly',
                console: 'readonly',
                process: 'readonly',
                sessionStorage: 'readonly',
                localStorage: 'readonly',
                requestAnimationFrame: 'readonly',
                cancelAnimationFrame: 'readonly',
                setTimeout: 'readonly',
                clearTimeout: 'readonly',
                setInterval: 'readonly',
                clearInterval: 'readonly',
                alert: 'readonly',
                URLSearchParams: 'readonly',
                URL: 'readonly'
            }
        }
    },
    {
        ignores: ['dist/', 'node_modules/', '*.config.js']
    }
]

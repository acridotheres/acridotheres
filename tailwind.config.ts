import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {},
    fontFamily: {
      sans: ['Readex Pro Variable', 'Readex Pro', 'sans-serif'],
    }
  },

  plugins: []
} satisfies Config;

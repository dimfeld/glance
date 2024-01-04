import { fontFamily } from 'tailwindcss/defaultTheme';
import colors from 'tailwindcss/colors';
import typography from '@tailwindcss/typography';
import svelteUx from 'svelte-ux/plugins/tailwind.cjs';

/** @type {import('tailwindcss').Config} */
const config = {
  darkMode: ['class'],
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/svelte-ux/**/*.{svelte,js}'],
  safelist: ['dark'],
  plugins: [typography, svelteUx],
  ux: {
    themes: {
      light: {
        // based on daisy UI bumblebee
        primary: 'oklch(80.39% 0.194 50.76)',
        'primary-content': 'oklch(39.38% 0.068 70.76)',
        secondary: 'oklch(89.51% 0.2132 96.61)',
        'secondary-content': 'oklch(38.92% 0.046 96.61)',
        accent: 'oklch(81.27% 0.157 56.52)',
        neutral: 'oklch(12.75% 0.075 281.99)',
        'surface-100': 'oklch(100% 0 0)',
      },

      dark: {
        primary: 'hsl(35 93% 55%)',
        'primary-content': 'hsl(35 9.3% 5.5%)',

        secondary: 'hsl(35 53.94% 20.35%)',
        'secondary-content': 'hsl(35 9.3% 97.75%)',

        neutral: 'hsl(35 53.94% 20.35%)',
        'neutral-content': 'hsl(35 9.3% 55.5%)',

        'surface-100': 'hsl(35 60.45% 4.4%)',
        'surface-content': 'hsl(35 9.3% 90%)',

        accent: 'hsl(35 53.94% 20.35%)',
        'accent-foreground': 'hsl(35 9.3% 97.75%)',

        info: '#8DCAC1',
        success: '#9DB787',
        warning: '#FFD25F',

        danger: 'hsl(0 62.8% 40.6%)',
        'danger-content': 'hsl(35 9.3% 97.75%)',
      },

      // dark: {
      //   // based on daisy UI coffee
      //   primary: '#DB924B',
      //   secondary: '#263E3F',
      //   accent: '#10576D',
      //   neutral: '#322C32',
      //   'surface-100': '#20161F',
      //   'surface-content': '#c5bfa0',
      //   info: '#8DCAC1',
      //   success: '#9DB787',
      //   warning: '#FFD25F',
      //   danger: '#FC9581',
      // },
    },
  },
  theme: {
    container: {
      center: true,
      padding: '2rem',
      screens: {
        '2xl': '1400px',
      },
    },
    extend: {
      fontFamily: {
        sans: [...fontFamily.sans],
      },
    },
  },
};

export default config;

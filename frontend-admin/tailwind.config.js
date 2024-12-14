import defaultTheme from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter var", ...defaultTheme.fontFamily.sans],
        oswald: ["Oswald", ...defaultTheme.fontFamily.serif],
        space: ["Space Mono", ...defaultTheme.fontFamily.mono],
      },
      colors: {
        tealdeuterium: {
          50: '#E6F2F2',
          100: '#CCE5E5',
          200: '#99CCCC',
          300: '#66B2B2',
          400: '#339999',
          500: '#008080',
          600: '#007373',
          700: '#006666',
          800: '#005959',
          900: '#004D4D',
          950: '#003333',
        },
        orangedeuterium: {
          50: '#FFF3E6',
          100: '#FFE6CC',
          200: '#FFCC99',
          300: '#FFB366',
          400: '#FF9933',
          500: '#FF8811',
          600: '#E67A0F',
          700: '#CC6D0E',
          800: '#B35F0C',
          900: '#99510A',
          950: '#663408',
        },
        floral: {
          50: '#FFFCFA',
          100: '#FFF8F0',
          200: '#F2ECE3',
          300: '#E6E1D7',
          400: '#D9D6CB',
          500: '#CCCABE',
          600: '#BFBBB2',
          700: '#B2ACA5',
          800: '#A59D98',
          900: '#99908B',
          950: '#7D746E', // Darker, greyer tint for neutrality
        },
      },
    },
  },
  plugins: [],
}


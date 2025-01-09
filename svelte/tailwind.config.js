import defaultTheme from 'tailwindcss/defaultTheme';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			backgroundImage: {
				'roses-center': "url('/beauty/roses-center-2.svg')"
			},
			fontFamily: {
				// sans: ["Inter var", ...defaultTheme.fontFamily.sans],
				sans: ['Montserrat', ...defaultTheme.fontFamily.sans],
				serif: ['Lora', ...defaultTheme.fontFamily.serif],
			},
			colors: {
				forest: {
					50: '#F6F8F2',
					100: '#EDF0E4',
					200: '#DCE2CC',
					300: '#CBD4B4',
					400: '#BAC69C',
					500: '#A9B884',
					600: '#8FA066',
					700: '#75854C',
					800: '#5C6B32',
					900: '#435118',
					950: '#2A380F'
				},
				sand: {
					50: '#FDF8F0',
					100: '#F9ECD8',
					200: '#F2D8B1',
					300: '#EBC48A',
					400: '#E4B063',
					500: '#DD9C3C',
					600: '#C47F28',
					700: '#A3651F',
					800: '#824C16',
					900: '#61330D',
					950: '#412009'
				}
			}
		}
	},
	plugins: [
		require('@tailwindcss/container-queries'),
	]
};

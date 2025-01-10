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
				},
				brick: {
					50: '#FCF5F3',
					100: '#F7E4DE',
					200: '#EDCBBD',
					300: '#E3B09C',
					400: '#D9957B',
					500: '#CF7A5A',
					600: '#A85E41',
					700: '#592614',  // Your provided color
					800: '#4A1F10',
					900: '#3B180C',
					950: '#2C1109'
				},
				pakistan: {
					50: '#F4F7F3',
					100: '#E5EBE2',
					200: '#CFDAC9',
					300: '#B8C8AF',
					400: '#A2B795',
					500: '#8BA57C',
					600: '#698357',
					700: '#486239',
					800: '#1C3A13',  // Your provided color
					900: '#152D0E',
					950: '#0E1F09'
				}
			}

		}
	},
	plugins: [
		require('@tailwindcss/container-queries'),
	]
};

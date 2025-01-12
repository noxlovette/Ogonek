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
				mono: ['Noto Sans Mono', ...defaultTheme.fontFamily.mono]
			},
			colors: {
				brick: {
					50: '#FAF6F4',
					100: '#F5E9E4',
					200: '#E8D4CC',
					300: '#DBBEB3',
					400: '#CCA79A',
					500: '#BD8F7F',
					600: '#A67864',
					700: '#8B614E',
					800: '#5D3F32',    // More earthy brown than red
					900: '#3B2820',
					950: '#291B16'
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
				},
				milk: {
					50: '#F8F7F4',     // Slightly warmer than pure white
					100: '#E8E6DD',
					200: '#D3D1CC',
					300: '#C8C7C3',
					400: '#B3B3B0',
					500: '#9F9E9D',
					600: '#8B8B8A',
					700: '#777776',
					800: '#636363',
					900: '#4F4F4F',
					950: '#3B3B3B'
				},
				azure: {
					50: '#F0F7F9',
					100: '#E1EEF2',
					200: '#C3DFE6',
					300: '#A5CFD9',
					400: '#87C0CD',
					500: '#5FA3B3',
					600: '#458799',
					700: '#2e6171',    // Your color
					800: '#234B57',
					900: '#18343D',
					950: '#0F2228'
				}
			}

		}
	},
	plugins: [
		require('@tailwindcss/container-queries'),
		require('@tailwindcss/forms'),
		require('@tailwindcss/typography'),
	]
};

import defaultTheme from 'tailwindcss/defaultTheme';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			backgroundImage: {
				'roses-center': "url('/beauty/roses-center-2.svg')",
				noise: "url('/beauty/nnnoise.svg')"
			},
			fontFamily: {
				// sans: ["Inter var", ...defaultTheme.fontFamily.sans],
				garamond: ['EB Garamond', 'serif'],
				sans: ['Montserrat', ...defaultTheme.fontFamily.sans],
				serif: ['Lora', ...defaultTheme.fontFamily.serif],
				playfair: ['Playfair Display', 'serif']
			},
			colors: {
				// First color - 646f4b (Forest)
				forest: {
					50: '#F5F7F0',
					100: '#EBEDE0',
					200: '#E1E4D0',
					300: '#D6DABF',
					400: '#CBD1AF',
					500: '#C0C89F',
					600: '#A6AF7F',
					700: '#8C965F',
					800: '#737D3F',
					900: '#59641F',
					950: '#3F4B00'
				},

				// Second color - f5e0b7 (Sand)
				sand: {
					50: '#FFF7E6',
					100: '#FFEDC3',
					200: '#FFE3A0',
					300: '#FFD97D',
					400: '#FFCF5A',
					500: '#FFC537',
					600: '#FFA200',
					700: '#CC8200',
					800: '#996100',
					900: '#664100',
					950: '#332000'
				},

				// Third color - e7a977 (Clay)
				clay: {
					50: '#FFEAD7',
					100: '#FFD6B3',
					200: '#FFC28F',
					300: '#FFAE6B',
					400: '#FF9A47',
					500: '#FF8623',
					600: '#E66A00',
					700: '#B35200',
					800: '#803A00',
					900: '#4D2200',
					950: '#1A0A00'
				},

				// Fourth color - 453a49 (Dusk)
				dusk: {
					50: '#F4F3F5',
					100: '#E9E7EA',
					200: '#DDDADF',
					300: '#D2CDD4',
					400: '#C6C1C9',
					500: '#BAB5BE',
					600: '#9A929E',
					700: '#7A6F7E',
					800: '#5A4C5E',
					900: '#3A293E',
					950: '#1A061E'
				},

				// Fifth color - a63c06 (Rust)
				rust: {
					50: '#FFE1D1',
					100: '#FFC3A3',
					200: '#FFA575',
					300: '#FF8747',
					400: '#FF6919',
					500: '#FF4B00',
					600: '#E63300',
					700: '#B32600',
					800: '#801A00',
					900: '#4D0D00',
					950: '#1A0000'
				}
			}
		}
	},
	plugins: []
};

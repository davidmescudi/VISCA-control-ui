import aspectRatio from '@tailwindcss/aspect-ratio';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			keyframes: {
				blinkGreen: {
					'0%, 100%': { borderColor: 'border-neutral-700' },
					'50%': { borderColor: 'green' }
				},
				blinkRed: {
					'0%, 100%': { borderColor: 'border-neutral-700' },
					'50%': { borderColor: 'red' }
				}
			},
			animation: {
				blinkGreen: 'blinkGreen 0.5s ease-in-out 2',
				blinkRed: 'blinkRed 0.5s ease-in-out 2',
			}
		}
	},

	plugins: [typography, forms, aspectRatio]
} as Config;

// Prevents FOUC (Flash of Unstyled Content)
const theme =
	localStorage.currentTheme === 'dark' ||
	(!('currentTheme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)
		? 'dark'
		: 'light';

document.documentElement.classList.toggle('dark', theme === 'dark');

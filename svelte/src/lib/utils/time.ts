export const formatDateTime = (isoString: string): string => {
	const date = new Date(isoString);

	return new Intl.DateTimeFormat('en-GB', {
		// year: 'numeric',
		month: 'short',
		day: 'numeric'
		// hour: 'numeric',
		// minute: 'numeric',
		// hour12: true
	}).format(date);
};

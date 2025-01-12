export const formatDateTime = (isoString: string): string => {
	const date = new Date(isoString);

	return new Intl.DateTimeFormat('en-US', {
		year: 'numeric',
		month: 'long',
		day: 'numeric',
		hour: 'numeric',
		minute: 'numeric',
		hour12: true
	}).format(date);
};

export function getGreeting() {
	const date = new Date();
	const hours = date.getHours();

	if (hours >= 5 && hours < 12) {
		return 'morning ☕';
	} else if (hours >= 12 && hours < 18) {
		return 'afternoon ☀️';
	} else if (hours >= 18 && hours < 22) {
		return 'evening 🌖';
	} else {
		return 'night';
	}
}


import { importSPKI, jwtVerify } from 'jose';
import { env } from '$env/dynamic/public';


export async function ValidateAccess(jwt: string) {
	const spki = env.PUBLIC_spki || '';
	const alg = env.PUBLIC_alg || 'RS256';
	const publicKey = await importSPKI(spki, alg)


	const { payload } = await jwtVerify(jwt, publicKey, {
		issuer: 'auth:auth',
		// audience: 'svelte:user:general',
	})

	return payload
}

import { createAvatar } from '@dicebear/core';
import { lorelei, thumbs } from '@dicebear/collection';

export function getAvatar(seed: string) {
	const avatar = createAvatar(thumbs, {
		seed,
	});

	return avatar.toDataUri();
}

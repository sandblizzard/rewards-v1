import { writable } from 'svelte/store';

export const connectedUserWallet = writable(null);

export const loggedInWithGithub = writable(false);

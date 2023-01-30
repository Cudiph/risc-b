import { writable } from 'svelte/store';
import { getLocalConfig } from '$lib/processor';

const menu = writable('main');
const query = writable('');
const whitelistSet = writable(new Set('sample'));

if (import.meta.env.VITE_WEBAPP === 'true') {
  const parsed = getLocalConfig();
  whitelistSet.set(new Set(parsed.whitelist));
} else {
  browser.storage.local.get('whitelist').then((item) => {
    whitelistSet.set(new Set(item.whitelist));
  });
}

export { menu, query, whitelistSet };

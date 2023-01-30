import { whitelistSet } from './stores';
import { get } from 'svelte/store';

export async function removeWhiteList(cekres: CekResponse): Promise<CekResponse> {
  if (!cekres.result_vec) {
    return cekres;
  }
  let invalid_term_pointer = -1;
  const index_to_remove: number[] = [];
  for (let i = 0; i < cekres.result_vec.length; i++) {
    const word = cekres.result_vec[i];
    if (!word.startsWith('<b>') || !word.endsWith('</b>')) continue;

    invalid_term_pointer++;
    const term = cekres.reccomendation[invalid_term_pointer][1];

    if (!get<Set<string>>(whitelistSet).has(term)) continue;

    index_to_remove.unshift(invalid_term_pointer);
    cekres.result_vec[i] = term;
  }

  // remove whitelisted
  for (const num of index_to_remove) {
    cekres.reccomendation.splice(num, 1);
  }

  // remove whitelisted for invalid case
  for (let i = cekres.reccomendation.length - 1; i >= 0; i--) {
    if (cekres.reccomendation[i][0] === 'TERM') break;
    let term = cekres.reccomendation[i][1];
    if (!get<Set<string>>(whitelistSet).has(term)) continue;

    cekres.reccomendation.splice(i, 1);
  }

  return cekres;
}

export function getLocalConfig(): LocalStorage {
  const raw = localStorage.getItem('config');
  let parsed: LocalStorage;
  if (!raw) {
    parsed = {
      endpoint: import.meta.env.VITE_DEFAULT_ENDPOINT,
      english: false,
      correction: false,
      format: 'HTML',
      tolerance: 'LOW',
      tidak_baku: false,
      whitelist: [],
    };

    localStorage.setItem('config', JSON.stringify(parsed));
  } else {
    parsed = JSON.parse(raw);
  }

  return parsed;
}

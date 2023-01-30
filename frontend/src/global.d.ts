/// <reference types="svelte" />

interface CekRequest {
  query: string;
  format: string;
  correction: bool;
  tidak_baku: bool;
  result_vec: bool;
  english: bool;
  tolerance: string;
}

interface CekResponse {
  result: string;
  result_vec: string[];
  valid: boolean;
  reccomendation: [string, string, [string, string][]][];
  detail?: KBBIJson;
  error?: string;
}

interface KBBIJson {
  pranala: string;
  entri: KBBIEntry[];
}

interface KBBIEntry {
  nama: string;
  nomor: string;
  kata_dasar: string[];
  pelafalan: string;
  bentuk_tidak_baku: string[];
  varian: string[];
  makna: KBBIMakna[];
}

interface KBBIMakna {
  kelas: KBBIKelas[];
  submakna: string[];
  info: string;
  contoh: string[];
}

interface KBBIKelas {
  kode: string;
  nama: string;
  deskripsi: string;
}

interface LocalStorage {
  endpoint: string;
  english: boolean;
  correction: boolean;
  format: 'MD' | 'HTML' | 'NONE';
  tolerance: 'LOW' | 'MEDIUM' | 'HIGH' | 'HIGHEST';
  tidak_baku: boolean;
  whitelist: string[];
}

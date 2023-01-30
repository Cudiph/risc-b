<script lang="ts">
  import { getLocalConfig } from '$lib/processor';
  import Whitelist from './Whitelist.svelte';

  let english = false;
  let correction = false;
  let tidak_baku = false;
  let format: 'MD' | 'HTML' | 'NONE' = 'HTML';
  let tolerance: 'LOW' | 'MEDIUM' | 'HIGH' | 'HIGHEST' = 'LOW';
  let endpoint = import.meta.env.VITE_DEFAULT_ENDPOINT;
  let saveOnChange: () => void;

  if (import.meta.env.VITE_WEBAPP === 'true') {
    saveOnChange = () => {
      const config = getLocalConfig();
      let obj_config = {
        english,
        endpoint,
        correction,
        format,
        tolerance,
        tidak_baku,
        whitelist: config.whitelist,
      };

      localStorage.setItem('config', JSON.stringify(obj_config));
    };

    let config = getLocalConfig();

    english = config.english;
    correction = config.correction;
    tidak_baku = config.tidak_baku;
    format = config.format;
    tolerance = config.tolerance;
    endpoint = config.endpoint;
  } else {
    browser.storage.local
      .get({
        endpoint: import.meta.env.VITE_DEFAULT_ENDPOINT,
        english: false,
        correction: false,
        format: 'HTML',
        tolerance: 'LOW',
        tidak_baku: false,
      })
      .then((items) => {
        english = items.english;
        correction = items.correction;
        tidak_baku = items.tidak_baku;
        format = items.format;
        tolerance = items.tolerance;
        endpoint = items.endpoint;
      });

    saveOnChange = () => {
      browser.storage.local.set({
        english,
        correction,
        format,
        tolerance,
        endpoint,
        tidak_baku,
      });
    };
  }
</script>

<div class="options">
  <div class="card" title="REST API endpoint untuk digunakan apabila hosting server secara mandiri">
    <label for="endpoint">API endpoint:</label>
    <input type="text" id="endpoint" bind:value={endpoint} on:input={saveOnChange} />
  </div>

  <div class="card" title="Kata tidak baku seperti apotik akan dianggap valid apabila diceklis">
    <label for="tidak-baku">Tidak Baku:</label>
    <input id="tidak-baku" type="checkbox" bind:checked={tidak_baku} on:change={saveOnChange} />
  </div>

  <div class="card" title="Kata dari Bahasa Inggris akan dianggap valid apabila diceklis">
    <label for="english">Bahasa Inggris:</label>
    <input id="english" type="checkbox" bind:checked={english} on:change={saveOnChange} />
  </div>

  <div class="card" title="Kata yang salah akan dikoreksi pada bagian Hasil apabila tersedia">
    <label for="english">Koreksi kata yang salah:</label>
    <input id="correction" type="checkbox" bind:checked={correction} on:change={saveOnChange} />
  </div>

  <div class="card" title="Lihat Levahnstein Distance di Wikipedia">
    <label for="tolerance">Tingkat Toleransi (Jarak Levahnstein):</label>
    <select id="tolerance" bind:value={tolerance} on:change={saveOnChange}>
      <option value="LOW">Rendah</option>
      <option value="MEDIUM">Sedang</option>
      <option value="HIGH" disabled>Tinggi</option>
      <option value="HIGHEST" disabled>Paling Tinggi</option>
    </select>
  </div>

  <!-- <div class="card" title="Dinonaktifkan untuk penggunaan umum"> -->
  <!--   <label for="format">format keluaran:</label> -->
  <!--   <select id="format" bind:value={format} on:change={saveOnChange} disabled> -->
  <!--     <option value="HTML">HTML</option> -->
  <!--     <option value="MD">Markdown</option> -->
  <!--     <option value="NONE">None</option> -->
  <!--   </select> -->
  <!-- </div> -->

  <Whitelist />
</div>

<style>
  input[type='text'] {
    width: 100%;
  }
</style>

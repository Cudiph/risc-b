<script lang="ts">
  import { removeWhiteList, getLocalConfig } from '$lib/processor';
  import Detail from './Detail.svelte';
  import Suggestion from './Suggestion.svelte';
  import { query } from '$lib/stores';

  let cekresponse: Promise<CekResponse> = null;

  let timeoutID: number;
  function handleInput() {
    clearTimeout(timeoutID);
    timeoutID = setTimeout(async () => {
      if ($query.trim() === '') return;
      let options: any;

      if (import.meta.env.VITE_WEBAPP === 'true') {
        const config = getLocalConfig();
        options = {
          english: config.english,
          correction: config.correction,
          format: config.format,
          tolerance: config.tolerance,
          tidak_baku: config.tidak_baku,
        };
      } else {
        options = await browser.storage.local.get([
          'english',
          'correction',
          'format',
          'tolerance',
          'tidak_baku',
        ]);
      }

      let payload = {
        query: $query,
        result_vec: true,
        ...options,
      };

      cekresponse = requestToServer(payload as CekRequest);
    }, 1000);
  }

  async function requestToServer(payload: CekRequest): Promise<CekResponse> {
    let endpoint: string;

    if (import.meta.env.VITE_WEBAPP === 'true') {
      const config = getLocalConfig();
      endpoint = config.endpoint;
    } else {
      endpoint = (await browser.storage.local.get('endpoint')).endpoint;
    }

    let response: Response;
    try {
      response = await fetch(endpoint, {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
          'Content-Type': 'application/json',
        },
      });
    } catch (e) {
      throw new Error(e);
    }

    return response.json() as Promise<CekResponse>;
  }
</script>

<div class="input-form">
  <!-- svelte-ignore a11y-autofocus -->
  <textarea
    autofocus
    rows="8"
    on:input={handleInput}
    bind:value={$query}
    placeholder="mulai mengetik..."
  />
  {#if cekresponse}
    {#await cekresponse}
      <p>Memuat Hasil...</p>
    {:then ceker}
      {#if ceker.detail}
        <Detail detail={ceker.detail} />
      {:else}
        <Suggestion cekResponse={removeWhiteList(ceker)} />
      {/if}
    {:catch err}
      <p class="red">{err}</p>
    {/await}
  {/if}
</div>

<style>
  textarea {
    width: 95%;
    resize: vertical;
  }

  textarea {
    outline: none;
  }
</style>

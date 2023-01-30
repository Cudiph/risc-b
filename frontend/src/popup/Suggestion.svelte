<script lang="ts">
  export let cekResponse: Promise<CekResponse>;

  function joinSuggestion(listofsug: [string, string][]) {
    const sugTermOnly = [];
    for (const tuple of listofsug) {
      sugTermOnly.push(tuple[1]);
    }
    return sugTermOnly.join(', ');
  }
</script>

<div>
  {#await cekResponse}
    <p>Memuat hasil...</p>
  {:then res}
    {#if res.error}
      <p class="red">Error: {res.error}</p>
    {:else if !res.reccomendation.length}
      <h2>Kalimatnya sudah <span class="green">valid</span>!</h2>
    {:else}
      <h2>Kalimat <span class="red">tidak valid</span>!</h2>
      <div class="card">
        <p>Hasil:</p>
        <p>
          {@html res.result_vec
            .join('')
            .replace(/\n/g, '<br>')
            .replace(/<(?!\/?br?)/gim, '&lt;')
            .replace(/(?<!\/?br?)>/gim, '&gt;')}
        </p>
      </div>

      <div class="card">
        <p>Remediasi</p>
        <ol>
          {#each res.reccomendation as rec}
            <li>
              <b>{rec[1]}:</b>
              {#if rec[2].length !== 0}
                {joinSuggestion(rec[2])}
              {:else}
                Tidak ada remediasi
              {/if}
            </li>
          {/each}
        </ol>
      </div>
    {/if}
  {:catch err}
    <p class="red">Error: {err}</p>
  {/await}
</div>

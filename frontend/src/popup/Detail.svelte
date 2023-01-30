<script lang="ts">
  export let detail: KBBIJson;
  const colors = ['red', 'orange'];

  // change (n) to superscript
  function parseSuperscript(str: string): string {
    const match = str.match(/(.*)\((\d+)\)$/);
    if (match) {
      const ret = match[1].trim() + `<sup>${match[2]}</sup>`;
      return ret;
    }

    return str;
  }
</script>

<div>
  <p>Pranala: <a href={detail.pranala}>kbbi.kemdikbud.go.id</a></p>
  {#each detail.entri as entri}
    <h2>
      {#each entri.kata_dasar as kd}
        <span>{@html parseSuperscript(kd)} &gt;&gt;</span>
      {/each}
      {entri.nama}{#if entri.nomor}<sup>{entri.nomor}</sup>{/if}
      <span class="pelafalan">{entri.pelafalan}</span>
    </h2>

    <span class:hide={!entri.bentuk_tidak_baku.length}>
      bentuk tidak baku: <b>{entri.bentuk_tidak_baku.join(', ')} </b>
    </span>
    <ol class="align-left">
      {#each entri.makna as makna}
        <li>
          {#each makna.kelas as kelas, i}
            <span
              class="{colors[i % colors.length]} kelas"
              title={kelas.nama + (kelas.deskripsi ? ': ' + kelas.deskripsi : '')}
              >{kelas.kode}</span
            >
          {/each}
          <span class="submakna">{makna.submakna.join('; ')}</span>
          <span class="contoh" class:hide={!makna.contoh.length}>: {makna.contoh.join('')}</span>
        </li>
      {/each}
    </ol>
  {/each}
</div>

<style>
  .kelas {
    font-style: italic;
  }
  .kelas::after {
    content: ' ';
  }

  .orange {
    color: orange;
  }

  .contoh {
    opacity: 50%;
  }

  .hide {
    display: none;
  }

  .align-left {
    text-align: left;
  }

  .pelafalan {
    opacity: 60%;
    font-size: 0.9em;
  }
</style>

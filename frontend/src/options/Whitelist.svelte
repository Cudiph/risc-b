<script lang="ts">
  import { slide } from 'svelte/transition';
  import { whitelistSet } from '$lib/stores';
  import { getLocalConfig } from '$lib/processor';

  /// Should have implemented in backend but I have no time learning auth in actix

  let whitelist = [];
  let inp = '';
  let exist = false;
  let importFile: Promise<string | ArrayBuffer>;

  // load
  if (import.meta.env.VITE_WEBAPP === 'true') {
    const parsed = getLocalConfig();
    whitelist = parsed.whitelist;
  } else {
    browser.storage.local.get({ whitelist: [] }).then((item) => {
      whitelist = item.whitelist;
    });
  }

  function save(whitelist: string[]) {
    if (import.meta.env.VITE_WEBAPP === 'true') {
      const parsed = getLocalConfig();
      parsed.whitelist = whitelist;
      localStorage.setItem('config', JSON.stringify(parsed));
    } else {
      browser.storage.local.set({
        whitelist,
      });
    }
    $whitelistSet = new Set(whitelist);
  }

  $: save(whitelist);

  function removeIndex(i: number) {
    whitelist.splice(i, 1);
    whitelist = whitelist;
  }

  function addText(str: string) {
    if (!str.trim()) return;
    if (whitelist.includes(str)) {
      exist = true;
      return;
    }

    whitelist = [...whitelist, ...str.split(/\s+/)];
    inp = '';
    exist = false;
  }

  async function readFile(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files.length) return;
    const limit = 512; // in KB
    if (target.files[0].size / 1024 > limit) {
      alert(`file is too big\nsize limit: ${limit}KB`);
      target.value = '';
      return;
    }
    importFile = readFileContent(target.files[0]);
  }

  function readFileContent(file: File): Promise<string | ArrayBuffer> {
    const reader = new FileReader();
    return new Promise((resolve, reject) => {
      reader.onload = (event) => resolve(event.target.result);
      reader.onerror = (error) => reject(error);
      reader.readAsText(file);
    });
  }

  async function importWhitelist() {
    const text = await importFile;
    whitelist = (text as string)
      .trim()
      .replace(/(^\s+|[^\S\n]+)/gm, '')
      .split('\n');
    alert(`import selesai, ${whitelist.length} kata ditambahkan`);
  }

  function exportWhitelist() {
    const blob = new Blob([whitelist.join('\n')], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = url;
    a.download = 'custom-dictionary.txt';
    a.click();
  }
</script>

<div class="whitelist">
  <h3 title="Biasanya diisi nama orang, slang, dan kata-kata yang dianggap valid">
    Kata Yang Dianggap Valid:
  </h3>
  <div class="container">
    {#each whitelist.slice(0, 300) as teks, i}
      <div class="item" class:bg-grey={i % 2 == 0} in:slide>
        <span class="text">{teks}</span>
        <!-- svelte-ignore: a11y-visible -->
        <div
          class="icon"
          on:click={(_) => removeIndex(i)}
          on:keypress={(e) => {
            if (e.key == 'Enter' || e.key == ' ') removeIndex(i);
          }}
          tabindex="0"
          role="button"
        >
          <svg width="24" height="24" xmlns="http://www.w3.org/2000/svg"
            ><path
              d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
            /></svg
          >
        </div>
      </div>
    {/each}
  </div>

  <form
    class="user-input"
    on:submit={(e) => {
      e.preventDefault();
      addText(inp);
    }}
  >
    <input type="text" bind:value={inp} />
    <button type="submit">Tambahkan kata</button>
    {#if exist}
      <span class="red">Kata sudah ada</span>
    {/if}
  </form>

  <input type="file" accept="text/plain" on:change={readFile} />
  <button
    on:click={importWhitelist}
    title="import dari file dimana setiap katanya dipisahkan dengan newline (enter)">import</button
  >
  <button on:click={exportWhitelist} title="Download list dalam file txt">export</button>
  <button on:click={(_) => (whitelist = [])}>clear</button>
</div>

<style>
  .whitelist {
    text-align: left;
    box-sizing: border-box;
    margin: 0 15px;
  }
  .container {
    overflow: scroll;
    max-height: 300px;
    min-height: 300px;
    min-width: 50%;
    border: solid 1px var(--opposite-color);
  }

  .item {
    margin: 0;
    vertical-align: center;
    max-width: 100%;
    padding: 10px;
  }

  .icon {
    float: right;
    height: 24px;
    width: 24px;
    cursor: pointer;
    border-radius: 50%;
    background-size: 40px;
  }

  .icon:hover,
  .icon:focus {
    background: radial-gradient(circle, #aaa, #ccc);
  }

  .user-input {
    padding: 10px 0;
  }
</style>

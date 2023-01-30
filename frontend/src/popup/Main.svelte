<script lang="ts">
  import Input from './Input.svelte';
  import Header from './Header.svelte';
  import Options from '../options/Main.svelte';
  import Footer from './Footer.svelte';
  import { menu } from '$lib/stores';
</script>

<main>
  <Header />
  <h2>RISC - Bahasa Indonesia Spell Ceker</h2>
  {#if $menu == 'options'}
    <Options />
  {:else}
    <Input />
  {/if}

  <Footer />
</main>

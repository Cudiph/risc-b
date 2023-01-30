<script lang="ts">
  import { menu } from '$lib/stores';

  function updateMenu(e: Event, str: string) {
    (e.target as HTMLButtonElement).blur();
    $menu = str;
  }
</script>

<div class="header">
  <div class="flex">
    <button
      class:active={$menu == 'main'}
      class="navbar-child"
      on:click={(e) => updateMenu(e, 'main')}>Spellceker</button
    >
    <button
      class:active={$menu == 'options'}
      class="navbar-child"
      on:click={(e) => updateMenu(e, 'options')}>Options</button
    >
  </div>
</div>

<style>
  .header {
    margin: 10px auto;
  }

  .active {
    border-bottom: solid 1px var(--opposite-color) !important;
  }

  .navbar-child {
    background-color: unset;
    outline: none;
    border: none;
    flex: auto;
    display: inline;
    cursor: pointer;
    border-bottom: #550000;
    font-size: 1.5em;
    font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode',
      Geneva, Verdana, sans-serif;
  }

  .navbar-child:focus {
    border: solid 1px var(--opposite-color);
  }
</style>

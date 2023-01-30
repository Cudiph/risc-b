<script lang="ts">
  let isPopup = false;

  if (import.meta.env.VITE_WEBAPP !== 'true') {
    browser.windows.getCurrent().then((win) => {
      if (win.type === 'popup') isPopup = true;
      else isPopup = false;
    });
  }

  function handleDetachClick(_: Event) {
    browser.windows.create({
      url: window.location.href,
      type: 'popup',
      width: 819,
      height: 600,
    });
  }
</script>

<div class="footer">
  {#if import.meta.env.VITE_WEBAPP !== 'true'}
    {#if !isPopup}
      <button on:click={handleDetachClick}>Detach Window</button>
    {/if}
  {/if}
</div>

<style>
  .footer {
    margin-top: 1em;
  }
</style>

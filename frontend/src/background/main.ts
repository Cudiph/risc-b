browser.runtime.onInstalled.addListener(() => {
  // set initial settings
  const init: LocalStorage = {
    endpoint: import.meta.env.VITE_DEFAULT_ENDPOINT,
    english: false,
    correction: false,
    format: 'HTML',
    tolerance: 'LOW',
    tidak_baku: false,
    whitelist: [],
  };

  browser.storage.local.set(init);
});

export {};

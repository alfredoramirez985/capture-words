<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { appSettings } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
  import { fly, fade } from "svelte/transition";
  import { error as logError, info } from "@tauri-apps/plugin-log";

  type Translation = { id: number, phrase_id: number, language_code: string, translated_text: string, created_at: string };
  type PhraseWithTranslations = { id: number, session_id: number, phrase: string, created_at: string, translations: Translation[] };

  let phrase = $state("");
  let savedPhrases: Array<PhraseWithTranslations> = $state([]);
  let typingTimer: ReturnType<typeof setTimeout> | null = null;
  let unlisten: () => void;
  
  let sourceLang = $state("ES");
  let targetLang = $state("EN-US");

  onMount(async () => {
    if (!appSettings.activeSession) {
      goto('/');
    } else {
      await loadPhrases();
      unlisten = await listen('translations-ready', () => {
        loadPhrases();
      });
    }
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  async function loadPhrases() {
    if (!appSettings.activeSession) return;
    try {
      savedPhrases = await invoke("get_phrases", { sessionId: appSettings.activeSession.id });
    } catch (err) {
      logError(`Failed to load phrases: ${err}`);
      console.error("Failed to load phrases:", err);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      capturePhrase();
    }
  }

  function handleTyping() {
    if (typingTimer) clearTimeout(typingTimer);
    if (phrase && appSettings.activeSession) {
      typingTimer = setTimeout(() => {
        capturePhrase();
      }, appSettings.captureDelaySeconds * 1000);
    }
  }

  async function capturePhrase() {
    if (!phrase || !appSettings.activeSession) return;
    try {
      await invoke("save_phrase", { 
        sessionId: appSettings.activeSession.id, 
        phrase,
        sourceLang,
        targetLang,
        apiKey: appSettings.geminiToken
      });
      info(`Successfully captured phrase`);
      await loadPhrases();
      phrase = "";
      if (typingTimer) {
        clearTimeout(typingTimer);
        typingTimer = null;
      }
    } catch (err) {
      logError(`Failed to save phrase: ${err}`);
      console.error("Failed to save phrase:", err);
    }
  }

  function goBack() {
    goto('/');
  }
</script>

<main class="container" in:fly={{ x: 50, duration: 400 }} out:fade={{ duration: 200 }}>
  {#if appSettings.activeSession}
    <nav class="navbar">
      <button class="back-btn" onclick={goBack}>&larr; Sessions</button>
      <h2 class="session-title">{appSettings.activeSession.name}</h2>
      <div style="width: 100px;"></div> <!-- Spacer -->
    </nav>

    <div class="lang-selector-row">
      <div class="lang-group">
        <label for="source-lang">From:</label>
        <select id="source-lang" bind:value={sourceLang}>
          <option value="ES">Spanish</option>
          <option value="EN">English</option>
          <option value="FR">French</option>
          <option value="IT">Italian</option>
          <option value="PT">Portuguese</option>
        </select>
      </div>
      <div class="lang-group" style="margin-left: 20px;">
        <label for="target-lang">To:</label>
        <select id="target-lang" bind:value={targetLang}>
          <option value="EN-US">English</option>
          <option value="ES">Spanish</option>
          <option value="FR">French</option>
          <option value="IT">Italian</option>
          <option value="PT-BR">Portuguese</option>
        </select>
      </div>
    </div>

    <div class="row" style="margin-top: 40px;">
      <input id="phrase-input" placeholder="Start typing a phrase..." bind:value={phrase} oninput={handleTyping} onkeydown={handleKeyDown} style="width: 400px; padding: 1em; font-size: 1.2em;" />
    </div>
    
    <div style="margin-top: 40px; text-align: left; max-width: 500px; margin-left: auto; margin-right: auto;">
      <h3 style="text-align: center;">Captured so far:</h3>
      {#if savedPhrases.length === 0}
        <p style="color: #666; text-align: center;">Waiting for input...</p>
      {:else}
        <ul style="list-style-type: none; padding: 0;">
          {#each savedPhrases as sp}
            <li class="phrase-item">
              <div style="display: flex; justify-content: space-between; align-items: baseline;">
                <strong>{sp.phrase}</strong> 
                <small style="color: #888;">{sp.created_at}</small>
              </div>
              {#if sp.translations && sp.translations.length > 0}
                <div class="translations">
                  {#each sp.translations as t}
                    <div class="translation-line">
                      <span class="lang-badge">{t.language_code}</span> {t.translated_text}
                    </div>
                  {/each}
                </div>
              {:else}
                <div class="translating-text">Translating...</div>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</main>

<style>
:root {
  font-family: Inter, Avenir, sans-serif;
  color: #0f0f0f;
  background-color: #f6f6f6;
}
.container {
  margin: 0;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  text-align: center;
  min-height: 100vh;
}
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin: 0 20px;
}
.session-title {
  margin: 0;
  flex-grow: 1;
}
.lang-selector-row {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 20px;
}
.lang-group {
  display: flex;
  align-items: center;
  gap: 8px;
}
.lang-group select {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid #ccc;
  background-color: white;
  min-width: 120px;
}
.row {
  display: flex;
  justify-content: center;
}
input, button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-family: inherit;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0,0,0,0.2);
  outline: none;
}
button { cursor: pointer; background-color: #fff; }
button:hover { border-color: #396cd8; }
.back-btn { background-color: transparent; box-shadow: none; border: 1px solid #ccc; }
.phrase-item {
  padding: 12px;
  border-bottom: 1px solid #ddd;
  background: white;
  margin-bottom: 8px;
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.05);
}
.translations {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px dashed #eee;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.translation-line {
  display: flex;
  align-items: center;
  font-size: 0.9em;
  color: #444;
}
.lang-badge {
  background-color: #e0f2fe;
  color: #0369a1;
  font-size: 0.7em;
  font-weight: bold;
  padding: 2px 6px;
  border-radius: 4px;
  margin-right: 8px;
  width: 35px;
  text-align: center;
  display: inline-block;
}
.translating-text {
  margin-top: 10px;
  font-size: 0.85em;
  color: #888;
  font-style: italic;
}
@media (prefers-color-scheme: dark) {
  :root { color: #f6f6f6; background-color: #2f2f2f; }
  .navbar, .phrase-item { background-color: #3f3f3f; border-color: #555; }
  input, button { color: #fff; background-color: #0f0f0f98; }
  .translations { border-top-color: #555; }
  .translation-line { color: #ccc; }
  .lang-badge { background-color: #0c4a6e; color: #7dd3fc; }
}
</style>

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
.container {
  margin: 0;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  min-height: 100vh;
}
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  width: 100%;
  max-width: 800px;
  margin: 0 auto 40px auto;
}
.session-title { flex-grow: 1; margin: 0; }
.lang-selector-row { display: flex; justify-content: center; gap: 20px; margin-top: 20px; }
.lang-group { display: flex; align-items: center; gap: 8px; }
.lang-group select { min-width: 120px; }
.row { display: flex; justify-content: center; }
.phrase-item {
  padding: 16px;
  border-bottom: 1px solid var(--purple-border);
  background: var(--purple-panel);
  margin-bottom: 12px;
  border-radius: 12px;
  box-shadow: 0 4px 14px rgba(0,0,0,0.2);
  backdrop-filter: blur(8px);
}
.translations {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.translation-line {
  display: flex;
  align-items: center;
  font-size: 0.95em;
  color: var(--text-bright);
}
.lang-badge {
  background-color: rgba(199, 125, 255, 0.15);
  color: var(--purple-highlight);
  font-size: 0.75em;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 4px;
  margin-right: 12px;
  width: 40px;
  text-align: center;
  display: inline-block;
}
.translating-text {
  margin-top: 10px;
  font-size: 0.9em;
  color: var(--purple-highlight);
  font-style: italic;
  animation: pulse 1.5s infinite alternate;
}
@keyframes pulse {
  from { opacity: 0.6; }
  to { opacity: 1; }
}
</style>

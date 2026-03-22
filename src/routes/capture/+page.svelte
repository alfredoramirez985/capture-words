<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { appSettings } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
  import { fly, fade } from "svelte/transition";

  let phrase = $state("");
  let savedPhrases: Array<{ id: number, session_id: number, phrase: string, created_at: string }> = $state([]);
  let typingTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    if (!appSettings.activeSession) {
      goto('/');
    } else {
      loadPhrases();
    }
  });

  async function loadPhrases() {
    if (!appSettings.activeSession) return;
    try {
      savedPhrases = await invoke("get_phrases", { sessionId: appSettings.activeSession.id });
    } catch (error) {
      console.error("Failed to load phrases:", error);
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
      await invoke("save_phrase", { sessionId: appSettings.activeSession.id, phrase });
      await loadPhrases();
      phrase = "";
      if (typingTimer) {
        clearTimeout(typingTimer);
        typingTimer = null;
      }
    } catch (error) {
      console.error("Failed to save phrase:", error);
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

    <div class="row" style="margin-top: 40px;">
      <input id="phrase-input" placeholder="Start typing a phrase..." bind:value={phrase} oninput={handleTyping} style="width: 400px; padding: 1em; font-size: 1.2em;" />
    </div>
    
    <div style="margin-top: 40px; text-align: left; max-width: 500px; margin-left: auto; margin-right: auto;">
      <h3 style="text-align: center;">Captured so far:</h3>
      {#if savedPhrases.length === 0}
        <p style="color: #666; text-align: center;">Waiting for input...</p>
      {:else}
        <ul style="list-style-type: none; padding: 0;">
          {#each savedPhrases as sp}
            <li class="phrase-item">
              <strong>{sp.phrase}</strong> 
              <br/><small style="color: #888;">{sp.created_at}</small>
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
@media (prefers-color-scheme: dark) {
  :root { color: #f6f6f6; background-color: #2f2f2f; }
  .navbar, .phrase-item { background-color: #3f3f3f; border-color: #555; }
  input, button { color: #fff; background-color: #0f0f0f98; }
}
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { appSettings, type Session } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
  import { fly, fade } from "svelte/transition";
  import { error as logError } from "@tauri-apps/plugin-log";

  let savedSessions: Session[] = $state([]);

  onMount(async () => {
    await loadSessions();
  });

  async function loadSessions() {
    try {
      savedSessions = await invoke("get_sessions");
    } catch (err) {
      logError(`Failed to load sessions: ${err}`);
      console.error("Failed to load sessions:", err);
    }
  }

  async function toggleFavorite(session: Session, event: Event) {
    event.stopPropagation();
    try {
      const newState = !session.is_favorite;
      await invoke("toggle_favorite", { sessionId: session.id, isFavorite: newState });
      await loadSessions();
    } catch (err) {
      logError(`Failed to toggle favorite: ${err}`);
      console.error("Failed to toggle favorite:", err);
    }
  }

  function selectSession(session: Session) {
    appSettings.activeSession = session;
    goto('/capture');
  }
</script>

<main class="container" in:fly={{ y: -50, duration: 400 }} out:fade={{ duration: 200 }}>
  <nav class="navbar">
    <button class="back-btn" onclick={() => goto('/')}>&larr; Back to Home</button>
    <div style="width: 100px;"></div> <!-- Spacer -->
  </nav>

  <h2>Past Sessions</h2>
  {#if savedSessions.length === 0}
    <p style="color: #666; font-style: italic;">No sessions found. Go back and create one!</p>
  {:else}
    <ul class="session-list">
      {#each savedSessions as session}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <li class="session-card" onclick={() => selectSession(session)}>
          <div class="session-info">
            <div class="session-name">
              <strong>{session.name}</strong>
              <small class="date-badge">{new Date(session.created_at + 'Z').toLocaleDateString()}</small>
            </div>
            {#if session.description}
              <p class="session-desc">{session.description}</p>
            {/if}
          </div>
          <button type="button" class="star-btn" onclick={(e) => toggleFavorite(session, e)} title={session.is_favorite ? "Remove from favorites" : "Add to favorites"}>
            {#if session.is_favorite}
              <span style="color: #fbbf24; font-size: 1.5em; line-height: 1;">★</span>
            {:else}
              <span style="color: #ccc; font-size: 1.5em; line-height: 1;">☆</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
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
  margin: 0 20px 40px 20px;
}
button {
  border-radius: 8px;
  border: 1px solid #ccc;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-family: inherit;
  transition: all 0.25s;
  outline: none;
}
button { cursor: pointer; box-shadow: 0 2px 2px rgba(0,0,0,0.1); background-color: #fff; }
button:hover { border-color: #396cd8; }
button:active { background-color: #eee; }
.back-btn { background-color: transparent; box-shadow: none; border: 1px solid #ccc; }

.session-list {
  list-style-type: none;
  padding: 0;
  max-width: 600px;
  margin: 0 auto 40px auto;
  text-align: left;
}
.session-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  margin-bottom: 10px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  border: 1px solid #eaeaea;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}
.session-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
  border-color: #d0d0d0;
}
.session-name {
  display: flex;
  align-items: baseline;
  gap: 10px;
  font-size: 1.1em;
}
.date-badge {
  color: #888;
  font-size: 0.75em;
  background-color: #f0f0f0;
  padding: 2px 6px;
  border-radius: 4px;
}
.session-desc {
  margin: 5px 0 0 0;
  font-size: 0.9em;
  color: #666;
}
.star-btn {
  background: none;
  border: none;
  box-shadow: none;
  padding: 5px;
  border-radius: 50%;
}
.star-btn:hover {
  background-color: rgba(0,0,0,0.05);
  border-color: transparent;
}
@media (prefers-color-scheme: dark) {
  :root { color: #f6f6f6; background-color: #2f2f2f; }
  .navbar { background-color: #3f3f3f; border-color: #555; }
  button { color: #fff; background-color: #0f0f0f98; border-color: #555; }
  .session-card { background-color: #3f3f3f; border-color: #555; }
  .session-card:hover { border-color: #777; }
  .date-badge { background-color: #555; color: #ccc; }
  .session-desc { color: #aaa; }
  .star-btn:hover { background-color: rgba(255,255,255,0.1); }
}
</style>

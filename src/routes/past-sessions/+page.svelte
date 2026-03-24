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
.session-list {
  list-style-type: none;
  padding: 0;
  width: 100%;
  max-width: 600px;
  margin: 0 auto 40px auto;
  text-align: left;
}
.session-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  margin-bottom: 12px;
  cursor: pointer;
}
.session-name {
  display: flex;
  align-items: baseline;
  gap: 12px;
  font-size: 1.1em;
}
.date-badge {
  color: var(--purple-highlight);
  font-size: 0.75em;
  background-color: rgba(199, 125, 255, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
}
.session-desc {
  margin: 5px 0 0 0;
  font-size: 0.9em;
  color: rgba(255, 255, 255, 0.6);
}
</style>

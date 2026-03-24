<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { appSettings, type Session } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
  import { fly, fade } from "svelte/transition";
  import { error as logError, info } from "@tauri-apps/plugin-log";

  let newSessionName = $state("");
  let newSessionDesc = $state("");

  async function createNewSession(event: Event) {
    event.preventDefault();
    if (!newSessionName) return;
    try {
      const newId: number = await invoke("create_session", { 
        name: newSessionName, 
        description: newSessionDesc || null,
        init_page: null,
        finish_page: null
      });
      
      const newSession: Session = {
        id: newId,
        name: newSessionName,
        description: newSessionDesc || null,
        created_at: new Date().toISOString()
      };

      appSettings.activeSession = newSession;
      newSessionName = "";
      newSessionDesc = "";
      
      info(`Successfully created session ${newId}: ${newSession.name}`);
      goto('/capture');
    } catch (err) {
      logError(`Failed to create session: ${err}`);
      console.error("Failed to create session:", err);
    }
  }


</script>

<main class="container" in:fly={{ y: -50, duration: 400 }} out:fade={{ duration: 200 }}>
  <div style="position: absolute; top: 20px; right: 20px;">
    <button type="button" onclick={() => goto('/settings')}>Settings</button>
  </div>
  
  <h1>Capture Words</h1>
  <p>Create a session to start capturing phrases.</p>
  
  <hr style="margin: 30px auto; width: 50%; border-color: #ddd;" />

  <h2>Create New Session</h2>
  <form class="row" onsubmit={createNewSession} style="margin-bottom: 40px; margin-top: 20px;">
    <input placeholder="Session Name..." bind:value={newSessionName} required />
    <input placeholder="Description (Optional)..." bind:value={newSessionDesc} />
    <button type="submit">Create & Enter</button>
  </form>



  <hr style="margin: 40px auto; width: 60%; border-color: #ddd;" />

  <h2>Review Past Sessions</h2>
  <div class="row" style="margin-top: 20px;">
    <button type="button" onclick={() => goto('/past-sessions')}>
      View Past Sessions
    </button>
  </div>
</main>

<style>
.container {
  margin: 0;
  padding-top: 15vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
.row {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 15px;
}
</style>

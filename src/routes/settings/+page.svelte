<script lang="ts">
  import { appSettings } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
</script>

<main class="container">
  <h2>Settings</h2>
  
  <div class="row" style="margin-bottom: 20px; justify-content: center; display: flex; gap: 15px;">
    <label for="delay-input" style="align-self: center;">Auto-capture delay (sec):</label>
    <input id="delay-input" type="number" min="1" max="120" bind:value={appSettings.captureDelaySeconds} style="width: 80px; text-align: center;" />
  </div>

  <div class="row" style="margin-bottom: 30px; justify-content: center; display: flex; gap: 15px;">
    <label for="gemini-token-input" style="align-self: center;">Gemini API Token:</label>
    <input id="gemini-token-input" type="password" placeholder="Paste token here" bind:value={appSettings.geminiToken} style="width: 320px; text-align: center;" />
  </div>

  <div class="row" style="margin-top: 20px; justify-content: center; display: flex;">
    <button type="button" onclick={() => goto('/')}>Back to Capture</button>
  </div>
</main>

<style>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
h2 {
  text-align: center;
}
</style>

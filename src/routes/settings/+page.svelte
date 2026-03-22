<script lang="ts">
  import { appSettings } from "$lib/settings.svelte";
  import { goto } from "$app/navigation";
</script>

<main class="container">
  <h2>Settings</h2>
  
  <div class="row" style="margin-bottom: 20px; justify-content: center; display: flex;">
    <label for="delay-input" style="margin-right: 10px; align-self: center;">Auto-capture delay (sec):</label>
    <input id="delay-input" type="number" min="1" max="120" bind:value={appSettings.captureDelaySeconds} style="padding: 0.5em; border-radius: 8px; width: 60px; text-align: center;" />
  </div>

  <div class="row" style="margin-top: 20px; justify-content: center; display: flex;">
    <button type="button" onclick={() => goto('/')}>Back to Capture</button>
  </div>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;
}

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

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  outline: none;
}

button:hover {
  border-color: #396cd8;
  cursor: pointer;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  input, button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
}
</style>

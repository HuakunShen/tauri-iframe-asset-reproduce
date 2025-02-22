<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const htmlUrl = convertFileSrc("/", "fileserver");

  onMount(() => {
    fetch(htmlUrl)
      .then((res) => res.text())
      .then((data) => console.log(data));
  });
</script>

<main class="container">
  <h1>iframe</h1>
  <iframe
    src={htmlUrl}
    sandbox="allow-same-origin allow-scripts"
    title="fileserver"
  ></iframe>
</main>

<style>
  iframe {
    border: 1px solid red;
  }
</style>

<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const imgUrl = convertFileSrc("/", "img");
  const htmlUrl = convertFileSrc("/", "fileserver");

  onMount(() => {
    fetch(htmlUrl)
      .then((res) => res.text())
      .then((data) => console.log(data));
  });
</script>

<main class="container">
  <img src={imgUrl} alt="" width="200" height="200" />
  <h1>iframe</h1>
  <iframe
    src={htmlUrl}
    sandbox="allow-same-origin allow-scripts"
    title="fileserver"
  ></iframe>
</main>

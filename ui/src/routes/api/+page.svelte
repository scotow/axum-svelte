<script lang="ts">
import { onMount } from "svelte";

let content: string = "";

onMount(async () => {
  fetch("/my-file.txt")
  .then(response => response.text())
  .then(data => {
    content = data;
  });
});
</script>

<h1>{content}</h1>
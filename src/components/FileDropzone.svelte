<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import FileDrop from "svelte-tauri-filedrop";

  import { getFilename, getName } from "../utils/file-util";

  export let extensions: string[];
  export let onSelected: any;

  let name = "Choose a file";

  let file = {
    url: "",
    path: "",
    name: "",
    outputPath: "",
  };

  async function chooseFile() {
    try {
      // Open a selection dialog for image files
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: name,
            extensions: extensions,
          },
        ],
      });

      if (Array.isArray(selected)) {
        // user selected multiple files
        console.log("user selected multiple files", selected);
      } else if (selected === null) {
        // user cancelled the selection
        console.log("user cancelled the selection");
      } else {
        // user selected a single file
        console.log("user selected a single file", selected);
        let filename = getFilename(selected);
        file = {
          url: convertFileSrc(selected),
          path: selected,
          name: getName(selected),
          outputPath: selected.replaceAll(`/${filename}`, ""),
        };
        onSelected(file);
      }
    } catch (e) {
      console.error(e);
    }
  }
</script>

<button
  on:click={chooseFile}
  class="flex flex-col justify-center items-center w-full h-64 bg-gray-50 rounded-lg border-2 border-gray-300 border-dashed cursor-pointer dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
>
  <div class="flex flex-col items-center">
    <svg
      aria-hidden="true"
      class="mb-3 w-10 h-10 text-gray-400"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      ><path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
      ></path></svg
    >
    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
      <span class="font-semibold">Click to upload</span>
    </p>
    <p class="text-xs text-gray-500 dark:text-gray-400">
      {extensions.join(", ").toUpperCase()}
    </p>
    {#if file.name}
      <p>{file.name}</p>
    {/if}
  </div>
</button>

<style scoped>
</style>

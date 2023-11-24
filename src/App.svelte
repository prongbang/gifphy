<script lang="ts">
  import { Navbar, NavBrand, GradientButton, Spinner } from "flowbite-svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import FileDropzone from "./components/FileDropzone.svelte";
  import { videoExtensions } from "./utils/file-util";
  import {
    DONE_STATUS,
    ERROR_STATUS,
    IN_PROGRESS_STATUS,
  } from "./utils/status";

  let fileSelected = {
    url: "",
    path: "",
    name: "",
    outputPath: "",
  };

  const onSelected = (file: any) => {
    console.log("file:", file);
    fileSelected = file;
  };

  let options = {
    outputPath: "",
  };

  let message = "";
  let processing = false;

  interface Payload {
    status: string;
    progress: number;
    logs: string;
    error: string;
  }

  const onDone = (path: string) => {
    message = "Success";
    console.log(path);
  };

  const onError = (error: string) => {
    console.log(error);
    message = error;
  };

  const unlisten = listen<Payload>("converter-event", (event) => {
    let payload = event.payload;
    processing = true;
    if (payload.status == DONE_STATUS) {
      processing = false;
      setTimeout(() => {
        let outputFile = `${fileSelected.outputPath}/${fileSelected.name}.gif`;
        onDone(convertFileSrc(outputFile));
      }, 250);
    } else if (payload.status == ERROR_STATUS) {
      processing = false;
      onError(message);
    }
  });

  async function converter() {
    if (!fileSelected.name || processing) return;
    
    let options = {
      inputFile: fileSelected.path,
      outputPath: fileSelected.outputPath,
    };
    await invoke("converter", options)
      .then(() => {
        console.log("processing...");
      })
      .catch((e) => {
        console.log(e);
      });
  }
</script>

<main class="w-screen h-screen flex flex-col">
  <Navbar>
    <NavBrand href="/">
      <img src="/tauri.svg" class="h-6 sm:h-9 mr-2" alt="Gifphy" />
      <span
        class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
      >
        Gifphy
      </span>
    </NavBrand>
  </Navbar>
  <div class="flex flex-grow items-center justify-center">
    <div class="w-full container">
      <FileDropzone {onSelected} extensions={videoExtensions} />

      <div class="mt-6 flex items-center justify-center">
        <GradientButton on:click={converter} size="lg" color="tealToLime">
          {#if processing}
            <Spinner class="mr-3" size="4" color="primary" />
          {/if}
          Convert
        </GradientButton>
      </div>
    </div>
  </div>
</main>

<style>
</style>

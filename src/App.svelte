<script lang="ts">
  import { Navbar, NavBrand, GradientButton, Spinner } from "flowbite-svelte";
  import FileDropzone from "./components/FileDropzone.svelte";
    import { videoExtensions } from "./utils/file-util";

  const onSelected = (file: any) => {
    console.log("file:", file)
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
      <FileDropzone onSelected={onSelected} extensions={videoExtensions}/>

      <div class="mt-6 flex items-center justify-center">
        <GradientButton size="lg" color="tealToLime">
          {#if (false)}
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

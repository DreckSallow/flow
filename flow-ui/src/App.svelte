<script lang="ts">
  import { onMount } from "svelte";
  import SystemUi from "./system-ui.svelte";
  import Switch from "./components/switch.svelte";
  import { SunIcon, MoonIcon } from "./components/icons";
</script>

<SystemUi>
  <nav
    class="flex flex-row items-center justify-between py-2 px-3 bg-soft rounded-lg"
  >
    <div class="nav-logo">
      <img src="/images/flow-logo.png" alt="Flow Logo" />
    </div>
    <ul class="flex flex-row items-center">
      <Switch checked let:checked>
        {#if checked}
          <MoonIcon class="fill-zinc-600 w-10/12" />
        {:else}
          <SunIcon class="fill-zinc-600 w-10/12" />
        {/if}
      </Switch>
    </ul>
  </nav>
</SystemUi>

<style>
  .nav-logo {
    width: 70px;
  }

  nav {
    border: 1px solid white;
    min-height: 70px;
    width: 100%;
  }
</style>

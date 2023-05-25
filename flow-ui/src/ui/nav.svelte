<script>
  import { MoonIcon, SunIcon } from "../components/icons";
  import Switch from "../components/switch.svelte";
  import { themeStore, updateTheme } from "../lib/store/theme";
</script>

<nav
  class="flex flex-row items-center justify-between py-2 px-3 bg-soft rounded-lg"
>
  <div class="nav-logo">
    <img src="/images/flow-logo.png" alt="Flow Logo" />
  </div>
  <ul class="flex flex-row items-center">
    <Switch
      checked={$themeStore == "dark"}
      let:checked
      click={(s) => {
        updateTheme($themeStore == "light" ? "dark" : "light");
        return !s;
      }}
    >
      {#if checked}
        <MoonIcon class="fill-strong w-10/12" />
      {:else}
        <SunIcon class="fill-strong w-10/12" />
      {/if}
    </Switch>
  </ul>
</nav>

<style>
  .nav-logo {
    width: 70px;
  }

  nav {
    min-height: 70px;
    width: 100%;
  }
</style>

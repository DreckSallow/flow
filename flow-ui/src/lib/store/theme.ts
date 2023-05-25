import { writable } from "svelte/store";

export const themeKeyStore = "flow-theme";

export type ThemeType = "light" | "dark";

export const themeStore = writable<ThemeType>("light");

export const getTheme = () => {
  const theme = localStorage.getItem(themeKeyStore);
  if (theme && (theme as ThemeType) === "dark") {
    themeStore.set(theme as ThemeType);
    document.body.classList.remove("light-theme");
    document.body.classList.add("dark-theme");
  }
};

export const updateTheme = (th: ThemeType) => {
  document.body.classList.remove(th === "dark" ? "light-theme" : "dark-theme");
  document.body.classList.add(th === "dark" ? "dark-theme" : "light-theme");

  localStorage.setItem(themeKeyStore, th);
  themeStore.set(th);
};

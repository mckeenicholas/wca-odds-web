<script setup lang="ts">
import ColorModeSwitcher from "@/components/custom/ColorModeSwitcher.vue";
import { Github } from "lucide-vue-next";
import { useRoute } from "vue-router";
import "./assets/index.css";
import BackButton from "./components/custom/BackButton.vue";

const route = useRoute();

const versionNum = "0.6.4";
</script>

<template>
  <BackButton v-if="route.path !== '/'" />
  <div class="flex min-h-screen flex-col">
    <main class="flex-grow">
      <ColorModeSwitcher />
      <RouterView class="-mt-4" />
    </main>
    <footer
      class="flex h-10 items-center justify-end px-4 text-sm font-semibold text-muted-foreground"
    >
      <span class="mr-3"
        >Made by
        <a class="underline hover:text-gray-300" href="https://nmckee.org"
          >Nicholas McKee</a
        >
      </span>
      <a
        href="https://github.com/mckeenicholas/wca-odds-web"
        class="mr-3 hover:text-gray-400"
        aria-label="GitHub Repository"
      >
        <Github class="h-4 w-4" />
      </a>
      <span>v{{ versionNum }}</span>
    </footer>
  </div>
</template>

<style lang="css">
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-thumb {
  background: hsl(var(--muted-foreground));
  border-radius: 4px;
}

body {
  overflow-y: scroll;
}
</style>

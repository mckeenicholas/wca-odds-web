<script setup lang="ts">
import "./assets/index.css";
import { Github } from "lucide-vue-next";
import ColorModeSwitcher from "@/components/custom/ColorModeSwitcher.vue";
import BackButton from "./components/custom/BackButton.vue";
import { useRoute } from "vue-router";

const route = useRoute();

const versionNum = "0.6.4";
</script>

<template>
  <BackButton v-if="route.path !== '/'" />
  <div class="flex flex-col min-h-screen">
    <main class="flex-grow">
      <ColorModeSwitcher />
      <RouterView class="-mt-4" />
    </main>
    <footer
      class="h-10 px-4 text-sm text-muted-foreground flex items-center justify-end font-semibold"
    >
      <span class="mr-3"
        >Made by
        <a class="underline hover:text-gray-300" href="https://nmckee.org"
          >Nicholas McKee</a
        >
      </span>
      <a
        href="https://github.com/mckeenicholas/wca-odds-web"
        class="hover:text-gray-400 mr-3"
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

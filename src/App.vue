<script setup lang="ts">
import "./assets/index.css";
import { useColorMode } from "@vueuse/core";
import { Button } from "./components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./components/ui/dropdown-menu";
import { Moon, Sun, Github } from "lucide-vue-next";

const versionNum = "0.3.2";

const mode = useColorMode();
</script>

<template>
  <div class="flex flex-col min-h-screen">
    <main class="flex-grow">
      <div class="absolute top-2 right-2">
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button variant="outline" class="px-2.5">
              <Moon
                class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
              />
              <Sun
                class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
              />
              <span class="sr-only">Toggle theme</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem @click="mode = 'light'"> Light </DropdownMenuItem>
            <DropdownMenuItem @click="mode = 'dark'"> Dark </DropdownMenuItem>
            <DropdownMenuItem @click="mode = 'auto'"> System </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <RouterView />
    </main>
    <footer
      class="h-12 py-2 px-4 text-sm text-muted-foreground flex items-center justify-end font-semibold"
    >
      <span class="mr-3"
        >Made by
        <a class="underline hover:text-gray-400" href="https://nmckee.org"
          >Nicholas McKee</a
        >
      </span>
      <a
        href="https://github.com/mckeenicholas/wca-odds-web"
        class="hover:text-gray-400 mr-3"
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

html {
  scrollbar-gutter: stable;
}
</style>

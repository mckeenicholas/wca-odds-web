<script setup lang="ts">
import "./assets/index.css";
import { useColorMode } from "@vueuse/core";
import { Icon } from "@iconify/vue";
import { Button } from "./components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./components/ui/dropdown-menu";

const mode = useColorMode();
</script>

<template>
  <main>
    <div class="absolute top-2 right-2">
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="outline">
            <Icon
              icon="radix-icons:moon"
              class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
            />
            <Icon
              icon="radix-icons:sun"
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

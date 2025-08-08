<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useColorMode } from "@vueuse/core";
import { Moon, Sun } from "lucide-vue-next";

const mode = useColorMode();
</script>

<template>
  <div class="flex w-full flex-row justify-end">
    <DropdownMenu>
      <DropdownMenuTrigger as-child class="m-2">
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
</template>

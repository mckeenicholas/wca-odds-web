<script setup lang="ts">
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ref } from "vue";
import Chevron from "./RotatableChevron.vue";

const { title = "" } = defineProps<{ title?: string }>();

const open = ref<boolean>(false);
</script>

<template>
  <div class="rounded-md border">
    <Collapsible v-model:open="open">
      <CollapsibleTrigger as-child>
        <div
          class="flex cursor-pointer flex-row rounded-sm py-2 pe-3 ps-4 hover:bg-secondary"
        >
          <div class="flex-grow">
            {{ title }}
          </div>
          <div class="flex flex-col place-content-center">
            <Chevron :up="open" />
          </div>
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <slot></slot>
      </CollapsibleContent>
    </Collapsible>
  </div>
</template>

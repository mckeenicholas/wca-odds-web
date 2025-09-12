<script setup lang="ts">
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ref, useId } from "vue";
import Chevron from "./RotatableChevron.vue";

const { title = "", id = useId() } = defineProps<{
  title?: string;
  id?: string;
}>();

const open = ref<boolean>(false);
</script>

<template>
  <div class="rounded-md border">
    <Collapsible v-model:open="open">
      <CollapsibleTrigger as-child :aria-controls="id">
        <button
          type="button"
          class="flex w-full cursor-pointer flex-row rounded-sm border-none bg-transparent py-2 pe-3 ps-4 text-left text-inherit hover:bg-secondary"
        >
          <div class="flex-grow">
            {{ title }}
          </div>
          <div class="flex flex-col place-content-center">
            <Chevron :up="open" />
          </div>
        </button>
      </CollapsibleTrigger>
      <CollapsibleContent :id="id">
        <slot></slot>
      </CollapsibleContent>
    </Collapsible>
  </div>
</template>

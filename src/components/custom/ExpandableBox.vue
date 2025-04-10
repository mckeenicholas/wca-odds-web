<script setup lang="ts">
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ref } from "vue";
import { ChevronUp } from "lucide-vue-next";

const { title = "" } = defineProps<{ title?: string }>();

const open = ref<boolean>(false);
</script>

<template>
  <div class="border rounded-md">
    <Collapsible v-model:open="open">
      <CollapsibleTrigger as-child>
        <div
          class="py-2 ps-4 pe-3 hover:bg-secondary cursor-pointer flex flex-row"
        >
          <div class="flex-grow">
            {{ title }}
          </div>
          <div class="flex flex-col place-content-center">
            <ChevronUp
              class="scale-75 transition-transform duration-450"
              :class="{ '-rotate-180': open }"
            />
          </div>
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <slot></slot>
      </CollapsibleContent>
    </Collapsible>
  </div>
</template>

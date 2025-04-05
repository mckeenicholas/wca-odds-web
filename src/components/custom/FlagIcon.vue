<script setup lang="ts">
import "flag-icons/css/flag-icons.min.css";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { computed } from "vue";

const props = defineProps<{
  code: string;
  muted?: boolean;
}>();

const countryName = computed(() => {
  try {
    const regionNames = new Intl.DisplayNames(["en"], { type: "region" });
    return regionNames.of(props.code) || props.code;
  } catch {
    return props.code;
  }
});
</script>

<template>
  <TooltipProvider :delayDuration="250">
    <Tooltip>
      <TooltipTrigger>
        <span :class="[`shadow-md fi fi-${code.toLowerCase()}`, { 'opacity-50': muted }]">
        </span>
      </TooltipTrigger>
      <TooltipContent>
        {{ countryName }}
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>

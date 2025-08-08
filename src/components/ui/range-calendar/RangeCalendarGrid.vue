<script lang="ts" setup>
import { cn } from "@/lib/utils";
import { reactiveOmit } from "@vueuse/core";
import {
  RangeCalendarGrid,
  type RangeCalendarGridProps,
  useForwardProps,
} from "reka-ui";
import type { HTMLAttributes } from "vue";

const props = defineProps<
  RangeCalendarGridProps & { class?: HTMLAttributes["class"] }
>();

const delegatedProps = reactiveOmit(props, "class");

const forwardedProps = useForwardProps(delegatedProps);
</script>

<template>
  <RangeCalendarGrid
    :class="cn('w-full border-collapse space-y-1', props.class)"
    v-bind="forwardedProps"
  >
    <slot />
  </RangeCalendarGrid>
</template>

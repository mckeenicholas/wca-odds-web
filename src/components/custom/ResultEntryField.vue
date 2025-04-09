<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { SupportedWCAEvent } from "@/lib/types";
import { toClockFormat } from "@/lib/utils";
import { onMounted, ref, watch } from "vue";

const { event } = defineProps<{
  event: SupportedWCAEvent;
}>();
const model = defineModel<number>({ required: true });
const inputValue = ref<string>("");

const isFMC = event === "333fm";

const formatInput = (input: string): string => {
  if (isFMC) {
    return input.replace(/\D/g, "");
  }

  const number = parseInt(input.replace(/\D/g, ""));
  if (isNaN(number) || number === 0) return "";

  const str = number.toString().padStart(8, "0").slice(-8);
  const [hh, mm, ss, cc] = [
    str.slice(0, 2),
    str.slice(2, 4),
    str.slice(4, 6),
    str.slice(6, 8),
  ];
  return `${hh}:${mm}:${ss}.${cc}`.replace(/^[0:]*(?!\.)/g, "");
};

const toCentiseconds = (input: string): number => {
  if (input === "") return 0;
  if (input.toLowerCase() === "dnf") return -1;

  if (isFMC) {
    const moves = parseInt(input.replace(/\D/g, "")) || 0;
    return moves * 100;
  }

  const digits = input.replace(/\D/g, "");
  if (!digits) return 0;

  const num = parseInt(digits);
  const hh = Math.floor(num / 1000000) * 360000;
  const mm = Math.floor((num % 1000000) / 10000) * 6000;
  const ss = Math.floor((num % 10000) / 100) * 100;
  const cc = num % 100;

  return hh + mm + ss + cc;
};

const updateInputFromModel = (value: number) => {
  if (value === -1) {
    inputValue.value = "DNF";
  } else if (value === 0) {
    inputValue.value = "";
  } else if (isFMC) {
    inputValue.value = Math.floor(value / 100).toString();
  } else {
    const formatted = toClockFormat(value);
    inputValue.value = formatted;
  }
};

onMounted(() => updateInputFromModel(model.value));

watch(() => model.value, updateInputFromModel, { flush: "post" });

watch(inputValue, (value) => {
  if (value === "DNF") {
    model.value = -1;
  } else {
    const formattedInput = formatInput(value);
    inputValue.value = formattedInput;
    model.value = toCentiseconds(formattedInput);
  }
});

const handleKeydown = (event: KeyboardEvent) => {
  if (["d", "D"].includes(event.key)) {
    event.preventDefault();
    inputValue.value = "DNF";
    model.value = -1;
  }
};
</script>

<template>
  <Input
    @keydown="handleKeydown"
    v-model="inputValue"
    class="min-w-[50vw] lg:min-w-0"
  />
</template>

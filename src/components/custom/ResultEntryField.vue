<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { toClockFormat } from "@/lib/utils";
import { onMounted, ref, watch } from "vue";

const model = defineModel<number>({ required: true });
const inputValue = ref<string>("");

onMounted(() => {
  updateInputFromModel(model.value);
});

watch(
  () => model.value,
  (newValue) => {
    updateInputFromModel(newValue);
  },
  { flush: "post" },
);

watch(inputValue, (value) => {
  if (value !== "DNF") {
    inputValue.value = reformatInput(value);
    model.value = toCentiseconds(inputValue.value);
  } else if (value === "DNF" && model.value !== -1) {
    model.value = -1;
  }
});

const updateInputFromModel = (value: number) => {
  if (value === -1) {
    if (inputValue.value !== "DNF") {
      inputValue.value = "DNF";
    }
  } else {
    const formatted = toClockFormat(value);
    if (formatted !== inputValue.value) {
      inputValue.value = formatted;
    }
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  if (["d", "D"].includes(event.key)) {
    event.preventDefault();
    inputValue.value = "DNF";
    model.value = -1;
  }
};

const toInt = (input: string): number | null => {
  const int = parseInt(input);
  return isNaN(int) ? null : int;
};

const reformatInput = (input: string): string => {
  const number = toInt(input.replace(/\D/g, "")) || 0;
  if (number === 0) return "";
  const str = "00000000" + number.toString().slice(0, 8);
  const match = str.match(/(\d\d)(\d\d)(\d\d)(\d\d)$/);
  if (!match) return "";
  const [, hh, mm, ss, cc] = match;
  return `${hh}:${mm}:${ss}.${cc}`.replace(/^[0:]*(?!\.)/g, "");
};

const toCentiseconds = (input: string): number => {
  if (input === "") return 0;
  if (input.toLowerCase() === "dnf") return -1;
  const num = toInt(input.replace(/\D/g, "")) || 0;
  return (
    Math.floor(num / 1000000) * 360000 +
    Math.floor((num % 1000000) / 10000) * 6000 +
    Math.floor((num % 10000) / 100) * 100 +
    (num % 100)
  );
};
</script>

<template>
  <Input @keydown="handleKeydown" v-model="inputValue" />
</template>

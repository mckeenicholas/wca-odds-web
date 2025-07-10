<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { Input } from "@/components/ui/input";

const model = defineModel<number>({ required: true });
const inputValue = ref("");

const formatInput = (input: string): string => {
  const number = parseInt(input.replace(/\D/g, ""));
  if (isNaN(number) || number === 0) return "";

  return number.toString();
};

const toValue = (input: string): number => {
  if (input === "") return 0;
  if (input.toLowerCase() === "dnf") return -1;

  const digits = input.replace(/\D/g, "");
  if (!digits) return 0;

  return parseInt(digits);
};

const updateInputFromModel = (value: number) => {
  if (value === -1) {
    inputValue.value = "DNF";
  } else if (value === 0) {
    inputValue.value = "";
  } else {
    const formatted = value.toString();
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
    model.value = toValue(formattedInput);
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
    class="min-w-[50vw] lg:min-w-0"
    v-model="inputValue"
    @keydown="handleKeydown"
  />
</template>

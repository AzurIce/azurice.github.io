<script lang="ts" setup>
import { useSiteConfig } from 'valaxy'

const siteConfig = useSiteConfig()
</script>

<template>
  <div class="header pt-6 pb-8 space-y-2 md:space-y-5">
    <div class="flex flex-col items-center gap-2">
        <h1 class="
          st-text
          text-2xl
          leading-9
          font-extrabold
          tracking-tight
          sm:text-3xl sm:leading-10
          md:text-5xl md:leading-14">
          {{ siteConfig.title }}
        </h1>
        <span class="text-lg leading-7 text-gray-400">{{ siteConfig.subtitle }}</span>
    </div>
  </div>
</template>

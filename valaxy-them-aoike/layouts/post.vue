<script setup lang="ts">
import { useFrontmatter } from "valaxy";

const frontmatter = useFrontmatter();
</script>


<template>
  <Layout>
    <template #main-header-after>
      <YunPostMeta :frontmatter="frontmatter" />
      <!-- <YunWalineMeta />
      <YunPostCategoriesAndTags :frontmatter="frontmatter" /> -->
    </template>
    <template #main-content>
      <AoikeArticle />
    </template>
  </Layout>
</template>

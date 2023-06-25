<script setup lang="ts">

</script>

<template>
    <Layout>
        <div class="main">
            <div class="tag-cloud" style="display: flex; flex-wrap: wrap; gap: 10px; justify-content: center">
                {#{% for tag, post in tags %}
                    {% set factor = 1 + [tags[tag] | length] | min * 0.05 %}
                    {% set color = [160 - (factor-1) * 100, 0] | max %}
                    <div style="font-size: {{ factor }}rem; color: rgb({{ color }}, {{ color }}, {{ color }})">
                        tag 
                        <sup> tags[tag] | length </sup>
                    </div>
                {% endfor %}#}
            </div>
            <div style="display: flex; flex-wrap: wrap; justify-content: space-between; row-gap: 10px;">
                {% for category, posts in categories %}
                    macros::category_card(category=category, posts=posts) 
                {% endfor %}
            </div>
        </div>
    </Layout>
</template>
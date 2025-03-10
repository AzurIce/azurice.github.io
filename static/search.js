document.addEventListener('DOMContentLoaded', function() {
  // 获取搜索表单和输入框
  const searchForm = document.querySelector('form[name="search"]');
  const searchInput = searchForm.querySelector('input[name="query"]');
  
  // 创建搜索结果容器
  const searchResultsContainer = document.querySelector('div[id="search-results"]');
  searchResultsContainer.style.display = 'none';
  
  // 初始化Fuse.js
  const fuseOptions = {
    includeScore: true,
    keys: [
      { name: 'title', weight: 0.7 },
      { name: 'body', weight: 0.5 },
      { name: 'description', weight: 0.3 }
    ]
  };
  
  // 确保searchIndex已加载
  if (window.searchIndex) {
    const fuse = new Fuse(window.searchIndex, fuseOptions);
    
    // 处理搜索表单提交
    searchForm.addEventListener('submit', function(e) {
      e.preventDefault();
      const query = searchInput.value.trim();
      
      if (query) {
        const results = fuse.search(query);
        displayResults(results);
      } else {
        hideResults();
      }
    });
    
    // 实时搜索（输入时搜索）
    searchInput.addEventListener('input', function() {
      const query = searchInput.value.trim();
      
      if (query && query.length > 1) {
        const results = fuse.search(query);
        displayResults(results);
      } else {
        hideResults();
      }
    });
    
    // 点击页面其他地方时隐藏搜索结果
    document.addEventListener('click', function(e) {
      if (!searchForm.contains(e.target) && !searchResultsContainer.contains(e.target)) {
        hideResults();
      }
    });
  }
  
  // 显示搜索结果
  function displayResults(results) {
    searchResultsContainer.innerHTML = '';
    
    if (results.length === 0) {
      searchResultsContainer.innerHTML = '<div class="p-2 text-center text-gray-500">没有找到相关结果</div>';
      searchResultsContainer.style.display = 'block';
      return;
    }
    
    const resultsList = document.createElement('ul');
    resultsList.className = 'divide-y';
    
    results.slice(0, 10).forEach(function(result) {
      const item = result.item;
      const li = document.createElement('li');
      li.className = 'p-2 hover:bg-gray-100 transition-colors';
      
      const link = document.createElement('a');
      link.href = item.url;
      link.className = 'block';
      
      const title = document.createElement('div');
      title.className = 'font-medium';
      title.textContent = item.title || '无标题';
      
      const snippet = document.createElement('div');
      snippet.className = 'text-sm text-gray-600 truncate';
      snippet.textContent = item.body ? item.body.substring(0, 100) + '...' : '';
      
      link.appendChild(title);
      link.appendChild(snippet);
      li.appendChild(link);
      resultsList.appendChild(li);
    });
    
    searchResultsContainer.appendChild(resultsList);
    searchResultsContainer.style.display = 'block';
  }
  
  // 隐藏搜索结果
  function hideResults() {
    searchResultsContainer.style.display = 'none';
  }
});
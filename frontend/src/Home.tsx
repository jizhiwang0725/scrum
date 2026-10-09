// 路由
import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { ResultsPage } from "./ResultsPage";

// UI组件
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from '@/components/ui/input';
import { Search } from "lucide-react";

// 动态组件

// Main page
function Home() {

  // 数组结构赋值语法，第二个元素是状态更新函数（State Setter Function/Updater Function）
  const [query, setQuery] = useState(""); // 用于组件中声明状态（State）

  const navigate = useNavigate() // Navigate函数

  // 常量函数，箭头函数（Arrow Function）通常用于定义回调函数（Callback Function）或事件处理器
  // 这是一个用于处理表单提交的事件处理器（Event Handler）
  const handleSearch = (e: React.SyntheticEvent<HTMLFormElement>) => { //尖括号为泛指
    e.preventDefault(); // 阻止默认行为
    if (!query.trim()) return; // 卫语句（Guard Clause）和输入验证（Input Validation）
    
    navigate(`/results?keyword=${encodeURIComponent(query)}`); //encodeURLComponent 防止输入特殊字符导致URL格式错误
  };

  return (

    // 标题
    <div className="font-heading min-h-screen bg-background text-foreground flex flex-col items-center justify-center space-y-8">
      <h1 className="text-9xl">
        Recipe Finder
      </h1>
      
      {/* 搜索栏 */}
      <form onSubmit={handleSearch} className="w-full max-w-lg relative flex items-center">

        {/* 搜索图标 */}
        <Search className="absolute left-3.5 h-5 w-5 text-muted-foreground pointer-events-none" />
        {/* pointer-event-none - 穿透点击 */}
        
        {/* 输入 */}
        <Input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search for recipes..."
          className="h-12 pl-11 pr-24 text-base shadow-sm focus-visible:ring-primary/40"
        /> {/* pl-11 pr-24: 站位避让，给搜索图标和搜索按钮留出空间 */}

        {/* 搜索按键 */}
        <Button 
          type="submit" 
          size="sm" 
          className="absolute right-1.5 h-9 px-4"
        >
          Search
        </Button>

      </form>

    </div>
  )
}

// Routing
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
      
        <Route path="/" element={<Home />} />

        {/* 当浏览器网址编程“/results”时，渲染搜索结果页 */}
        <Route path="/results" element={<ResultsPage />} />     

      </Routes>
    </BrowserRouter>
  )
}
